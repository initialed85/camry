package api

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"net/http"
	"net/netip"
	"slices"
	"strings"
	"time"

	"github.com/cridenour/go-postgis"
	"github.com/go-chi/chi/v5"
	"github.com/gomodule/redigo/redis"
	"github.com/google/uuid"
	"github.com/initialed85/djangolang/pkg/helpers"
	"github.com/initialed85/djangolang/pkg/introspect"
	"github.com/initialed85/djangolang/pkg/query"
	"github.com/initialed85/djangolang/pkg/server"
	"github.com/initialed85/djangolang/pkg/stream"
	"github.com/initialed85/djangolang/pkg/types"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/jackc/pgx/v5/pgxpool"
	"golang.org/x/exp/maps"
)

type Video struct {
	ID                                  uuid.UUID      `json:"id"`
	CreatedAt                           time.Time      `json:"created_at"`
	UpdatedAt                           time.Time      `json:"updated_at"`
	DeletedAt                           *time.Time     `json:"deleted_at"`
	FileName                            string         `json:"file_name"`
	StartedAt                           time.Time      `json:"started_at"`
	EndedAt                             *time.Time     `json:"ended_at"`
	Duration                            *time.Duration `json:"duration"`
	FileSize                            *float64       `json:"file_size"`
	ThumbnailName                       *string        `json:"thumbnail_name"`
	Status                              *string        `json:"status"`
	ObjectDetectorClaimedUntil          time.Time      `json:"object_detector_claimed_until"`
	ObjectTrackerClaimedUntil           time.Time      `json:"object_tracker_claimed_until"`
	CameraID                            uuid.UUID      `json:"camera_id"`
	CameraIDObject                      *Camera        `json:"camera_id_object"`
	ReferencedByDetectionVideoIDObjects []*Detection   `json:"referenced_by_detection_video_id_objects"`
}

var VideoTable = "video"

var (
	VideoTableIDColumn                         = "id"
	VideoTableCreatedAtColumn                  = "created_at"
	VideoTableUpdatedAtColumn                  = "updated_at"
	VideoTableDeletedAtColumn                  = "deleted_at"
	VideoTableFileNameColumn                   = "file_name"
	VideoTableStartedAtColumn                  = "started_at"
	VideoTableEndedAtColumn                    = "ended_at"
	VideoTableDurationColumn                   = "duration"
	VideoTableFileSizeColumn                   = "file_size"
	VideoTableThumbnailNameColumn              = "thumbnail_name"
	VideoTableStatusColumn                     = "status"
	VideoTableObjectDetectorClaimedUntilColumn = "object_detector_claimed_until"
	VideoTableObjectTrackerClaimedUntilColumn  = "object_tracker_claimed_until"
	VideoTableCameraIDColumn                   = "camera_id"
)

var (
	VideoTableIDColumnWithTypeCast                         = `"id" AS id`
	VideoTableCreatedAtColumnWithTypeCast                  = `"created_at" AS created_at`
	VideoTableUpdatedAtColumnWithTypeCast                  = `"updated_at" AS updated_at`
	VideoTableDeletedAtColumnWithTypeCast                  = `"deleted_at" AS deleted_at`
	VideoTableFileNameColumnWithTypeCast                   = `"file_name" AS file_name`
	VideoTableStartedAtColumnWithTypeCast                  = `"started_at" AS started_at`
	VideoTableEndedAtColumnWithTypeCast                    = `"ended_at" AS ended_at`
	VideoTableDurationColumnWithTypeCast                   = `"duration" AS duration`
	VideoTableFileSizeColumnWithTypeCast                   = `"file_size" AS file_size`
	VideoTableThumbnailNameColumnWithTypeCast              = `"thumbnail_name" AS thumbnail_name`
	VideoTableStatusColumnWithTypeCast                     = `"status" AS status`
	VideoTableObjectDetectorClaimedUntilColumnWithTypeCast = `"object_detector_claimed_until" AS object_detector_claimed_until`
	VideoTableObjectTrackerClaimedUntilColumnWithTypeCast  = `"object_tracker_claimed_until" AS object_tracker_claimed_until`
	VideoTableCameraIDColumnWithTypeCast                   = `"camera_id" AS camera_id`
)

var VideoTableColumns = []string{
	VideoTableIDColumn,
	VideoTableCreatedAtColumn,
	VideoTableUpdatedAtColumn,
	VideoTableDeletedAtColumn,
	VideoTableFileNameColumn,
	VideoTableStartedAtColumn,
	VideoTableEndedAtColumn,
	VideoTableDurationColumn,
	VideoTableFileSizeColumn,
	VideoTableThumbnailNameColumn,
	VideoTableStatusColumn,
	VideoTableObjectDetectorClaimedUntilColumn,
	VideoTableObjectTrackerClaimedUntilColumn,
	VideoTableCameraIDColumn,
}

var VideoTableColumnsWithTypeCasts = []string{
	VideoTableIDColumnWithTypeCast,
	VideoTableCreatedAtColumnWithTypeCast,
	VideoTableUpdatedAtColumnWithTypeCast,
	VideoTableDeletedAtColumnWithTypeCast,
	VideoTableFileNameColumnWithTypeCast,
	VideoTableStartedAtColumnWithTypeCast,
	VideoTableEndedAtColumnWithTypeCast,
	VideoTableDurationColumnWithTypeCast,
	VideoTableFileSizeColumnWithTypeCast,
	VideoTableThumbnailNameColumnWithTypeCast,
	VideoTableStatusColumnWithTypeCast,
	VideoTableObjectDetectorClaimedUntilColumnWithTypeCast,
	VideoTableObjectTrackerClaimedUntilColumnWithTypeCast,
	VideoTableCameraIDColumnWithTypeCast,
}

var VideoIntrospectedTable *introspect.Table

var VideoTableColumnLookup map[string]*introspect.Column

var (
	VideoTablePrimaryKeyColumn = VideoTableIDColumn
)

func init() {
	VideoIntrospectedTable = tableByName[VideoTable]

	/* only needed during templating */
	if VideoIntrospectedTable == nil {
		VideoIntrospectedTable = &introspect.Table{}
	}

	VideoTableColumnLookup = VideoIntrospectedTable.ColumnByName
}

type VideoOnePathParams struct {
	PrimaryKey uuid.UUID `json:"primaryKey"`
}

type VideoLoadQueryParams struct {
	Depth *int `json:"depth"`
}

/*
TODO: find a way to not need this- there is a piece in the templating logic
that uses goimports but pending where the code is built, it may resolve
the packages to import to the wrong ones (causing odd failures)
these are just here to ensure we don't get unused imports
*/
var _ = []any{
	time.Time{},
	uuid.UUID{},
	pgtype.Hstore{},
	postgis.PointZ{},
	netip.Prefix{},
	errors.Is,
	sql.ErrNoRows,
}

func (m *Video) GetPrimaryKeyColumn() string {
	return VideoTablePrimaryKeyColumn
}

func (m *Video) GetPrimaryKeyValue() any {
	return m.ID
}

func (m *Video) FromItem(item map[string]any) error {
	if item == nil {
		return fmt.Errorf(
			"item unexpectedly nil during VideoFromItem",
		)
	}

	if len(item) == 0 {
		return fmt.Errorf(
			"item unexpectedly empty during VideoFromItem",
		)
	}

	wrapError := func(k string, v any, err error) error {
		return fmt.Errorf("%v: %#+v; error; %v", k, v, err)
	}

	for k, v := range item {
		_, ok := VideoTableColumnLookup[k]
		if !ok {
			return fmt.Errorf(
				"item contained unexpected key %#+v during VideoFromItem; item: %#+v",
				k, item,
			)
		}

		switch k {
		case "id":
			if v == nil {
				continue
			}

			temp1, err := types.ParseUUID(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(uuid.UUID)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuid.UUID", temp1))
				}
			}

			m.ID = temp2

		case "created_at":
			if v == nil {
				continue
			}

			temp1, err := types.ParseTime(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Time)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uucreated_at.UUID", temp1))
				}
			}

			m.CreatedAt = temp2

		case "updated_at":
			if v == nil {
				continue
			}

			temp1, err := types.ParseTime(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Time)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuupdated_at.UUID", temp1))
				}
			}

			m.UpdatedAt = temp2

		case "deleted_at":
			if v == nil {
				continue
			}

			temp1, err := types.ParseTime(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Time)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uudeleted_at.UUID", temp1))
				}
			}

			m.DeletedAt = &temp2

		case "file_name":
			if v == nil {
				continue
			}

			temp1, err := types.ParseString(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(string)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uufile_name.UUID", temp1))
				}
			}

			m.FileName = temp2

		case "started_at":
			if v == nil {
				continue
			}

			temp1, err := types.ParseTime(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Time)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uustarted_at.UUID", temp1))
				}
			}

			m.StartedAt = temp2

		case "ended_at":
			if v == nil {
				continue
			}

			temp1, err := types.ParseTime(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Time)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuended_at.UUID", temp1))
				}
			}

			m.EndedAt = &temp2

		case "duration":
			if v == nil {
				continue
			}

			temp1, err := types.ParseDuration(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Duration)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuduration.UUID", temp1))
				}
			}

			m.Duration = &temp2

		case "file_size":
			if v == nil {
				continue
			}

			temp1, err := types.ParseFloat(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(float64)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uufile_size.UUID", temp1))
				}
			}

			m.FileSize = &temp2

		case "thumbnail_name":
			if v == nil {
				continue
			}

			temp1, err := types.ParseString(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(string)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuthumbnail_name.UUID", temp1))
				}
			}

			m.ThumbnailName = &temp2

		case "status":
			if v == nil {
				continue
			}

			temp1, err := types.ParseString(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(string)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uustatus.UUID", temp1))
				}
			}

			m.Status = &temp2

		case "object_detector_claimed_until":
			if v == nil {
				continue
			}

			temp1, err := types.ParseTime(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Time)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuobject_detector_claimed_until.UUID", temp1))
				}
			}

			m.ObjectDetectorClaimedUntil = temp2

		case "object_tracker_claimed_until":
			if v == nil {
				continue
			}

			temp1, err := types.ParseTime(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(time.Time)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuobject_tracker_claimed_until.UUID", temp1))
				}
			}

			m.ObjectTrackerClaimedUntil = temp2

		case "camera_id":
			if v == nil {
				continue
			}

			temp1, err := types.ParseUUID(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(uuid.UUID)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uucamera_id.UUID", temp1))
				}
			}

			m.CameraID = temp2

		}
	}

	return nil
}

func (m *Video) Reload(ctx context.Context, tx pgx.Tx, includeDeleteds ...bool) error {
	extraWhere := ""
	if len(includeDeleteds) > 0 && includeDeleteds[0] {
		if slices.Contains(VideoTableColumns, "deleted_at") {
			extraWhere = "\n    AND (deleted_at IS null OR deleted_at IS NOT null)"
		}
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	t, err := SelectVideo(
		ctx,
		tx,
		fmt.Sprintf("%v = $1%v", m.GetPrimaryKeyColumn(), extraWhere),
		m.GetPrimaryKeyValue(),
	)
	if err != nil {
		return err
	}

	m.ID = t.ID
	m.CreatedAt = t.CreatedAt
	m.UpdatedAt = t.UpdatedAt
	m.DeletedAt = t.DeletedAt
	m.FileName = t.FileName
	m.StartedAt = t.StartedAt
	m.EndedAt = t.EndedAt
	m.Duration = t.Duration
	m.FileSize = t.FileSize
	m.ThumbnailName = t.ThumbnailName
	m.Status = t.Status
	m.ObjectDetectorClaimedUntil = t.ObjectDetectorClaimedUntil
	m.ObjectTrackerClaimedUntil = t.ObjectTrackerClaimedUntil
	m.CameraID = t.CameraID
	m.CameraIDObject = t.CameraIDObject
	m.ReferencedByDetectionVideoIDObjects = t.ReferencedByDetectionVideoIDObjects

	return nil
}

func (m *Video) Insert(ctx context.Context, tx pgx.Tx, setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) error {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setPrimaryKey && (setZeroValues || !types.IsZeroUUID(m.ID) || slices.Contains(forceSetValuesForFields, VideoTableIDColumn) || isRequired(VideoTableColumnLookup, VideoTableIDColumn)) {
		columns = append(columns, VideoTableIDColumn)

		v, err := types.FormatUUID(m.ID)
		if err != nil {
			return fmt.Errorf("failed to handle m.ID: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, VideoTableCreatedAtColumn) || isRequired(VideoTableColumnLookup, VideoTableCreatedAtColumn) {
		columns = append(columns, VideoTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.CreatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, VideoTableUpdatedAtColumn) || isRequired(VideoTableColumnLookup, VideoTableUpdatedAtColumn) {
		columns = append(columns, VideoTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.UpdatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, VideoTableDeletedAtColumn) || isRequired(VideoTableColumnLookup, VideoTableDeletedAtColumn) {
		columns = append(columns, VideoTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.DeletedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.FileName) || slices.Contains(forceSetValuesForFields, VideoTableFileNameColumn) || isRequired(VideoTableColumnLookup, VideoTableFileNameColumn) {
		columns = append(columns, VideoTableFileNameColumn)

		v, err := types.FormatString(m.FileName)
		if err != nil {
			return fmt.Errorf("failed to handle m.FileName: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.StartedAt) || slices.Contains(forceSetValuesForFields, VideoTableStartedAtColumn) || isRequired(VideoTableColumnLookup, VideoTableStartedAtColumn) {
		columns = append(columns, VideoTableStartedAtColumn)

		v, err := types.FormatTime(m.StartedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.StartedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.EndedAt) || slices.Contains(forceSetValuesForFields, VideoTableEndedAtColumn) || isRequired(VideoTableColumnLookup, VideoTableEndedAtColumn) {
		columns = append(columns, VideoTableEndedAtColumn)

		v, err := types.FormatTime(m.EndedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.EndedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroDuration(m.Duration) || slices.Contains(forceSetValuesForFields, VideoTableDurationColumn) || isRequired(VideoTableColumnLookup, VideoTableDurationColumn) {
		columns = append(columns, VideoTableDurationColumn)

		v, err := types.FormatDuration(m.Duration)
		if err != nil {
			return fmt.Errorf("failed to handle m.Duration: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.FileSize) || slices.Contains(forceSetValuesForFields, VideoTableFileSizeColumn) || isRequired(VideoTableColumnLookup, VideoTableFileSizeColumn) {
		columns = append(columns, VideoTableFileSizeColumn)

		v, err := types.FormatFloat(m.FileSize)
		if err != nil {
			return fmt.Errorf("failed to handle m.FileSize: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.ThumbnailName) || slices.Contains(forceSetValuesForFields, VideoTableThumbnailNameColumn) || isRequired(VideoTableColumnLookup, VideoTableThumbnailNameColumn) {
		columns = append(columns, VideoTableThumbnailNameColumn)

		v, err := types.FormatString(m.ThumbnailName)
		if err != nil {
			return fmt.Errorf("failed to handle m.ThumbnailName: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Status) || slices.Contains(forceSetValuesForFields, VideoTableStatusColumn) || isRequired(VideoTableColumnLookup, VideoTableStatusColumn) {
		columns = append(columns, VideoTableStatusColumn)

		v, err := types.FormatString(m.Status)
		if err != nil {
			return fmt.Errorf("failed to handle m.Status: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.ObjectDetectorClaimedUntil) || slices.Contains(forceSetValuesForFields, VideoTableObjectDetectorClaimedUntilColumn) || isRequired(VideoTableColumnLookup, VideoTableObjectDetectorClaimedUntilColumn) {
		columns = append(columns, VideoTableObjectDetectorClaimedUntilColumn)

		v, err := types.FormatTime(m.ObjectDetectorClaimedUntil)
		if err != nil {
			return fmt.Errorf("failed to handle m.ObjectDetectorClaimedUntil: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.ObjectTrackerClaimedUntil) || slices.Contains(forceSetValuesForFields, VideoTableObjectTrackerClaimedUntilColumn) || isRequired(VideoTableColumnLookup, VideoTableObjectTrackerClaimedUntilColumn) {
		columns = append(columns, VideoTableObjectTrackerClaimedUntilColumn)

		v, err := types.FormatTime(m.ObjectTrackerClaimedUntil)
		if err != nil {
			return fmt.Errorf("failed to handle m.ObjectTrackerClaimedUntil: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) || slices.Contains(forceSetValuesForFields, VideoTableCameraIDColumn) || isRequired(VideoTableColumnLookup, VideoTableCameraIDColumn) {
		columns = append(columns, VideoTableCameraIDColumn)

		v, err := types.FormatUUID(m.CameraID)
		if err != nil {
			return fmt.Errorf("failed to handle m.CameraID: %v", err)
		}

		values = append(values, v)
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	item, err := query.Insert(
		ctx,
		tx,
		VideoTable,
		columns,
		nil,
		false,
		false,
		VideoTableColumns,
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to insert %#+v; %v", m, err)
	}
	v := (*item)[VideoTableIDColumn]

	if v == nil {
		return fmt.Errorf("failed to find %v in %#+v", VideoTableIDColumn, item)
	}

	wrapError := func(err error) error {
		return fmt.Errorf(
			"failed to treat %v: %#+v as uuid.UUID: %v",
			VideoTableIDColumn,
			(*item)[VideoTableIDColumn],
			err,
		)
	}

	temp1, err := types.ParseUUID(v)
	if err != nil {
		return wrapError(err)
	}

	temp2, ok := temp1.(uuid.UUID)
	if !ok {
		return wrapError(fmt.Errorf("failed to cast to uuid.UUID"))
	}

	m.ID = temp2

	err = m.Reload(ctx, tx, slices.Contains(forceSetValuesForFields, "deleted_at"))
	if err != nil {
		return fmt.Errorf("failed to reload after insert: %v", err)
	}

	return nil
}

func (m *Video) Update(ctx context.Context, tx pgx.Tx, setZeroValues bool, forceSetValuesForFields ...string) error {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, VideoTableCreatedAtColumn) {
		columns = append(columns, VideoTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.CreatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, VideoTableUpdatedAtColumn) {
		columns = append(columns, VideoTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.UpdatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, VideoTableDeletedAtColumn) {
		columns = append(columns, VideoTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.DeletedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.FileName) || slices.Contains(forceSetValuesForFields, VideoTableFileNameColumn) {
		columns = append(columns, VideoTableFileNameColumn)

		v, err := types.FormatString(m.FileName)
		if err != nil {
			return fmt.Errorf("failed to handle m.FileName: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.StartedAt) || slices.Contains(forceSetValuesForFields, VideoTableStartedAtColumn) {
		columns = append(columns, VideoTableStartedAtColumn)

		v, err := types.FormatTime(m.StartedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.StartedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.EndedAt) || slices.Contains(forceSetValuesForFields, VideoTableEndedAtColumn) {
		columns = append(columns, VideoTableEndedAtColumn)

		v, err := types.FormatTime(m.EndedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.EndedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroDuration(m.Duration) || slices.Contains(forceSetValuesForFields, VideoTableDurationColumn) {
		columns = append(columns, VideoTableDurationColumn)

		v, err := types.FormatDuration(m.Duration)
		if err != nil {
			return fmt.Errorf("failed to handle m.Duration: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.FileSize) || slices.Contains(forceSetValuesForFields, VideoTableFileSizeColumn) {
		columns = append(columns, VideoTableFileSizeColumn)

		v, err := types.FormatFloat(m.FileSize)
		if err != nil {
			return fmt.Errorf("failed to handle m.FileSize: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.ThumbnailName) || slices.Contains(forceSetValuesForFields, VideoTableThumbnailNameColumn) {
		columns = append(columns, VideoTableThumbnailNameColumn)

		v, err := types.FormatString(m.ThumbnailName)
		if err != nil {
			return fmt.Errorf("failed to handle m.ThumbnailName: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Status) || slices.Contains(forceSetValuesForFields, VideoTableStatusColumn) {
		columns = append(columns, VideoTableStatusColumn)

		v, err := types.FormatString(m.Status)
		if err != nil {
			return fmt.Errorf("failed to handle m.Status: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.ObjectDetectorClaimedUntil) || slices.Contains(forceSetValuesForFields, VideoTableObjectDetectorClaimedUntilColumn) {
		columns = append(columns, VideoTableObjectDetectorClaimedUntilColumn)

		v, err := types.FormatTime(m.ObjectDetectorClaimedUntil)
		if err != nil {
			return fmt.Errorf("failed to handle m.ObjectDetectorClaimedUntil: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.ObjectTrackerClaimedUntil) || slices.Contains(forceSetValuesForFields, VideoTableObjectTrackerClaimedUntilColumn) {
		columns = append(columns, VideoTableObjectTrackerClaimedUntilColumn)

		v, err := types.FormatTime(m.ObjectTrackerClaimedUntil)
		if err != nil {
			return fmt.Errorf("failed to handle m.ObjectTrackerClaimedUntil: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) || slices.Contains(forceSetValuesForFields, VideoTableCameraIDColumn) {
		columns = append(columns, VideoTableCameraIDColumn)

		v, err := types.FormatUUID(m.CameraID)
		if err != nil {
			return fmt.Errorf("failed to handle m.CameraID: %v", err)
		}

		values = append(values, v)
	}

	v, err := types.FormatUUID(m.ID)
	if err != nil {
		return fmt.Errorf("failed to handle m.ID: %v", err)
	}

	values = append(values, v)

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	_, err = query.Update(
		ctx,
		tx,
		VideoTable,
		columns,
		fmt.Sprintf("%v = $$??", VideoTableIDColumn),
		VideoTableColumns,
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to update %#+v; %v", m, err)
	}

	err = m.Reload(ctx, tx, slices.Contains(forceSetValuesForFields, "deleted_at"))
	if err != nil {
		return fmt.Errorf("failed to reload after update")
	}

	return nil
}

func (m *Video) Delete(ctx context.Context, tx pgx.Tx, hardDeletes ...bool) error {
	hardDelete := false
	if len(hardDeletes) > 0 {
		hardDelete = hardDeletes[0]
	}

	if !hardDelete && slices.Contains(VideoTableColumns, "deleted_at") {
		m.DeletedAt = helpers.Ptr(time.Now().UTC())
		err := m.Update(ctx, tx, false, "deleted_at")
		if err != nil {
			return fmt.Errorf("failed to soft-delete (update) %#+v; %v", m, err)
		}
	}

	values := make([]any, 0)
	v, err := types.FormatUUID(m.ID)
	if err != nil {
		return fmt.Errorf("failed to handle m.ID: %v", err)
	}

	values = append(values, v)

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	err = query.Delete(
		ctx,
		tx,
		VideoTable,
		fmt.Sprintf("%v = $$??", VideoTableIDColumn),
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to delete %#+v; %v", m, err)
	}

	_ = m.Reload(ctx, tx, true)

	return nil
}

func (m *Video) LockTable(ctx context.Context, tx pgx.Tx, noWait bool) error {
	return query.LockTable(ctx, tx, VideoTable, noWait)
}

func SelectVideos(ctx context.Context, tx pgx.Tx, where string, orderBy *string, limit *int, offset *int, values ...any) ([]*Video, error) {
	if slices.Contains(VideoTableColumns, "deleted_at") {
		if !strings.Contains(where, "deleted_at") {
			if where != "" {
				where += "\n    AND "
			}

			where += "deleted_at IS null"
		}
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	items, err := query.Select(
		ctx,
		tx,
		VideoTableColumnsWithTypeCasts,
		VideoTable,
		where,
		orderBy,
		limit,
		offset,
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to call SelectVideos; err: %v", err)
	}

	objects := make([]*Video, 0)

	for _, item := range *items {
		object := &Video{}

		err = object.FromItem(item)
		if err != nil {
			return nil, err
		}

		thatCtx := ctx

		thatCtx, ok1 := query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("%s{%v}", VideoTable, object.GetPrimaryKeyValue()))
		thatCtx, ok2 := query.HandleQueryPathGraphCycles(thatCtx, fmt.Sprintf("__ReferencedBy__%s{%v}", VideoTable, object.GetPrimaryKeyValue()))
		if !(ok1 && ok2) {
			continue
		}

		_ = thatCtx

		if !types.IsZeroUUID(object.CameraID) {
			thisCtx := thatCtx
			thisCtx, ok1 := query.HandleQueryPathGraphCycles(thisCtx, fmt.Sprintf("%s{%v}", CameraTable, object.CameraID))
			thisCtx, ok2 := query.HandleQueryPathGraphCycles(thisCtx, fmt.Sprintf("__ReferencedBy__%s{%v}", CameraTable, object.CameraID))
			if ok1 && ok2 {
				object.CameraIDObject, err = SelectCamera(
					thisCtx,
					tx,
					fmt.Sprintf("%v = $1", CameraTablePrimaryKeyColumn),
					object.CameraID,
				)
				if err != nil {
					if !errors.Is(err, sql.ErrNoRows) {
						return nil, err
					}
				}
			}
		}

		err = func() error {
			thisCtx := thatCtx
			thisCtx, ok1 := query.HandleQueryPathGraphCycles(thisCtx, fmt.Sprintf("%s{%v}", VideoTable, object.GetPrimaryKeyValue()))
			thisCtx, ok2 := query.HandleQueryPathGraphCycles(thisCtx, fmt.Sprintf("__ReferencedBy__%s{%v}", VideoTable, object.GetPrimaryKeyValue()))

			if ok1 && ok2 {
				object.ReferencedByDetectionVideoIDObjects, err = SelectDetections(
					thisCtx,
					tx,
					fmt.Sprintf("%v = $1", DetectionTableVideoIDColumn),
					nil,
					nil,
					nil,
					object.GetPrimaryKeyValue(),
				)
				if err != nil {
					if !errors.Is(err, sql.ErrNoRows) {
						return err
					}
				}
			}

			return nil
		}()
		if err != nil {
			return nil, err
		}

		objects = append(objects, object)
	}

	return objects, nil
}

func SelectVideo(ctx context.Context, tx pgx.Tx, where string, values ...any) (*Video, error) {
	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	objects, err := SelectVideos(
		ctx,
		tx,
		where,
		nil,
		helpers.Ptr(2),
		helpers.Ptr(0),
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to call SelectVideo; err: %v", err)
	}

	if len(objects) > 1 {
		return nil, fmt.Errorf("attempt to call SelectVideo returned more than 1 row")
	}

	if len(objects) < 1 {
		return nil, sql.ErrNoRows
	}

	object := objects[0]

	return object, nil
}

func handleGetVideos(arguments *server.SelectManyArguments, db *pgxpool.Pool) ([]*Video, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	objects, err := SelectVideos(arguments.Ctx, tx, arguments.Where, arguments.OrderBy, arguments.Limit, arguments.Offset, arguments.Values...)
	if err != nil {
		return nil, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, err
	}

	return objects, nil
}

func handleGetVideo(arguments *server.SelectOneArguments, db *pgxpool.Pool, primaryKey uuid.UUID) ([]*Video, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	object, err := SelectVideo(arguments.Ctx, tx, arguments.Where, arguments.Values...)
	if err != nil {
		return nil, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, err
	}

	return []*Video{object}, nil
}

func handlePostVideos(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, objects []*Video, forceSetValuesForFieldsByObjectIndex [][]string) ([]*Video, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		return nil, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid: %v", err)
		return nil, err
	}
	_ = xid

	for i, object := range objects {
		err = object.Insert(arguments.Ctx, tx, false, false, forceSetValuesForFieldsByObjectIndex[i]...)
		if err != nil {
			err = fmt.Errorf("failed to insert %#+v; %v", object, err)
			return nil, err
		}

		objects[i] = object
	}

	errs := make(chan error, 1)
	go func() {
		_, err = waitForChange(arguments.Ctx, []stream.Action{stream.INSERT}, VideoTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change: %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		return nil, err
	}

	select {
	case <-arguments.Ctx.Done():
		err = fmt.Errorf("context canceled")
		return nil, err
	case err = <-errs:
		if err != nil {
			return nil, err
		}
	}

	return objects, nil
}

func handlePutVideo(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Video) ([]*Video, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		return nil, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid: %v", err)
		return nil, err
	}
	_ = xid

	err = object.Update(arguments.Ctx, tx, true)
	if err != nil {
		err = fmt.Errorf("failed to update %#+v; %v", object, err)
		return nil, err
	}

	errs := make(chan error, 1)
	go func() {
		_, err = waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, VideoTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change: %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		return nil, err
	}

	select {
	case <-arguments.Ctx.Done():
		err = fmt.Errorf("context canceled")
		return nil, err
	case err = <-errs:
		if err != nil {
			return nil, err
		}
	}

	return []*Video{object}, nil
}

func handlePatchVideo(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Video, forceSetValuesForFields []string) ([]*Video, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		return nil, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid: %v", err)
		return nil, err
	}
	_ = xid

	err = object.Update(arguments.Ctx, tx, false, forceSetValuesForFields...)
	if err != nil {
		err = fmt.Errorf("failed to update %#+v; %v", object, err)
		return nil, err
	}

	errs := make(chan error, 1)
	go func() {
		_, err = waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, VideoTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change: %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		return nil, err
	}

	select {
	case <-arguments.Ctx.Done():
		err = fmt.Errorf("context canceled")
		return nil, err
	case err = <-errs:
		if err != nil {
			return nil, err
		}
	}

	return []*Video{object}, nil
}

func handleDeleteVideo(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Video) error {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		return err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid: %v", err)
		return err
	}
	_ = xid

	err = object.Delete(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to delete %#+v; %v", object, err)
		return err
	}

	errs := make(chan error, 1)
	go func() {
		_, err = waitForChange(arguments.Ctx, []stream.Action{stream.DELETE, stream.SOFT_DELETE}, VideoTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change: %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		return err
	}

	select {
	case <-arguments.Ctx.Done():
		err = fmt.Errorf("context canceled")
		return err
	case err = <-errs:
		if err != nil {
			return err
		}
	}

	return nil
}

func GetVideoRouter(db *pgxpool.Pool, redisPool *redis.Pool, httpMiddlewares []server.HTTPMiddleware, objectMiddlewares []server.ObjectMiddleware, waitForChange server.WaitForChange) chi.Router {
	r := chi.NewRouter()

	for _, m := range httpMiddlewares {
		r.Use(m)
	}

	getManyHandler, err := server.GetCustomHTTPHandler(
		http.MethodGet,
		"/",
		http.StatusOK,
		func(
			ctx context.Context,
			pathParams server.EmptyPathParams,
			queryParams map[string]any,
			req server.EmptyRequest,
			rawReq any,
		) (*helpers.TypedResponse[Video], error) {
			redisConn := redisPool.Get()
			defer func() {
				_ = redisConn.Close()
			}()

			arguments, err := server.GetSelectManyArguments(ctx, queryParams, VideoIntrospectedTable, nil, nil)
			if err != nil {
				return nil, err
			}

			cachedObjectsAsJSON, cacheHit, err := helpers.GetCachedObjectsAsJSON(arguments.RequestHash, redisConn)
			if err != nil {
				return nil, err
			}

			if cacheHit {
				var cachedObjects []*Video
				err = json.Unmarshal(cachedObjectsAsJSON, &cachedObjects)
				if err != nil {
					return nil, err
				}

				return &helpers.TypedResponse[Video]{
					Status:  http.StatusOK,
					Success: true,
					Error:   nil,
					Objects: cachedObjects,
				}, nil
			}

			objects, err := handleGetVideos(arguments, db)
			if err != nil {
				return nil, err
			}

			objectsAsJSON, err := json.Marshal(objects)
			if err != nil {
				return nil, err
			}

			err = helpers.StoreCachedResponse(arguments.RequestHash, redisConn, string(objectsAsJSON))
			if err != nil {
				log.Printf("warning: %v", err)
			}

			return &helpers.TypedResponse[Video]{
				Status:  http.StatusOK,
				Success: true,
				Error:   nil,
				Objects: objects,
			}, nil
		},
	)
	if err != nil {
		panic(err)
	}
	r.Get("/", getManyHandler.ServeHTTP)

	getOneHandler, err := server.GetCustomHTTPHandler(
		http.MethodGet,
		"/{primaryKey}",
		http.StatusOK,
		func(
			ctx context.Context,
			pathParams VideoOnePathParams,
			queryParams VideoLoadQueryParams,
			req server.EmptyRequest,
			rawReq any,
		) (*helpers.TypedResponse[Video], error) {
			redisConn := redisPool.Get()
			defer func() {
				_ = redisConn.Close()
			}()

			arguments, err := server.GetSelectOneArguments(ctx, queryParams.Depth, VideoIntrospectedTable, pathParams.PrimaryKey, nil, nil)
			if err != nil {
				return nil, err
			}

			cachedObjectsAsJSON, cacheHit, err := helpers.GetCachedObjectsAsJSON(arguments.RequestHash, redisConn)
			if err != nil {
				return nil, err
			}

			if cacheHit {
				var cachedObjects []*Video
				err = json.Unmarshal(cachedObjectsAsJSON, &cachedObjects)
				if err != nil {
					return nil, err
				}

				return &helpers.TypedResponse[Video]{
					Status:  http.StatusOK,
					Success: true,
					Error:   nil,
					Objects: cachedObjects,
				}, nil
			}

			objects, err := handleGetVideo(arguments, db, pathParams.PrimaryKey)
			if err != nil {
				return nil, err
			}

			objectsAsJSON, err := json.Marshal(objects)
			if err != nil {
				return nil, err
			}

			err = helpers.StoreCachedResponse(arguments.RequestHash, redisConn, string(objectsAsJSON))
			if err != nil {
				log.Printf("warning: %v", err)
			}

			return &helpers.TypedResponse[Video]{
				Status:  http.StatusOK,
				Success: true,
				Error:   nil,
				Objects: objects,
			}, nil
		},
	)
	if err != nil {
		panic(err)
	}
	r.Get("/{primaryKey}", getOneHandler.ServeHTTP)

	postHandler, err := server.GetCustomHTTPHandler(
		http.MethodPost,
		"/",
		http.StatusCreated,
		func(
			ctx context.Context,
			pathParams server.EmptyPathParams,
			queryParams VideoLoadQueryParams,
			req []*Video,
			rawReq any,
		) (*helpers.TypedResponse[Video], error) {
			allRawItems, ok := rawReq.([]any)
			if !ok {
				return nil, fmt.Errorf("failed to cast %#+v to []map[string]any", rawReq)
			}

			allItems := make([]map[string]any, 0)
			for _, rawItem := range allRawItems {
				item, ok := rawItem.(map[string]any)
				if !ok {
					return nil, fmt.Errorf("failed to cast %#+v to map[string]any", rawItem)
				}

				allItems = append(allItems, item)
			}

			forceSetValuesForFieldsByObjectIndex := make([][]string, 0)
			for _, item := range allItems {
				forceSetValuesForFields := make([]string, 0)
				for _, possibleField := range maps.Keys(item) {
					if !slices.Contains(VideoTableColumns, possibleField) {
						continue
					}

					forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
				}
				forceSetValuesForFieldsByObjectIndex = append(forceSetValuesForFieldsByObjectIndex, forceSetValuesForFields)
			}

			arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
			if err != nil {
				return nil, err
			}

			objects, err := handlePostVideos(arguments, db, waitForChange, req, forceSetValuesForFieldsByObjectIndex)
			if err != nil {
				return nil, err
			}

			return &helpers.TypedResponse[Video]{
				Status:  http.StatusCreated,
				Success: true,
				Error:   nil,
				Objects: objects,
			}, nil
		},
	)
	if err != nil {
		panic(err)
	}
	r.Post("/", postHandler.ServeHTTP)

	putHandler, err := server.GetCustomHTTPHandler(
		http.MethodPatch,
		"/{primaryKey}",
		http.StatusOK,
		func(
			ctx context.Context,
			pathParams VideoOnePathParams,
			queryParams VideoLoadQueryParams,
			req Video,
			rawReq any,
		) (*helpers.TypedResponse[Video], error) {
			item, ok := rawReq.(map[string]any)
			if !ok {
				return nil, fmt.Errorf("failed to cast %#+v to map[string]any", item)
			}

			arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
			if err != nil {
				return nil, err
			}

			object := &req
			object.ID = pathParams.PrimaryKey

			objects, err := handlePutVideo(arguments, db, waitForChange, object)
			if err != nil {
				return nil, err
			}

			return &helpers.TypedResponse[Video]{
				Status:  http.StatusOK,
				Success: true,
				Error:   nil,
				Objects: objects,
			}, nil
		},
	)
	if err != nil {
		panic(err)
	}
	r.Put("/{primaryKey}", putHandler.ServeHTTP)

	patchHandler, err := server.GetCustomHTTPHandler(
		http.MethodPatch,
		"/{primaryKey}",
		http.StatusOK,
		func(
			ctx context.Context,
			pathParams VideoOnePathParams,
			queryParams VideoLoadQueryParams,
			req Video,
			rawReq any,
		) (*helpers.TypedResponse[Video], error) {
			item, ok := rawReq.(map[string]any)
			if !ok {
				return nil, fmt.Errorf("failed to cast %#+v to map[string]any", item)
			}

			forceSetValuesForFields := make([]string, 0)
			for _, possibleField := range maps.Keys(item) {
				if !slices.Contains(VideoTableColumns, possibleField) {
					continue
				}

				forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
			}

			arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
			if err != nil {
				return nil, err
			}

			object := &req
			object.ID = pathParams.PrimaryKey

			objects, err := handlePatchVideo(arguments, db, waitForChange, object, forceSetValuesForFields)
			if err != nil {
				return nil, err
			}

			return &helpers.TypedResponse[Video]{
				Status:  http.StatusOK,
				Success: true,
				Error:   nil,
				Objects: objects,
			}, nil
		},
	)
	if err != nil {
		panic(err)
	}
	r.Patch("/{primaryKey}", patchHandler.ServeHTTP)

	deleteHandler, err := server.GetCustomHTTPHandler(
		http.MethodDelete,
		"/{primaryKey}",
		http.StatusNoContent,
		func(
			ctx context.Context,
			pathParams VideoOnePathParams,
			queryParams VideoLoadQueryParams,
			req server.EmptyRequest,
			rawReq any,
		) (*server.EmptyResponse, error) {
			arguments := &server.LoadArguments{
				Ctx: ctx,
			}

			object := &Video{}
			object.ID = pathParams.PrimaryKey

			err := handleDeleteVideo(arguments, db, waitForChange, object)
			if err != nil {
				return nil, err
			}

			return nil, nil
		},
	)
	if err != nil {
		panic(err)
	}
	r.Delete("/{primaryKey}", deleteHandler.ServeHTTP)

	return r
}

func NewVideoFromItem(item map[string]any) (any, error) {
	object := &Video{}

	err := object.FromItem(item)
	if err != nil {
		return nil, err
	}

	return object, nil
}

func init() {
	register(
		VideoTable,
		Video{},
		NewVideoFromItem,
		"/videos",
		GetVideoRouter,
	)
}
