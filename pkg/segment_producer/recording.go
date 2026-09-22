package segment_producer

import (
	"context"
	"fmt"
	"log"
	"path/filepath"
	"sync"
	"time"

	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/camry/pkg/helpers"
	djangolang_helpers "github.com/initialed85/djangolang/pkg/helpers"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

type recording struct {
	ctx                  context.Context
	db                   *pgxpool.Pool
	camera               *api.Camera
	cameraMu             *sync.Mutex
	destinationPath      string
	claimRefreshDuration time.Duration
	isLowRes             bool

	mu    sync.Mutex
	video *api.Video
}

func newRecording(
	ctx context.Context,
	db *pgxpool.Pool,
	camera *api.Camera,
	cameraMu *sync.Mutex,
	destinationPath string,
	claimRefreshDuration time.Duration,
	isLowRes bool,
) *recording {
	return &recording{
		ctx:                  ctx,
		db:                   db,
		camera:               camera,
		cameraMu:             cameraMu,
		destinationPath:      destinationPath,
		claimRefreshDuration: claimRefreshDuration,
		isLowRes:             isLowRes,
	}
}

func (r *recording) markVideoFailed(ctx context.Context, tx pgx.Tx, video *api.Video) error {
	filePath := filepath.Join(r.destinationPath, video.FileName)
	_, fileName := filepath.Split(filePath)
	video.FileName = fileName

	ext := filepath.Ext(fileName)
	thumbnailPath := fmt.Sprintf("%v.jpg", filePath[:len(filePath)-len(ext)])
	if err := helpers.GenerateThumbnail(filePath, thumbnailPath); err == nil {
		_, thumbnailName := filepath.Split(thumbnailPath)
		video.ThumbnailName = djangolang_helpers.Ptr(thumbnailName)
	} else {
		// A missing/incomplete segment should still be marked failed.
		video.ThumbnailName = nil
	}

	video.Status = djangolang_helpers.Ptr("failed")
	return video.Update(ctx, tx, false)
}

func (r *recording) updateCamera(ctx context.Context, tx pgx.Tx, timestamp time.Time) error {
	r.cameraMu.Lock()
	defer r.cameraMu.Unlock()

	r.camera.LastSeen = timestamp
	r.camera.SegmentProducerClaimedUntil = timestamp.Add(r.claimRefreshDuration)
	return r.camera.UpdateFields(ctx, tx, map[string]any{
		api.CameraTableLastSeenColumn:                    r.camera.LastSeen,
		api.CameraTableSegmentProducerClaimedUntilColumn: r.camera.SegmentProducerClaimedUntil,
		api.CameraTableUpdatedAtColumn:                   time.Now().UTC(),
	})
}

func (r *recording) onOpen(filePath string, _ float64, _ time.Time) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	tx, err := r.db.Begin(r.ctx)
	if err != nil {
		return err
	}
	defer func() { _ = tx.Rollback(r.ctx) }()

	if r.video != nil {
		if err := r.markVideoFailed(r.ctx, tx, r.video); err != nil {
			return err
		}
		r.video = nil
	}

	_, fileName := filepath.Split(filePath)
	r.video = &api.Video{
		FileName:  fileName,
		StartedAt: internal.GetNow(),
		Status:    djangolang_helpers.Ptr("recording"),
		CameraID:  r.camera.ID,
		IsLowRes:  r.isLowRes,
	}
	if err := r.video.Insert(r.ctx, tx, false, false); err != nil {
		return err
	}

	if err := r.updateCamera(r.ctx, tx, internal.GetNow()); err != nil {
		return err
	}

	return tx.Commit(r.ctx)
}

func (r *recording) onUpdate(_ string, fileSize float64, timestamp time.Time) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if r.video == nil {
		return fmt.Errorf("assertion failed: there should be a Video in-flight that we can update")
	}

	tx, err := r.db.Begin(r.ctx)
	if err != nil {
		return err
	}
	defer func() { _ = tx.Rollback(r.ctx) }()

	r.video.FileSize = djangolang_helpers.Ptr(fileSize)
	r.video.Duration = djangolang_helpers.Ptr(timestamp.Sub(r.video.StartedAt))
	if err := r.video.Update(r.ctx, tx, false); err != nil {
		return err
	}
	if err := r.updateCamera(r.ctx, tx, timestamp); err != nil {
		return err
	}

	return tx.Commit(r.ctx)
}

func (r *recording) onSave(filePath string, fileSize float64, timestamp time.Time) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if r.video == nil {
		return fmt.Errorf("assertion failed: there should be a Video in-flight that we can update")
	}

	tx, err := r.db.Begin(r.ctx)
	if err != nil {
		return err
	}
	defer func() { _ = tx.Rollback(r.ctx) }()

	_, fileName := filepath.Split(filePath)
	r.video.FileName = fileName

	ext := filepath.Ext(fileName)
	thumbnailPath := fmt.Sprintf("%v.jpg", filePath[:len(filePath)-len(ext)])
	if err := helpers.GenerateThumbnail(filePath, thumbnailPath); err == nil {
		_, thumbnailName := filepath.Split(thumbnailPath)
		r.video.ThumbnailName = djangolang_helpers.Ptr(thumbnailName)
	} else {
		log.Printf("warning: failed to generate thumbnail for %s: %v", filePath, err)
	}

	r.video.FileSize = djangolang_helpers.Ptr(fileSize)
	r.video.Duration = djangolang_helpers.Ptr(timestamp.Sub(r.video.StartedAt))
	if duration, durationErr := helpers.GetVideoDuration(filePath); durationErr == nil {
		r.video.Duration = &duration
	}

	r.video.EndedAt = djangolang_helpers.Ptr(timestamp)
	r.video.Status = djangolang_helpers.Ptr("needs detection")
	if err := r.video.Update(r.ctx, tx, false); err != nil {
		return err
	}
	if err := r.updateCamera(r.ctx, tx, timestamp); err != nil {
		return err
	}

	if err := tx.Commit(r.ctx); err != nil {
		return err
	}
	r.video = nil
	return nil
}

func (r *recording) cleanup(ctx context.Context, tx pgx.Tx) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if r.video != nil && (r.video.Status == nil || *r.video.Status != "needs detection") {
		return r.markVideoFailed(ctx, tx, r.video)
	}
	return nil
}
