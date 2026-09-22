package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/camry/pkg/helpers"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/initialed85/djangolang/pkg/query"
	"github.com/initialed85/djangolang/pkg/types"
)

func main() {
	ctx := context.Background()
	destinationPath := os.Getenv("DESTINATION_PATH")
	if destinationPath == "" {
		destinationPath = "/srv/media"
	}

	db, err := config.GetDBFromEnvironment(ctx)
	if err != nil {
		log.Fatalf("failed to connect to database: %v", err)
	}
	defer db.Close()

	// Read the rows first so ffmpeg never runs while holding a database transaction.
	videos := make([]*api.Video, 0)
	for offset := 0; ; offset += 500 {
		tx, err := db.Begin(ctx)
		if err != nil {
			log.Fatalf("failed to begin select transaction: %v", err)
		}

		limit := 500
		batch, _, _, _, _, err := api.SelectVideos(
			ctx,
			tx,
			"thumbnail_name IS NULL AND ended_at IS NOT NULL AND status != 'recording'",
			internal.Ptr("started_at ASC"),
			internal.Ptr(limit),
			internal.Ptr(offset),
		)
		_ = tx.Rollback(ctx)
		if err != nil {
			log.Fatalf("failed to select videos: %v", err)
		}
		videos = append(videos, batch...)
		if len(batch) < limit {
			break
		}
	}

	log.Printf("found %d videos without thumbnails", len(videos))
	for i, video := range videos {
		if strings.TrimSpace(video.FileName) == "" {
			log.Printf("[%d/%d] skipping %s: empty file name", i+1, len(videos), video.ID)
			continue
		}

		videoPath := filepath.Join(destinationPath, video.FileName)
		thumbnailPath := strings.TrimSuffix(videoPath, filepath.Ext(videoPath)) + ".jpg"
		if err := helpers.GenerateThumbnail(videoPath, thumbnailPath); err != nil {
			log.Printf("[%d/%d] failed %s: %v", i+1, len(videos), video.FileName, err)
			continue
		}

		_, thumbnailName := filepath.Split(thumbnailPath)
		tx, err := db.Begin(ctx)
		if err != nil {
			log.Printf("[%d/%d] failed to begin update for %s: %v", i+1, len(videos), video.FileName, err)
			continue
		}

		thumbnailValue, err := types.FormatString(&thumbnailName)
		if err == nil {
			idValue, idErr := types.FormatUUID(video.ID)
			if idErr != nil {
				err = idErr
			} else {
				_, err = query.Update(
					ctx,
					tx,
					api.VideoTableWithSchema,
					[]string{api.VideoTableThumbnailNameColumn, api.VideoTableUpdatedAtColumn},
					fmt.Sprintf("%v = $$??", api.VideoTableIDColumn),
					api.VideoTableColumns,
					thumbnailValue,
					time.Now().UTC(),
					idValue,
				)
			}
		}
		if err != nil {
			_ = tx.Rollback(ctx)
			log.Printf("[%d/%d] failed to save %s: %v", i+1, len(videos), video.FileName, err)
			continue
		}
		if err := tx.Commit(ctx); err != nil {
			log.Printf("[%d/%d] failed to commit %s: %v", i+1, len(videos), video.FileName, err)
			continue
		}

		log.Printf("[%d/%d] generated %s", i+1, len(videos), thumbnailName)
	}
}
