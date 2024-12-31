package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/initialed85/djangolang/pkg/server"
)

type ClaimRequest struct {
	ClaimDurationSeconds float64 `json:"claim_duration_seconds"`
}

func RunServeWithEnvironment(
	httpMiddlewares []server.HTTPMiddleware,
	objectMiddlewares []server.ObjectMiddleware,
	addCustomHandlers func(chi.Router) error,
) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	port := config.Port()

	db, err := config.GetDBFromEnvironment(ctx)
	if err != nil {
		log.Fatalf("%v", err)
	}
	defer func() {
		db.Close()
	}()

	redisPool, err := config.GetRedisFromEnvironment()
	if err != nil {
		log.Fatalf("%v", err)
	}
	defer func() {
		_ = redisPool.Close()
	}()

	actualAddCustomHandlers := func(r chi.Router) error {
		claimVideoForObjectDetectorHandler, err := api.GetHTTPHandler(
			http.MethodPatch,
			"/custom/claim-video-for-object-detector",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams server.EmptyQueryParams,
				req ClaimRequest,
				rawReq any,
			) (*api.Video, error) {
				now := time.Now().UTC()

				claimUntil := now.Add(time.Second * time.Duration(req.ClaimDurationSeconds))

				if claimUntil.Sub(now) <= 0 {
					return nil, fmt.Errorf("claim_duration_seconds too short; must result in a claim that expires in the future")
				}

				tx, err := db.Begin(ctx)
				if err != nil {
					return nil, err
				}

				defer func() {
					_ = tx.Rollback(ctx)
				}()

				video := &api.Video{}

				// TODO: this is hacky; wait for as long as other processes are _probably_ claiming for plus some fudge (but not a certainty of course)
				err = video.AdvisoryLockWithRetries(ctx, tx, 3, time.Duration(req.ClaimDurationSeconds)+(time.Second*2), time.Second*1)
				if err != nil {
					return nil, err
				}

				videos, _, _, _, _, err := api.SelectVideos(
					ctx,
					tx,
					fmt.Sprintf(
						"%v = 'needs detection' AND %v < now()",
						api.VideoTableStatusColumn,
						api.VideoTableObjectDetectorClaimedUntilColumn,
					),
					internal.Ptr(fmt.Sprintf(
						"%v DESC",
						api.VideoTableStartedAtColumn,
					)),
					internal.Ptr(1),
					nil,
				)
				if err != nil {
					return nil, err
				}

				if len(videos) == 0 {
					return nil, nil
				}

				if len(videos) > 1 {
					return nil, fmt.Errorf("wanted exactly 1 unclaimed video, got %d", len(videos))
				}

				video = videos[0]

				video.ObjectDetectorClaimedUntil = claimUntil
				video.ObjectTrackerClaimedUntil = time.Time{} // zero to ensure we don't wipe out an existing value

				err = video.Update(ctx, tx, false)
				if err != nil {
					return nil, err
				}

				err = tx.Commit(ctx)
				if err != nil {
					return nil, err
				}

				return video, nil
			},
		)
		if err != nil {
			return err
		}

		r.Patch("/claim-video-for-object-detector", claimVideoForObjectDetectorHandler.ServeHTTP)

		if addCustomHandlers != nil {
			err = addCustomHandlers(r)
			if err != nil {
				return err
			}
		}

		return nil
	}

	api.RunServeWithArguments(ctx, cancel, port, db, redisPool, nil, nil, actualAddCustomHandlers)
}

func main() {
	if len(os.Args) < 2 {
		log.Fatal("first argument must be command (one of 'serve', 'dump-openapi-json', 'dump-openapi-yaml')")
	}

	command := strings.TrimSpace(strings.ToLower(os.Args[1]))

	switch command {

	case "dump-openapi-json":
		api.RunDumpOpenAPIJSON()

	case "dump-openapi-yaml":
		api.RunDumpOpenAPIYAML()

	case "serve":
		RunServeWithEnvironment(nil, nil, nil)
	}
}
