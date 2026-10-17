package audio_detector

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"
	"os/signal"
	"path/filepath"
	"syscall"
	"time"

	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

type Settings struct {
	DestinationPath   string
	FFmpegPath        string
	ClaimLeaseSeconds int
	IdleSleepSeconds  int
	MaxVideos         int
	Enabled           bool
	Config            Config
}

func GetSettings() (Settings, error) {
	settings := Settings{}

	var err error

	settings.DestinationPath, err = internal.GetEnvironment("DESTINATION_PATH", false, internal.Ptr("/srv/media"))
	if err != nil {
		return settings, err
	}

	settings.FFmpegPath, err = internal.GetEnvironment("FFMPEG_PATH", false, internal.Ptr("ffmpeg"))
	if err != nil {
		return settings, err
	}

	settings.ClaimLeaseSeconds, err = internal.GetEnvironment("CLAIM_LEASE_SECONDS", false, internal.Ptr(600))
	if err != nil {
		return settings, err
	}

	settings.IdleSleepSeconds, err = internal.GetEnvironment("IDLE_SLEEP_SECONDS", false, internal.Ptr(30))
	if err != nil {
		return settings, err
	}

	settings.MaxVideos, err = internal.GetEnvironment("MAX_VIDEOS", false, internal.Ptr(0))
	if err != nil {
		return settings, err
	}

	settings.Enabled, err = internal.GetEnvironment("ENABLED", false, internal.Ptr(true), true)
	if err != nil {
		return settings, err
	}

	sampleRate, err := internal.GetEnvironment("SAMPLE_RATE", false, internal.Ptr(48000))
	if err != nil {
		return settings, err
	}

	windowMs, err := internal.GetEnvironment("WINDOW_MS", false, internal.Ptr(50))
	if err != nil {
		return settings, err
	}

	gridMs, err := internal.GetEnvironment("GRID_MS", false, internal.Ptr(100))
	if err != nil {
		return settings, err
	}

	triggerDb, err := internal.GetEnvironment("TRIGGER_DB", false, internal.Ptr(6.0))
	if err != nil {
		return settings, err
	}

	maxGapWindows, err := internal.GetEnvironment("MAX_GAP_WINDOWS", false, internal.Ptr(2))
	if err != nil {
		return settings, err
	}

	fftSize, err := internal.GetEnvironment("FFT_SIZE", false, internal.Ptr(2048))
	if err != nil {
		return settings, err
	}

	melBands, err := internal.GetEnvironment("MEL_BANDS", false, internal.Ptr(16))
	if err != nil {
		return settings, err
	}

	settings.Config = Config{
		SampleRate:    sampleRate,
		WindowMs:      windowMs,
		GridMs:        gridMs,
		TriggerDb:     triggerDb,
		MaxGapWindows: maxGapWindows,
		FFTSize:       fftSize,
		MelBands:      melBands,
	}

	return settings, nil
}

func claimVideo(ctx context.Context, db *pgxpool.Pool, settings Settings) (*api.Video, error) {
	lease := time.Duration(settings.ClaimLeaseSeconds) * time.Second

	tx, err := db.Begin(ctx)
	if err != nil {
		return nil, err
	}

	defer func() {
		_ = tx.Rollback(ctx)
	}()

	orderBy := fmt.Sprintf(
		"%s ASC, %s ASC, %s ASC",
		api.VideoTableAudioDetectorClaimedUntilColumn,
		api.VideoTableStartedAtColumn,
		api.VideoTableIDColumn,
	)

	where := fmt.Sprintf(
		"%s IS NULL AND %s = false AND %s IN ('needs detection', 'needs tracking')",
		api.VideoTableAudioDetectedAtColumn,
		api.VideoTableIsLowResColumn,
		api.VideoTableStatusColumn,
	)

	video, err := api.AudioDetectorClaimVideo(
		ctx,
		tx,
		time.Now().UTC().Add(lease),
		lease,
		where,
		&orderBy,
	)
	if err != nil {
		return nil, err
	}

	if video == nil {
		return nil, nil
	}

	if err = tx.Commit(ctx); err != nil {
		return nil, err
	}

	return video, nil
}

func summarizeEvent(event Event, seenAt time.Time) map[string]any {
	return map[string]any{
		"offset_ms":     event.OffsetMs,
		"seen_at":       seenAt.UTC().Format(time.RFC3339),
		"duration_ms":   event.DurationMs,
		"kind":          event.Kind,
		"prominence_db": event.ProminenceDb,
		"peak_dbfs":     event.PeakDbfs,
		"crest_db":      event.CrestDb,
		"score":         event.Score,
	}
}

func spectralToMap(spectral Spectral) map[string]any {
	return map[string]any{
		"centroid_hz":     spectral.CentroidHz,
		"centroid_std_hz": spectral.CentroidStdHz,
		"flux":            spectral.Flux,
		"flatness":        spectral.Flatness,
		"tonality":        spectral.Tonality,
		"bands":           spectral.Bands,
	}
}

func markVideo(
	ctx context.Context,
	tx pgx.Tx,
	video *api.Video,
	profile Profile,
	summary []map[string]any,
) error {
	now := time.Now().UTC()

	return video.UpdateFields(ctx, tx, map[string]any{
		api.VideoTableAudioDetectedAtColumn:           now,
		api.VideoTableAudioProfileColumn:              profile,
		api.VideoTableAudioSummaryColumn:              summary,
		api.VideoTableAudioDetectorClaimedUntilColumn: now,
	})
}

func processVideo(ctx context.Context, db *pgxpool.Pool, video *api.Video, settings Settings) error {
	path := filepath.Join(settings.DestinationPath, video.FileName)

	pcm, decodeErr := DecodePCM(ctx, path, settings.Config.SampleRate, settings.FFmpegPath)

	// A decode failure other than "no audio stream" is retried once the claim lease expires,
	// so it must not be turned into a completed row.
	if decodeErr != nil && !errors.Is(decodeErr, ErrNoAudio) {
		return decodeErr
	}

	tx, err := db.Begin(ctx)
	if err != nil {
		return err
	}

	defer func() {
		_ = tx.Rollback(ctx)
	}()

	if errors.Is(decodeErr, ErrNoAudio) {
		// Values stays an empty array, not null, so UI consumers can index it unconditionally.
		profile := Profile{
			Version:    1,
			SampleRate: settings.Config.SampleRate,
			NoAudio:    true,
			Values:     []float64{},
		}

		if err = markVideo(ctx, tx, video, profile, []map[string]any{}); err != nil {
			return err
		}

		log.Printf("%s | %s: no audio stream", video.FileName, video.ID)

		return tx.Commit(ctx)
	}

	events, profile := Analyze(pcm, settings.Config)

	rows := make([]*api.AudioEvent, 0, len(events))
	summary := make([]map[string]any, 0, len(events))

	for _, event := range events {
		seenAt := video.StartedAt.Add(time.Duration(event.OffsetMs) * time.Millisecond)

		rows = append(rows, &api.AudioEvent{
			CameraID:     video.CameraID,
			VideoID:      video.ID,
			SeenAt:       seenAt.UTC(),
			OffsetMs:     event.OffsetMs,
			EndedAt:      seenAt.Add(time.Duration(event.DurationMs) * time.Millisecond).UTC(),
			DurationMs:   event.DurationMs,
			Kind:         event.Kind,
			RmsDbfs:      event.RmsDbfs,
			PeakDbfs:     event.PeakDbfs,
			FloorDbfs:    event.FloorDbfs,
			ProminenceDb: event.ProminenceDb,
			CrestDb:      event.CrestDb,
			Score:        event.Score,
			Spectral:     spectralToMap(event.Spectral),
		})

		summary = append(summary, summarizeEvent(event, seenAt))
	}

	if len(rows) > 0 {
		if _, err = api.InsertAudioEvents(ctx, tx, rows, false, false); err != nil {
			return err
		}
	}

	if err = markVideo(ctx, tx, video, profile, summary); err != nil {
		return err
	}

	if err = tx.Commit(ctx); err != nil {
		return err
	}

	log.Printf(
		"%s | %s: %d windows, floor %.1f dBFS, %d events",
		video.FileName,
		video.ID,
		profile.Windows,
		profile.FloorDbfs,
		len(events),
	)

	return nil
}

func Run() error {
	settings, err := GetSettings()
	if err != nil {
		return err
	}

	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer cancel()

	db, err := config.GetDBFromEnvironment(ctx)
	if err != nil {
		return err
	}
	defer func() {
		db.Close()
	}()

	if !settings.Enabled {
		log.Printf("audio detector is disabled")
		select {
		case <-ctx.Done():
			return nil
		case <-time.After(time.Hour):
			return nil
		}
	}

	log.Printf("audio detector started; watching %s", settings.DestinationPath)

	for processed := 0; ; processed++ {
		if settings.MaxVideos > 0 && processed >= settings.MaxVideos {
			log.Printf("audio detector reached its limit of %d videos", settings.MaxVideos)
			return nil
		}

		if err = ctx.Err(); err != nil {
			return nil
		}

		video, err := claimVideo(ctx, db, settings)
		if err != nil {
			log.Printf("failed to claim a video: %v", err)
			video = nil
		}

		if video == nil {
			select {
			case <-ctx.Done():
				return nil
			case <-time.After(time.Duration(settings.IdleSleepSeconds) * time.Second):
			}

			continue
		}

		if err = processVideo(ctx, db, video, settings); err != nil {
			log.Printf("failed to process %s | %s: %v", video.FileName, video.ID, err)
			continue
		}
	}
}
