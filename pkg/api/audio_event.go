package api

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"net/http"
	"net/netip"
	"slices"
	"sort"
	"strings"
	"time"

	"github.com/cridenour/go-postgis"
	"github.com/go-chi/chi/v5"
	"github.com/gomodule/redigo/redis"
	"github.com/google/uuid"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/initialed85/djangolang/pkg/helpers"
	"github.com/initialed85/djangolang/pkg/introspect"
	"github.com/initialed85/djangolang/pkg/query"
	"github.com/initialed85/djangolang/pkg/server"
	"github.com/initialed85/djangolang/pkg/stream"
	"github.com/initialed85/djangolang/pkg/types"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/jackc/pgx/v5/pgxpool"
)

type AudioEvent struct {
	ID             uuid.UUID  `json:"id"`
	CreatedAt      time.Time  `json:"created_at"`
	UpdatedAt      time.Time  `json:"updated_at"`
	DeletedAt      *time.Time `json:"deleted_at"`
	CameraID       uuid.UUID  `json:"camera_id"`
	CameraIDObject *Camera    `json:"camera_id_object"`
	VideoID        uuid.UUID  `json:"video_id"`
	VideoIDObject  *Video     `json:"video_id_object"`
	SeenAt         time.Time  `json:"seen_at"`
	OffsetMs       int64      `json:"offset_ms"`
	EndedAt        time.Time  `json:"ended_at"`
	DurationMs     int64      `json:"duration_ms"`
	Kind           string     `json:"kind"`
	RmsDbfs        float64    `json:"rms_dbfs"`
	PeakDbfs       float64    `json:"peak_dbfs"`
	FloorDbfs      float64    `json:"floor_dbfs"`
	ProminenceDb   float64    `json:"prominence_db"`
	CrestDb        float64    `json:"crest_db"`
	Score          float64    `json:"score"`
	Spectral       any        `json:"spectral"`
}

var AudioEventTable = "audio_event"

var AudioEventTableWithSchema = fmt.Sprintf("%s.%s", schema, AudioEventTable)

var AudioEventTableNamespaceID int32 = 1337 + 1

var (
	AudioEventTableIDColumn           = "id"
	AudioEventTableCreatedAtColumn    = "created_at"
	AudioEventTableUpdatedAtColumn    = "updated_at"
	AudioEventTableDeletedAtColumn    = "deleted_at"
	AudioEventTableCameraIDColumn     = "camera_id"
	AudioEventTableVideoIDColumn      = "video_id"
	AudioEventTableSeenAtColumn       = "seen_at"
	AudioEventTableOffsetMsColumn     = "offset_ms"
	AudioEventTableEndedAtColumn      = "ended_at"
	AudioEventTableDurationMsColumn   = "duration_ms"
	AudioEventTableKindColumn         = "kind"
	AudioEventTableRmsDbfsColumn      = "rms_dbfs"
	AudioEventTablePeakDbfsColumn     = "peak_dbfs"
	AudioEventTableFloorDbfsColumn    = "floor_dbfs"
	AudioEventTableProminenceDbColumn = "prominence_db"
	AudioEventTableCrestDbColumn      = "crest_db"
	AudioEventTableScoreColumn        = "score"
	AudioEventTableSpectralColumn     = "spectral"
)

var (
	AudioEventTableIDColumnWithTypeCast           = `"id" AS id`
	AudioEventTableCreatedAtColumnWithTypeCast    = `"created_at" AS created_at`
	AudioEventTableUpdatedAtColumnWithTypeCast    = `"updated_at" AS updated_at`
	AudioEventTableDeletedAtColumnWithTypeCast    = `"deleted_at" AS deleted_at`
	AudioEventTableCameraIDColumnWithTypeCast     = `"camera_id" AS camera_id`
	AudioEventTableVideoIDColumnWithTypeCast      = `"video_id" AS video_id`
	AudioEventTableSeenAtColumnWithTypeCast       = `"seen_at" AS seen_at`
	AudioEventTableOffsetMsColumnWithTypeCast     = `"offset_ms" AS offset_ms`
	AudioEventTableEndedAtColumnWithTypeCast      = `"ended_at" AS ended_at`
	AudioEventTableDurationMsColumnWithTypeCast   = `"duration_ms" AS duration_ms`
	AudioEventTableKindColumnWithTypeCast         = `"kind" AS kind`
	AudioEventTableRmsDbfsColumnWithTypeCast      = `"rms_dbfs" AS rms_dbfs`
	AudioEventTablePeakDbfsColumnWithTypeCast     = `"peak_dbfs" AS peak_dbfs`
	AudioEventTableFloorDbfsColumnWithTypeCast    = `"floor_dbfs" AS floor_dbfs`
	AudioEventTableProminenceDbColumnWithTypeCast = `"prominence_db" AS prominence_db`
	AudioEventTableCrestDbColumnWithTypeCast      = `"crest_db" AS crest_db`
	AudioEventTableScoreColumnWithTypeCast        = `"score" AS score`
	AudioEventTableSpectralColumnWithTypeCast     = `"spectral" AS spectral`
)

var AudioEventTableColumns = []string{
	AudioEventTableIDColumn,
	AudioEventTableCreatedAtColumn,
	AudioEventTableUpdatedAtColumn,
	AudioEventTableDeletedAtColumn,
	AudioEventTableCameraIDColumn,
	AudioEventTableVideoIDColumn,
	AudioEventTableSeenAtColumn,
	AudioEventTableOffsetMsColumn,
	AudioEventTableEndedAtColumn,
	AudioEventTableDurationMsColumn,
	AudioEventTableKindColumn,
	AudioEventTableRmsDbfsColumn,
	AudioEventTablePeakDbfsColumn,
	AudioEventTableFloorDbfsColumn,
	AudioEventTableProminenceDbColumn,
	AudioEventTableCrestDbColumn,
	AudioEventTableScoreColumn,
	AudioEventTableSpectralColumn,
}

var AudioEventTableColumnsWithTypeCasts = []string{
	AudioEventTableIDColumnWithTypeCast,
	AudioEventTableCreatedAtColumnWithTypeCast,
	AudioEventTableUpdatedAtColumnWithTypeCast,
	AudioEventTableDeletedAtColumnWithTypeCast,
	AudioEventTableCameraIDColumnWithTypeCast,
	AudioEventTableVideoIDColumnWithTypeCast,
	AudioEventTableSeenAtColumnWithTypeCast,
	AudioEventTableOffsetMsColumnWithTypeCast,
	AudioEventTableEndedAtColumnWithTypeCast,
	AudioEventTableDurationMsColumnWithTypeCast,
	AudioEventTableKindColumnWithTypeCast,
	AudioEventTableRmsDbfsColumnWithTypeCast,
	AudioEventTablePeakDbfsColumnWithTypeCast,
	AudioEventTableFloorDbfsColumnWithTypeCast,
	AudioEventTableProminenceDbColumnWithTypeCast,
	AudioEventTableCrestDbColumnWithTypeCast,
	AudioEventTableScoreColumnWithTypeCast,
	AudioEventTableSpectralColumnWithTypeCast,
}

var AudioEventIntrospectedTable *introspect.Table

var AudioEventTableColumnLookup map[string]*introspect.Column

var (
	AudioEventTablePrimaryKeyColumn = AudioEventTableIDColumn
)

func init() {
	AudioEventIntrospectedTable = tableByName[AudioEventTable]

	/* only needed during templating */
	if AudioEventIntrospectedTable == nil {
		AudioEventIntrospectedTable = &introspect.Table{}
	}

	AudioEventTableColumnLookup = AudioEventIntrospectedTable.ColumnByName
}

type AudioEventOnePathParams struct {
	PrimaryKey uuid.UUID `json:"primaryKey"`
}

type AudioEventLoadQueryParams struct {
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

func (m *AudioEvent) GetPrimaryKeyColumn() string {
	return AudioEventTablePrimaryKeyColumn
}

func (m *AudioEvent) GetPrimaryKeyValue() any {
	return m.ID
}
func (AudioEvent) GetQueryInfo() query.ModelQueryInfo {
	return query.ModelQueryInfo{
		TableName:        "audio_event",
		PrimaryKeyColumn: "id",
		Relationships: []query.RelationshipDefinition{
			{
				Name:         "camera_id",
				Direction:    query.RelationDirectionOutbound,
				TargetTable:  "camera",
				LocalColumn:  "camera_id",
				TargetColumn: "id",
				Cardinality:  query.RelationCardinalityOne,
			},
			{
				Name:         "video_id",
				Direction:    query.RelationDirectionOutbound,
				TargetTable:  "video",
				LocalColumn:  "video_id",
				TargetColumn: "id",
				Cardinality:  query.RelationCardinalityOne,
			},
		},
		Filters: []query.FilterDefinition{
			{
				Name:     "created_at",
				DataType: "timestamp with time zone",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "updated_at",
				DataType: "timestamp with time zone",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "deleted_at",
				DataType: "timestamp with time zone",
				Nullable: true,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpIsNull,
					query.FilterOpIsNotNull,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "camera_id",
				DataType: "uuid",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "video_id",
				DataType: "uuid",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "seen_at",
				DataType: "timestamp with time zone",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "offset_ms",
				DataType: "integer",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "ended_at",
				DataType: "timestamp with time zone",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "duration_ms",
				DataType: "integer",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "kind",
				DataType: "text",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpLike,
					query.FilterOpNotLike,
					query.FilterOpILike,
					query.FilterOpNotILike,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "rms_dbfs",
				DataType: "double precision",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "peak_dbfs",
				DataType: "double precision",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "floor_dbfs",
				DataType: "double precision",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "prominence_db",
				DataType: "double precision",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "crest_db",
				DataType: "double precision",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "score",
				DataType: "double precision",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpGt,
					query.FilterOpGte,
					query.FilterOpLt,
					query.FilterOpLte,
					query.FilterOpIn,
					query.FilterOpNotIn,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "spectral",
				DataType: "jsonb",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpContains,
					query.FilterOpNotContains,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
		},
	}
}

func (m *AudioEvent) FromItem(item map[string]any) error {
	if item == nil {
		return fmt.Errorf(
			"item unexpectedly nil during AudioEventFromItem",
		)
	}

	if len(item) == 0 {
		return fmt.Errorf(
			"item unexpectedly empty during AudioEventFromItem",
		)
	}

	wrapError := func(k string, v any, err error) error {
		return fmt.Errorf("%v: %#+v; error; %v", k, v, err)
	}

	for k, v := range item {
		_, ok := AudioEventTableColumnLookup[k]
		if !ok {
			return fmt.Errorf(
				"item contained unexpected key %#+v during AudioEventFromItem; item: %#+v",
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

		case "video_id":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuvideo_id.UUID", temp1))
				}
			}

			m.VideoID = temp2

		case "seen_at":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuseen_at.UUID", temp1))
				}
			}

			m.SeenAt = temp2

		case "offset_ms":
			if v == nil {
				continue
			}

			temp1, err := types.ParseInt(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(int64)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuoffset_ms.UUID", temp1))
				}
			}

			m.OffsetMs = temp2

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

			m.EndedAt = temp2

		case "duration_ms":
			if v == nil {
				continue
			}

			temp1, err := types.ParseInt(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(int64)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuduration_ms.UUID", temp1))
				}
			}

			m.DurationMs = temp2

		case "kind":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uukind.UUID", temp1))
				}
			}

			m.Kind = temp2

		case "rms_dbfs":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uurms_dbfs.UUID", temp1))
				}
			}

			m.RmsDbfs = temp2

		case "peak_dbfs":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uupeak_dbfs.UUID", temp1))
				}
			}

			m.PeakDbfs = temp2

		case "floor_dbfs":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uufloor_dbfs.UUID", temp1))
				}
			}

			m.FloorDbfs = temp2

		case "prominence_db":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuprominence_db.UUID", temp1))
				}
			}

			m.ProminenceDb = temp2

		case "crest_db":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uucrest_db.UUID", temp1))
				}
			}

			m.CrestDb = temp2

		case "score":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuscore.UUID", temp1))
				}
			}

			m.Score = temp2

		case "spectral":
			if v == nil {
				continue
			}

			temp1, err := types.ParseJSON(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1, true
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuspectral.UUID", temp1))
				}
			}

			m.Spectral = temp2

		}
	}

	return nil
}

func (m *AudioEvent) ToItem() map[string]any {
	item := make(map[string]any)

	b, err := json.Marshal(m)
	if err != nil {
		panic(fmt.Sprintf("%T.ToItem() failed intermediate marshal to JSON: %s", m, err))
	}

	err = json.Unmarshal(b, &item)
	if err != nil {
		panic(fmt.Sprintf("%T.ToItem() failed intermediate unmarshal from JSON: %s", m, err))
	}

	return item
}

func (m *AudioEvent) Reload(ctx context.Context, tx pgx.Tx, includeDeleteds ...bool) error {
	extraWhere := ""
	if len(includeDeleteds) > 0 && includeDeleteds[0] {
		if slices.Contains(AudioEventTableColumns, "deleted_at") {
			extraWhere = "\n    AND (deleted_at IS null OR deleted_at IS NOT null)"
		}
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	o, _, _, _, _, err := SelectAudioEvent(
		ctx,
		tx,
		fmt.Sprintf("%v = $1%v", m.GetPrimaryKeyColumn(), extraWhere),
		m.GetPrimaryKeyValue(),
	)
	if err != nil {
		return err
	}

	m.ID = o.ID
	m.CreatedAt = o.CreatedAt
	m.UpdatedAt = o.UpdatedAt
	m.DeletedAt = o.DeletedAt
	m.CameraID = o.CameraID
	m.CameraIDObject = o.CameraIDObject
	m.VideoID = o.VideoID
	m.VideoIDObject = o.VideoIDObject
	m.SeenAt = o.SeenAt
	m.OffsetMs = o.OffsetMs
	m.EndedAt = o.EndedAt
	m.DurationMs = o.DurationMs
	m.Kind = o.Kind
	m.RmsDbfs = o.RmsDbfs
	m.PeakDbfs = o.PeakDbfs
	m.FloorDbfs = o.FloorDbfs
	m.ProminenceDb = o.ProminenceDb
	m.CrestDb = o.CrestDb
	m.Score = o.Score
	m.Spectral = o.Spectral

	return nil
}

func (m *AudioEvent) GetColumnsAndValues(setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) ([]string, []any, error) {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setPrimaryKey && (setZeroValues || !types.IsZeroUUID(m.ID) || slices.Contains(forceSetValuesForFields, AudioEventTableIDColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableIDColumn)) {
		columns = append(columns, AudioEventTableIDColumn)

		v, err := types.FormatUUID(m.ID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.ID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableCreatedAtColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableCreatedAtColumn) {
		columns = append(columns, AudioEventTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.CreatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableUpdatedAtColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableUpdatedAtColumn) {
		columns = append(columns, AudioEventTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.UpdatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableDeletedAtColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableDeletedAtColumn) {
		columns = append(columns, AudioEventTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.DeletedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) || slices.Contains(forceSetValuesForFields, AudioEventTableCameraIDColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableCameraIDColumn) {
		columns = append(columns, AudioEventTableCameraIDColumn)

		v, err := types.FormatUUID(m.CameraID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.CameraID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.VideoID) || slices.Contains(forceSetValuesForFields, AudioEventTableVideoIDColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableVideoIDColumn) {
		columns = append(columns, AudioEventTableVideoIDColumn)

		v, err := types.FormatUUID(m.VideoID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.VideoID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.SeenAt) || slices.Contains(forceSetValuesForFields, AudioEventTableSeenAtColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableSeenAtColumn) {
		columns = append(columns, AudioEventTableSeenAtColumn)

		v, err := types.FormatTime(m.SeenAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.SeenAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.OffsetMs) || slices.Contains(forceSetValuesForFields, AudioEventTableOffsetMsColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableOffsetMsColumn) {
		columns = append(columns, AudioEventTableOffsetMsColumn)

		v, err := types.FormatInt(m.OffsetMs)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.OffsetMs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.EndedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableEndedAtColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableEndedAtColumn) {
		columns = append(columns, AudioEventTableEndedAtColumn)

		v, err := types.FormatTime(m.EndedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.EndedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.DurationMs) || slices.Contains(forceSetValuesForFields, AudioEventTableDurationMsColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableDurationMsColumn) {
		columns = append(columns, AudioEventTableDurationMsColumn)

		v, err := types.FormatInt(m.DurationMs)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.DurationMs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Kind) || slices.Contains(forceSetValuesForFields, AudioEventTableKindColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableKindColumn) {
		columns = append(columns, AudioEventTableKindColumn)

		v, err := types.FormatString(m.Kind)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Kind; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.RmsDbfs) || slices.Contains(forceSetValuesForFields, AudioEventTableRmsDbfsColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableRmsDbfsColumn) {
		columns = append(columns, AudioEventTableRmsDbfsColumn)

		v, err := types.FormatFloat(m.RmsDbfs)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.RmsDbfs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.PeakDbfs) || slices.Contains(forceSetValuesForFields, AudioEventTablePeakDbfsColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTablePeakDbfsColumn) {
		columns = append(columns, AudioEventTablePeakDbfsColumn)

		v, err := types.FormatFloat(m.PeakDbfs)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.PeakDbfs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.FloorDbfs) || slices.Contains(forceSetValuesForFields, AudioEventTableFloorDbfsColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableFloorDbfsColumn) {
		columns = append(columns, AudioEventTableFloorDbfsColumn)

		v, err := types.FormatFloat(m.FloorDbfs)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.FloorDbfs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.ProminenceDb) || slices.Contains(forceSetValuesForFields, AudioEventTableProminenceDbColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableProminenceDbColumn) {
		columns = append(columns, AudioEventTableProminenceDbColumn)

		v, err := types.FormatFloat(m.ProminenceDb)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.ProminenceDb; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.CrestDb) || slices.Contains(forceSetValuesForFields, AudioEventTableCrestDbColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableCrestDbColumn) {
		columns = append(columns, AudioEventTableCrestDbColumn)

		v, err := types.FormatFloat(m.CrestDb)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.CrestDb; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.Score) || slices.Contains(forceSetValuesForFields, AudioEventTableScoreColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableScoreColumn) {
		columns = append(columns, AudioEventTableScoreColumn)

		v, err := types.FormatFloat(m.Score)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Score; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroJSON(m.Spectral) || slices.Contains(forceSetValuesForFields, AudioEventTableSpectralColumn) || isRequired(AudioEventTableColumnLookup, AudioEventTableSpectralColumn) {
		columns = append(columns, AudioEventTableSpectralColumn)

		v, err := types.FormatJSON(m.Spectral)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Spectral; %v", err)
		}

		values = append(values, v)
	}

	return columns, values, nil
}

func (m *AudioEvent) Insert(ctx context.Context, tx pgx.Tx, setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) error {
	columns, values, err := m.GetColumnsAndValues(setPrimaryKey, setZeroValues, forceSetValuesForFields...)
	if err != nil {
		return fmt.Errorf("failed to get columns and values to insert %#+v; %v", m, err)
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	item, err := query.Insert(
		ctx,
		tx,
		AudioEventTableWithSchema,
		columns,
		nil,
		nil,
		nil,
		false,
		false,
		AudioEventTableColumns,
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to insert %#+v; %v", m, err)
	}
	v := (*item)[AudioEventTableIDColumn]

	if v == nil {
		return fmt.Errorf("failed to find %v in %#+v", AudioEventTableIDColumn, item)
	}

	wrapError := func(err error) error {
		return fmt.Errorf(
			"failed to treat %v: %#+v as uuid.UUID: %v",
			AudioEventTableIDColumn,
			(*item)[AudioEventTableIDColumn],
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
		return fmt.Errorf("failed to reload after insert; %v", err)
	}

	return nil
}

func (m *AudioEvent) Update(ctx context.Context, tx pgx.Tx, setZeroValues bool, forceSetValuesForFields ...string) error {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableCreatedAtColumn) {
		columns = append(columns, AudioEventTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.CreatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableUpdatedAtColumn) {
		columns = append(columns, AudioEventTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.UpdatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableDeletedAtColumn) {
		columns = append(columns, AudioEventTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.DeletedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) || slices.Contains(forceSetValuesForFields, AudioEventTableCameraIDColumn) {
		columns = append(columns, AudioEventTableCameraIDColumn)

		v, err := types.FormatUUID(m.CameraID)
		if err != nil {
			return fmt.Errorf("failed to handle m.CameraID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.VideoID) || slices.Contains(forceSetValuesForFields, AudioEventTableVideoIDColumn) {
		columns = append(columns, AudioEventTableVideoIDColumn)

		v, err := types.FormatUUID(m.VideoID)
		if err != nil {
			return fmt.Errorf("failed to handle m.VideoID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.SeenAt) || slices.Contains(forceSetValuesForFields, AudioEventTableSeenAtColumn) {
		columns = append(columns, AudioEventTableSeenAtColumn)

		v, err := types.FormatTime(m.SeenAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.SeenAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.OffsetMs) || slices.Contains(forceSetValuesForFields, AudioEventTableOffsetMsColumn) {
		columns = append(columns, AudioEventTableOffsetMsColumn)

		v, err := types.FormatInt(m.OffsetMs)
		if err != nil {
			return fmt.Errorf("failed to handle m.OffsetMs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.EndedAt) || slices.Contains(forceSetValuesForFields, AudioEventTableEndedAtColumn) {
		columns = append(columns, AudioEventTableEndedAtColumn)

		v, err := types.FormatTime(m.EndedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.EndedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.DurationMs) || slices.Contains(forceSetValuesForFields, AudioEventTableDurationMsColumn) {
		columns = append(columns, AudioEventTableDurationMsColumn)

		v, err := types.FormatInt(m.DurationMs)
		if err != nil {
			return fmt.Errorf("failed to handle m.DurationMs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Kind) || slices.Contains(forceSetValuesForFields, AudioEventTableKindColumn) {
		columns = append(columns, AudioEventTableKindColumn)

		v, err := types.FormatString(m.Kind)
		if err != nil {
			return fmt.Errorf("failed to handle m.Kind; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.RmsDbfs) || slices.Contains(forceSetValuesForFields, AudioEventTableRmsDbfsColumn) {
		columns = append(columns, AudioEventTableRmsDbfsColumn)

		v, err := types.FormatFloat(m.RmsDbfs)
		if err != nil {
			return fmt.Errorf("failed to handle m.RmsDbfs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.PeakDbfs) || slices.Contains(forceSetValuesForFields, AudioEventTablePeakDbfsColumn) {
		columns = append(columns, AudioEventTablePeakDbfsColumn)

		v, err := types.FormatFloat(m.PeakDbfs)
		if err != nil {
			return fmt.Errorf("failed to handle m.PeakDbfs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.FloorDbfs) || slices.Contains(forceSetValuesForFields, AudioEventTableFloorDbfsColumn) {
		columns = append(columns, AudioEventTableFloorDbfsColumn)

		v, err := types.FormatFloat(m.FloorDbfs)
		if err != nil {
			return fmt.Errorf("failed to handle m.FloorDbfs; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.ProminenceDb) || slices.Contains(forceSetValuesForFields, AudioEventTableProminenceDbColumn) {
		columns = append(columns, AudioEventTableProminenceDbColumn)

		v, err := types.FormatFloat(m.ProminenceDb)
		if err != nil {
			return fmt.Errorf("failed to handle m.ProminenceDb; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.CrestDb) || slices.Contains(forceSetValuesForFields, AudioEventTableCrestDbColumn) {
		columns = append(columns, AudioEventTableCrestDbColumn)

		v, err := types.FormatFloat(m.CrestDb)
		if err != nil {
			return fmt.Errorf("failed to handle m.CrestDb; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.Score) || slices.Contains(forceSetValuesForFields, AudioEventTableScoreColumn) {
		columns = append(columns, AudioEventTableScoreColumn)

		v, err := types.FormatFloat(m.Score)
		if err != nil {
			return fmt.Errorf("failed to handle m.Score; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroJSON(m.Spectral) || slices.Contains(forceSetValuesForFields, AudioEventTableSpectralColumn) {
		columns = append(columns, AudioEventTableSpectralColumn)

		v, err := types.FormatJSON(m.Spectral)
		if err != nil {
			return fmt.Errorf("failed to handle m.Spectral; %v", err)
		}

		values = append(values, v)
	}

	v, err := types.FormatUUID(m.ID)
	if err != nil {
		return fmt.Errorf("failed to handle m.ID; %v", err)
	}

	values = append(values, v)

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	_, err = query.Update(
		ctx,
		tx,
		AudioEventTableWithSchema,
		columns,
		fmt.Sprintf("%v = $$??", AudioEventTableIDColumn),
		AudioEventTableColumns,
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
func (m *AudioEvent) UpdateField(ctx context.Context, tx pgx.Tx, fieldName string, value any) error {
	var columnName string
	switch fieldName {

	case "id":
		columnName = AudioEventTableIDColumn

	case "created_at":
		columnName = AudioEventTableCreatedAtColumn

	case "updated_at":
		columnName = AudioEventTableUpdatedAtColumn

	case "deleted_at":
		columnName = AudioEventTableDeletedAtColumn

	case "camera_id":
		columnName = AudioEventTableCameraIDColumn

	case "video_id":
		columnName = AudioEventTableVideoIDColumn

	case "seen_at":
		columnName = AudioEventTableSeenAtColumn

	case "offset_ms":
		columnName = AudioEventTableOffsetMsColumn

	case "ended_at":
		columnName = AudioEventTableEndedAtColumn

	case "duration_ms":
		columnName = AudioEventTableDurationMsColumn

	case "kind":
		columnName = AudioEventTableKindColumn

	case "rms_dbfs":
		columnName = AudioEventTableRmsDbfsColumn

	case "peak_dbfs":
		columnName = AudioEventTablePeakDbfsColumn

	case "floor_dbfs":
		columnName = AudioEventTableFloorDbfsColumn

	case "prominence_db":
		columnName = AudioEventTableProminenceDbColumn

	case "crest_db":
		columnName = AudioEventTableCrestDbColumn

	case "score":
		columnName = AudioEventTableScoreColumn

	case "spectral":
		columnName = AudioEventTableSpectralColumn
	default:
		return fmt.Errorf("unknown field name: %v", fieldName)
	}

	var columnValue any
	var err error
	switch columnName {

	case AudioEventTableIDColumn:
		columnValue, err = types.FormatUUID(value)

	case AudioEventTableCreatedAtColumn:
		columnValue, err = types.FormatTime(value)

	case AudioEventTableUpdatedAtColumn:
		columnValue, err = types.FormatTime(value)

	case AudioEventTableDeletedAtColumn:
		columnValue, err = types.FormatTime(value)

	case AudioEventTableCameraIDColumn:
		columnValue, err = types.FormatUUID(value)

	case AudioEventTableVideoIDColumn:
		columnValue, err = types.FormatUUID(value)

	case AudioEventTableSeenAtColumn:
		columnValue, err = types.FormatTime(value)

	case AudioEventTableOffsetMsColumn:
		columnValue, err = types.FormatInt(value)

	case AudioEventTableEndedAtColumn:
		columnValue, err = types.FormatTime(value)

	case AudioEventTableDurationMsColumn:
		columnValue, err = types.FormatInt(value)

	case AudioEventTableKindColumn:
		columnValue, err = types.FormatString(value)

	case AudioEventTableRmsDbfsColumn:
		columnValue, err = types.FormatFloat(value)

	case AudioEventTablePeakDbfsColumn:
		columnValue, err = types.FormatFloat(value)

	case AudioEventTableFloorDbfsColumn:
		columnValue, err = types.FormatFloat(value)

	case AudioEventTableProminenceDbColumn:
		columnValue, err = types.FormatFloat(value)

	case AudioEventTableCrestDbColumn:
		columnValue, err = types.FormatFloat(value)

	case AudioEventTableScoreColumn:
		columnValue, err = types.FormatFloat(value)

	case AudioEventTableSpectralColumn:
		columnValue, err = types.FormatJSON(value)
	}
	if err != nil {
		return fmt.Errorf("failed to format value for %v; %v", columnName, err)
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	_, err = query.Update(
		ctx,
		tx,
		AudioEventTableWithSchema,
		[]string{columnName},
		fmt.Sprintf("%v = $$??", AudioEventTablePrimaryKeyColumn),
		[]string{AudioEventTablePrimaryKeyColumn},
		columnValue,
		m.GetPrimaryKeyValue(),
	)
	if err != nil {
		return fmt.Errorf("failed to update field %v: %v", fieldName, err)
	}

	err = m.Reload(ctx, tx, false)
	if err != nil {
		return fmt.Errorf("failed to reload after update")
	}

	return nil
}

func (m *AudioEvent) UpdateFields(ctx context.Context, tx pgx.Tx, fields map[string]any) error {
	if len(fields) == 0 {
		return nil
	}
	fieldNames := make([]string, 0, len(fields))
	for fieldName := range fields {
		fieldNames = append(fieldNames, fieldName)
	}
	sort.Strings(fieldNames)

	columns := make([]string, 0, len(fields))
	values := make([]any, 0, len(fields)*2)

	for _, fieldName := range fieldNames {
		value := fields[fieldName]
		var columnName string
		switch fieldName {

		case "id":
			columnName = AudioEventTableIDColumn

		case "created_at":
			columnName = AudioEventTableCreatedAtColumn

		case "updated_at":
			columnName = AudioEventTableUpdatedAtColumn

		case "deleted_at":
			columnName = AudioEventTableDeletedAtColumn

		case "camera_id":
			columnName = AudioEventTableCameraIDColumn

		case "video_id":
			columnName = AudioEventTableVideoIDColumn

		case "seen_at":
			columnName = AudioEventTableSeenAtColumn

		case "offset_ms":
			columnName = AudioEventTableOffsetMsColumn

		case "ended_at":
			columnName = AudioEventTableEndedAtColumn

		case "duration_ms":
			columnName = AudioEventTableDurationMsColumn

		case "kind":
			columnName = AudioEventTableKindColumn

		case "rms_dbfs":
			columnName = AudioEventTableRmsDbfsColumn

		case "peak_dbfs":
			columnName = AudioEventTablePeakDbfsColumn

		case "floor_dbfs":
			columnName = AudioEventTableFloorDbfsColumn

		case "prominence_db":
			columnName = AudioEventTableProminenceDbColumn

		case "crest_db":
			columnName = AudioEventTableCrestDbColumn

		case "score":
			columnName = AudioEventTableScoreColumn

		case "spectral":
			columnName = AudioEventTableSpectralColumn
		default:
			return fmt.Errorf("unknown field name: %v", fieldName)
		}

		var columnValue any
		var err error
		switch columnName {

		case AudioEventTableIDColumn:
			columnValue, err = types.FormatUUID(value)

		case AudioEventTableCreatedAtColumn:
			columnValue, err = types.FormatTime(value)

		case AudioEventTableUpdatedAtColumn:
			columnValue, err = types.FormatTime(value)

		case AudioEventTableDeletedAtColumn:
			columnValue, err = types.FormatTime(value)

		case AudioEventTableCameraIDColumn:
			columnValue, err = types.FormatUUID(value)

		case AudioEventTableVideoIDColumn:
			columnValue, err = types.FormatUUID(value)

		case AudioEventTableSeenAtColumn:
			columnValue, err = types.FormatTime(value)

		case AudioEventTableOffsetMsColumn:
			columnValue, err = types.FormatInt(value)

		case AudioEventTableEndedAtColumn:
			columnValue, err = types.FormatTime(value)

		case AudioEventTableDurationMsColumn:
			columnValue, err = types.FormatInt(value)

		case AudioEventTableKindColumn:
			columnValue, err = types.FormatString(value)

		case AudioEventTableRmsDbfsColumn:
			columnValue, err = types.FormatFloat(value)

		case AudioEventTablePeakDbfsColumn:
			columnValue, err = types.FormatFloat(value)

		case AudioEventTableFloorDbfsColumn:
			columnValue, err = types.FormatFloat(value)

		case AudioEventTableProminenceDbColumn:
			columnValue, err = types.FormatFloat(value)

		case AudioEventTableCrestDbColumn:
			columnValue, err = types.FormatFloat(value)

		case AudioEventTableScoreColumn:
			columnValue, err = types.FormatFloat(value)

		case AudioEventTableSpectralColumn:
			columnValue, err = types.FormatJSON(value)
		}
		if err != nil {
			return fmt.Errorf("failed to format value for %v; %v", columnName, err)
		}

		columns = append(columns, columnName)
		values = append(values, columnValue)
	}
	values = append(values, m.GetPrimaryKeyValue())

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	_, err := query.Update(
		ctx,
		tx,
		AudioEventTableWithSchema,
		columns,
		fmt.Sprintf("%v = $$??", AudioEventTablePrimaryKeyColumn),
		[]string{AudioEventTablePrimaryKeyColumn},
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to update fields: %v", err)
	}

	err = m.Reload(ctx, tx, false)
	if err != nil {
		return fmt.Errorf("failed to reload after update")
	}

	return nil
}

func (m *AudioEvent) Delete(ctx context.Context, tx pgx.Tx, hardDeletes ...bool) error {
	hardDelete := false
	if len(hardDeletes) > 0 {
		hardDelete = hardDeletes[0]
	}

	if !hardDelete && slices.Contains(AudioEventTableColumns, "deleted_at") {
		m.DeletedAt = helpers.Ptr(time.Now().UTC())
		err := m.Update(ctx, tx, false, "deleted_at")
		if err != nil {
			return fmt.Errorf("failed to soft-delete (update) %#+v; %v", m, err)
		}
	}

	values := make([]any, 0)
	v, err := types.FormatUUID(m.ID)
	if err != nil {
		return fmt.Errorf("failed to handle m.ID; %v", err)
	}

	values = append(values, v)

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	err = query.Delete(
		ctx,
		tx,
		AudioEventTableWithSchema,
		fmt.Sprintf("%v = $$??", AudioEventTableIDColumn),
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to delete %#+v; %v", m, err)
	}

	_ = m.Reload(ctx, tx, true)

	return nil
}

func (m *AudioEvent) LockTable(ctx context.Context, tx pgx.Tx, timeouts ...time.Duration) error {
	return query.LockTable(ctx, tx, AudioEventTableWithSchema, timeouts...)
}

func (m *AudioEvent) LockTableWithRetries(ctx context.Context, tx pgx.Tx, overallTimeout time.Duration, individualAttempttimeout time.Duration) error {
	return query.LockTableWithRetries(ctx, tx, AudioEventTableWithSchema, overallTimeout, individualAttempttimeout)
}

func (m *AudioEvent) AdvisoryLock(ctx context.Context, tx pgx.Tx, key int32, timeouts ...time.Duration) error {
	return query.AdvisoryLock(ctx, tx, AudioEventTableNamespaceID, key, timeouts...)
}

func (m *AudioEvent) AdvisoryLockWithRetries(ctx context.Context, tx pgx.Tx, key int32, overallTimeout time.Duration, individualAttempttimeout time.Duration) error {
	return query.AdvisoryLockWithRetries(ctx, tx, AudioEventTableNamespaceID, key, overallTimeout, individualAttempttimeout)
}

func SelectAudioEvents(ctx context.Context, tx pgx.Tx, where string, orderBy *string, limit *int, offset *int, values ...any) ([]*AudioEvent, int64, int64, int64, int64, error) {
	before := time.Now()

	if config.Debug() {
		log.Printf("entered SelectAudioEvents")

		defer func() {
			log.Printf("exited SelectAudioEvents in %s", time.Since(before))
		}()
	}
	if slices.Contains(AudioEventTableColumns, "deleted_at") {
		if !strings.Contains(where, "deleted_at") {
			if where != "" {
				where += "\n    AND "
			}

			where += "deleted_at IS null"
		}
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	possiblePathValue := query.GetCurrentPathValue(ctx)
	isLoadQuery := possiblePathValue != nil && len(possiblePathValue.VisitedTableNames) > 0

	shouldLoad := query.ShouldLoad(ctx, AudioEventTable) || query.ShouldLoad(ctx, fmt.Sprintf("referenced_by_%s", AudioEventTable))
	shouldLoad = shouldLoad || query.ShouldLoadFromSelectionTree(ctx, AudioEventTable) || query.ShouldLoadFromSelectionTree(ctx, fmt.Sprintf("referenced_by_%s", AudioEventTable))

	var action query.PathAction
	ctx, action = query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("%s{%v}", AudioEventTable, nil), !isLoadQuery)
	if (action == query.PathActionCycleDetected || action == query.PathActionDepthExceeded) && !shouldLoad {
		if config.Debug() {
			log.Printf("skipping SelectAudioEvent early (query.ShouldLoad(): %v, action: %v)", shouldLoad, action)
		}
		return []*AudioEvent{}, 0, 0, 0, 0, nil
	}

	var items *[]map[string]any
	var count int64
	var totalCount int64
	var page int64
	var totalPages int64
	var err error

	useInstead, shouldSkip := query.ShouldSkip[AudioEvent](ctx)
	if !shouldSkip {
		items, count, totalCount, page, totalPages, err = query.Select(
			ctx,
			tx,
			AudioEventTableColumnsWithTypeCasts,
			AudioEventTableWithSchema,
			where,
			orderBy,
			limit,
			offset,
			values...,
		)
		if err != nil {
			return nil, 0, 0, 0, 0, fmt.Errorf("failed to call SelectAudioEvents; %v", err)
		}
	} else {
		ctx = query.WithoutSkip(ctx)
		count = 1
		totalCount = 1
		page = 1
		totalPages = 1
		items = &[]map[string]any{
			nil,
		}
	}

	objects := make([]*AudioEvent, 0)

	for _, item := range *items {
		var object *AudioEvent

		if !shouldSkip {
			object = &AudioEvent{}
			err = object.FromItem(item)
			if err != nil {
				return nil, 0, 0, 0, 0, err
			}
		} else {
			object = useInstead
		}

		if object == nil {
			return nil, 0, 0, 0, 0, fmt.Errorf("assertion failed: object unexpectedly nil")
		}
		if !types.IsZeroUUID(object.CameraID) {
			ctx, action := query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("%s{%v}", CameraTable, object.CameraID), true)
			childNode := query.GetSelectionNode(ctx, query.RelationKey{
				Direction:   query.RelationDirectionOutbound,
				TableName:   CameraTable,
				LocalColumn: AudioEventTableCameraIDColumn,
			})
			shouldLoad := query.ShouldLoad(ctx, CameraTable)
			shouldLoad = shouldLoad || (childNode != nil && childNode.Selected)
			loadFK := true
			switch action {
			case query.PathActionCycleDetected:
				loadFK = false
			case query.PathActionDepthExceeded:
				if !shouldLoad {
					loadFK = false
				}
			case query.PathActionContinue:
			}

			if loadFK {
				thisBefore := time.Now()

				where := []string{fmt.Sprintf("%v = $1", CameraTablePrimaryKeyColumn)}
				values := []any{object.CameraID}
				if childNode != nil && childNode.Selected {
					where = []string{fmt.Sprintf("%v = $$??", CameraTablePrimaryKeyColumn)}
					values = []any{object.CameraID}
					where = append(where, childNode.Filters.Where...)
					values = append(values, childNode.Filters.Values...)
				}

				if config.Debug() {
					log.Printf("loading SelectAudioEvents->SelectCamera for object.CameraIDObject{%s: %v}", CameraTablePrimaryKeyColumn, object.CameraID)
				}

				object.CameraIDObject, _, _, _, _, err = SelectCamera(
					ctx,
					tx,
					strings.Join(where, "\n    AND "),
					values...,
				)
				if err != nil {
					if !errors.Is(err, sql.ErrNoRows) {
						return nil, 0, 0, 0, 0, err
					}
				}

				if config.Debug() {
					log.Printf("loaded SelectAudioEvents->SelectCamera for object.CameraIDObject in %s", time.Since(thisBefore))
				}
			}
		}
		if !types.IsZeroUUID(object.VideoID) {
			ctx, action := query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("%s{%v}", VideoTable, object.VideoID), true)
			childNode := query.GetSelectionNode(ctx, query.RelationKey{
				Direction:   query.RelationDirectionOutbound,
				TableName:   VideoTable,
				LocalColumn: AudioEventTableVideoIDColumn,
			})
			shouldLoad := query.ShouldLoad(ctx, VideoTable)
			shouldLoad = shouldLoad || (childNode != nil && childNode.Selected)
			loadFK := true
			switch action {
			case query.PathActionCycleDetected:
				loadFK = false
			case query.PathActionDepthExceeded:
				if !shouldLoad {
					loadFK = false
				}
			case query.PathActionContinue:
			}

			if loadFK {
				thisBefore := time.Now()

				where := []string{fmt.Sprintf("%v = $1", VideoTablePrimaryKeyColumn)}
				values := []any{object.VideoID}
				if childNode != nil && childNode.Selected {
					where = []string{fmt.Sprintf("%v = $$??", VideoTablePrimaryKeyColumn)}
					values = []any{object.VideoID}
					where = append(where, childNode.Filters.Where...)
					values = append(values, childNode.Filters.Values...)
				}

				if config.Debug() {
					log.Printf("loading SelectAudioEvents->SelectVideo for object.VideoIDObject{%s: %v}", VideoTablePrimaryKeyColumn, object.VideoID)
				}

				object.VideoIDObject, _, _, _, _, err = SelectVideo(
					ctx,
					tx,
					strings.Join(where, "\n    AND "),
					values...,
				)
				if err != nil {
					if !errors.Is(err, sql.ErrNoRows) {
						return nil, 0, 0, 0, 0, err
					}
				}

				if config.Debug() {
					log.Printf("loaded SelectAudioEvents->SelectVideo for object.VideoIDObject in %s", time.Since(thisBefore))
				}
			}
		}

		objects = append(objects, object)
	}

	return objects, count, totalCount, page, totalPages, nil
}

func SelectAudioEvent(ctx context.Context, tx pgx.Tx, where string, values ...any) (*AudioEvent, int64, int64, int64, int64, error) {
	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	objects, _, _, _, _, err := SelectAudioEvents(
		ctx,
		tx,
		where,
		nil,
		helpers.Ptr(2),
		helpers.Ptr(0),
		values...,
	)
	if err != nil {
		return nil, 0, 0, 0, 0, fmt.Errorf("failed to call SelectAudioEvent; %v", err)
	}

	if len(objects) > 1 {
		return nil, 0, 0, 0, 0, fmt.Errorf("attempt to call SelectAudioEvent returned more than 1 row")
	}

	if len(objects) < 1 {
		return nil, 0, 0, 0, 0, sql.ErrNoRows
	}

	object := objects[0]

	count := int64(1)
	totalCount := count
	page := int64(1)
	totalPages := page

	return object, count, totalCount, page, totalPages, nil
}

func InsertAudioEvents(ctx context.Context, tx pgx.Tx, objects []*AudioEvent, setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) ([]*AudioEvent, error) {
	var columns []string
	values := make([]any, 0)

	for i, object := range objects {
		thisColumns, thisValues, err := object.GetColumnsAndValues(setPrimaryKey, setZeroValues, forceSetValuesForFields...)
		if err != nil {
			return nil, err
		}

		if columns == nil {
			columns = thisColumns
		} else {
			if len(columns) != len(thisColumns) {
				return nil, fmt.Errorf(
					"assertion failed: call 1 of object.GetColumnsAndValues() gave %d columns but call %d gave %d columns",
					len(columns),
					i+1,
					len(thisColumns),
				)
			}
		}

		values = append(values, thisValues...)
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	items, err := query.BulkInsert(
		ctx,
		tx,
		AudioEventTableWithSchema,
		columns,
		nil,
		nil,
		nil,
		false,
		false,
		AudioEventTableColumns,
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to bulk insert %d objects; %v", len(objects), err)
	}

	returnedObjects := make([]*AudioEvent, 0)

	for _, item := range items {
		v := &AudioEvent{}
		err = v.FromItem(*item)
		if err != nil {
			return nil, fmt.Errorf("failed %T.FromItem for %#+v; %v", *item, *item, err)
		}

		err = v.Reload(query.WithSkip(ctx, v), tx)
		if err != nil {
			return nil, fmt.Errorf("failed %T.Reload for %#+v; %v", *item, *item, err)
		}

		returnedObjects = append(returnedObjects, v)
	}

	return returnedObjects, nil
}

func handleGetAudioEvents(arguments *server.SelectManyArguments, db *pgxpool.Pool) ([]*AudioEvent, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	objects, count, totalCount, page, totalPages, err := SelectAudioEvents(arguments.Ctx, tx, arguments.Where, arguments.OrderBy, arguments.Limit, arguments.Offset, arguments.Values...)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	return objects, count, totalCount, page, totalPages, nil
}

func handleGetAudioEvent(arguments *server.SelectOneArguments, db *pgxpool.Pool, primaryKey uuid.UUID) ([]*AudioEvent, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	object, count, totalCount, page, totalPages, err := SelectAudioEvent(arguments.Ctx, tx, arguments.Where, arguments.Values...)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	return []*AudioEvent{object}, count, totalCount, page, totalPages, nil
}

func handlePostAudioEvent(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, objects []*AudioEvent, forceSetValuesForFieldsByObjectIndex [][]string) ([]*AudioEvent, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction; %v", err)
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid; %v", err)
		return nil, 0, 0, 0, 0, err
	}

	/* TODO: problematic- basically the bulks insert insists all rows have the same schema, which they usually should */
	forceSetValuesForFieldsByObjectIndexMaximal := make(map[string]struct{})
	for _, forceSetforceSetValuesForFields := range forceSetValuesForFieldsByObjectIndex {
		for _, field := range forceSetforceSetValuesForFields {
			forceSetValuesForFieldsByObjectIndexMaximal[field] = struct{}{}
		}
	}

	returnedObjects, err := InsertAudioEvents(arguments.Ctx, tx, objects, false, false, slices.Collect(maps.Keys(forceSetValuesForFieldsByObjectIndexMaximal))...)
	if err != nil {
		err = fmt.Errorf("failed to insert %d objects; %v", len(objects), err)
		return nil, 0, 0, 0, 0, err
	}

	copy(objects, returnedObjects)

	errs := make(chan error, 1)
	go func() {
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.INSERT}, AudioEventTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change; %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction; %v", err)
		return nil, 0, 0, 0, 0, err
	}

	select {
	case <-arguments.Ctx.Done():
		err = fmt.Errorf("context canceled")
		return nil, 0, 0, 0, 0, err
	case err = <-errs:
		if err != nil {
			return nil, 0, 0, 0, 0, err
		}
	}

	count := int64(len(objects))
	totalCount := count
	page := int64(1)
	totalPages := page

	return objects, count, totalCount, page, totalPages, nil
}

func handlePutAudioEvent(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *AudioEvent) ([]*AudioEvent, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction; %v", err)
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid; %v", err)
		return nil, 0, 0, 0, 0, err
	}
	_ = xid

	err = object.Update(arguments.Ctx, tx, true)
	if err != nil {
		err = fmt.Errorf("failed to update %#+v; %v", object, err)
		return nil, 0, 0, 0, 0, err
	}

	errs := make(chan error, 1)
	go func() {
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, AudioEventTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change; %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction; %v", err)
		return nil, 0, 0, 0, 0, err
	}

	select {
	case <-arguments.Ctx.Done():
		err = fmt.Errorf("context canceled")
		return nil, 0, 0, 0, 0, err
	case err = <-errs:
		if err != nil {
			return nil, 0, 0, 0, 0, err
		}
	}

	count := int64(1)
	totalCount := count
	page := int64(1)
	totalPages := page

	return []*AudioEvent{object}, count, totalCount, page, totalPages, nil
}

func handlePatchAudioEvent(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *AudioEvent, forceSetValuesForFields []string) ([]*AudioEvent, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction; %v", err)
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid; %v", err)
		return nil, 0, 0, 0, 0, err
	}
	_ = xid

	err = object.Update(arguments.Ctx, tx, false, forceSetValuesForFields...)
	if err != nil {
		err = fmt.Errorf("failed to update %#+v; %v", object, err)
		return nil, 0, 0, 0, 0, err
	}

	errs := make(chan error, 1)
	go func() {
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, AudioEventTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change; %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction; %v", err)
		return nil, 0, 0, 0, 0, err
	}

	select {
	case <-arguments.Ctx.Done():
		err = fmt.Errorf("context canceled")
		return nil, 0, 0, 0, 0, err
	case err = <-errs:
		if err != nil {
			return nil, 0, 0, 0, 0, err
		}
	}

	count := int64(1)
	totalCount := count
	page := int64(1)
	totalPages := page

	return []*AudioEvent{object}, count, totalCount, page, totalPages, nil
}

func handleDeleteAudioEvent(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *AudioEvent) error {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction; %v", err)
		return err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	xid, err := query.GetXid(arguments.Ctx, tx)
	if err != nil {
		err = fmt.Errorf("failed to get xid; %v", err)
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
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.DELETE, stream.SOFT_DELETE}, AudioEventTable, xid)
		if err != nil {
			err = fmt.Errorf("failed to wait for change; %v", err)
			errs <- err
			return
		}

		errs <- nil
	}()

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction; %v", err)
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

func MutateRouterForAudioEvent(r chi.Router, db *pgxpool.Pool, redisPool *redis.Pool, objectMiddlewares []server.ObjectMiddleware, waitForChange server.WaitForChange) {

	func() {
		getManyHandler, err := getHTTPHandler(
			http.MethodGet,
			"/audio-events",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams map[string]any,
				req server.EmptyRequest,
				rawReq any,
			) (server.Response[AudioEvent], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectManyArguments(ctx, queryParams, AudioEventIntrospectedTable, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache not yet reached; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				cachedResponseAsJSON, cacheHit, err := server.GetCachedResponseAsJSON(arguments.RequestHash, redisConn)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache failed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				if cacheHit {
					var cachedResponse server.Response[AudioEvent]

					/* TODO: it'd be nice to be able to avoid this (i.e. just pass straight through) */
					err = json.Unmarshal(cachedResponseAsJSON, &cachedResponse)
					if err != nil {
						if config.Debug() {
							log.Printf("request cache hit but failed unmarshal; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
						}

						return server.Response[AudioEvent]{}, err
					}

					if config.Debug() {
						log.Printf("request cache hit; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return cachedResponse, nil
				}

				objects, count, totalCount, _, _, err := handleGetAudioEvents(arguments, db)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				limit := int64(0)
				if arguments.Limit != nil {
					limit = int64(*arguments.Limit)
				}

				offset := int64(0)
				if arguments.Offset != nil {
					offset = int64(*arguments.Offset)
				}

				response := server.Response[AudioEvent]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    objects,
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}

				/* TODO: it'd be nice to be able to avoid this (i.e. just marshal once, further out) */
				responseAsJSON, err := json.Marshal(response)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				err = server.StoreCachedResponse(arguments.RequestHash, redisConn, responseAsJSON)
				if err != nil {
					log.Printf("warning; %v", err)
				}

				if config.Debug() {
					log.Printf("request cache missed; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
				}

				return response, nil
			},
			AudioEvent{},
			AudioEventIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Get(getManyHandler.FullPath, getManyHandler.ServeHTTP)
	}()

	func() {
		getOneHandler, err := getHTTPHandler(
			http.MethodGet,
			"/audio-events/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams AudioEventOnePathParams,
				queryParams AudioEventLoadQueryParams,
				req server.EmptyRequest,
				rawReq any,
			) (server.Response[AudioEvent], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectOneArguments(ctx, queryParams.Depth, AudioEventIntrospectedTable, pathParams.PrimaryKey, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache not yet reached; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				cachedResponseAsJSON, cacheHit, err := server.GetCachedResponseAsJSON(arguments.RequestHash, redisConn)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache failed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				if cacheHit {
					var cachedResponse server.Response[AudioEvent]

					/* TODO: it'd be nice to be able to avoid this (i.e. just pass straight through) */
					err = json.Unmarshal(cachedResponseAsJSON, &cachedResponse)
					if err != nil {
						if config.Debug() {
							log.Printf("request cache hit but failed unmarshal; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
						}

						return server.Response[AudioEvent]{}, err
					}

					if config.Debug() {
						log.Printf("request cache hit; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return cachedResponse, nil
				}

				objects, count, totalCount, _, _, err := handleGetAudioEvent(arguments, db, pathParams.PrimaryKey)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				response := server.Response[AudioEvent]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    objects,
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}

				/* TODO: it'd be nice to be able to avoid this (i.e. just marshal once, further out) */
				responseAsJSON, err := json.Marshal(response)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[AudioEvent]{}, err
				}

				err = server.StoreCachedResponse(arguments.RequestHash, redisConn, responseAsJSON)
				if err != nil {
					log.Printf("warning; %v", err)
				}

				if config.Debug() {
					log.Printf("request cache hit; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
				}

				return response, nil
			},
			AudioEvent{},
			AudioEventIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Get(getOneHandler.FullPath, getOneHandler.ServeHTTP)
	}()

	func() {
		postHandler, err := getHTTPHandler(
			http.MethodPost,
			"/audio-events",
			http.StatusCreated,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams AudioEventLoadQueryParams,
				req []*AudioEvent,
				rawReq any,
			) (server.Response[AudioEvent], error) {
				allRawItems, ok := rawReq.([]any)
				if !ok {
					return server.Response[AudioEvent]{}, fmt.Errorf("failed to cast %#+v to []map[string]any", rawReq)
				}

				allItems := make([]map[string]any, 0)
				for _, rawItem := range allRawItems {
					item, ok := rawItem.(map[string]any)
					if !ok {
						return server.Response[AudioEvent]{}, fmt.Errorf("failed to cast %#+v to map[string]any", rawItem)
					}

					allItems = append(allItems, item)
				}

				forceSetValuesForFieldsByObjectIndex := make([][]string, 0)
				for _, item := range allItems {
					forceSetValuesForFields := make([]string, 0)
					for _, possibleField := range slices.Collect(maps.Keys(item)) {
						if !slices.Contains(AudioEventTableColumns, possibleField) {
							continue
						}

						forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
					}
					forceSetValuesForFieldsByObjectIndex = append(forceSetValuesForFieldsByObjectIndex, forceSetValuesForFields)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[AudioEvent]{}, err
				}

				objects, count, totalCount, _, _, err := handlePostAudioEvent(arguments, db, waitForChange, req, forceSetValuesForFieldsByObjectIndex)
				if err != nil {
					return server.Response[AudioEvent]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[AudioEvent]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    objects,
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}, nil
			},
			AudioEvent{},
			AudioEventIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Post(postHandler.FullPath, postHandler.ServeHTTP)
	}()

	func() {
		putHandler, err := getHTTPHandler(
			http.MethodPatch,
			"/audio-events/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams AudioEventOnePathParams,
				queryParams AudioEventLoadQueryParams,
				req AudioEvent,
				rawReq any,
			) (server.Response[AudioEvent], error) {
				item, ok := rawReq.(map[string]any)
				if !ok {
					return server.Response[AudioEvent]{}, fmt.Errorf("failed to cast %#+v to map[string]any", item)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[AudioEvent]{}, err
				}

				object := &req
				object.ID = pathParams.PrimaryKey

				objects, count, totalCount, _, _, err := handlePutAudioEvent(arguments, db, waitForChange, object)
				if err != nil {
					return server.Response[AudioEvent]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[AudioEvent]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    objects,
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}, nil
			},
			AudioEvent{},
			AudioEventIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Put(putHandler.FullPath, putHandler.ServeHTTP)
	}()

	func() {
		patchHandler, err := getHTTPHandler(
			http.MethodPatch,
			"/audio-events/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams AudioEventOnePathParams,
				queryParams AudioEventLoadQueryParams,
				req AudioEvent,
				rawReq any,
			) (server.Response[AudioEvent], error) {
				item, ok := rawReq.(map[string]any)
				if !ok {
					return server.Response[AudioEvent]{}, fmt.Errorf("failed to cast %#+v to map[string]any", item)
				}

				forceSetValuesForFields := make([]string, 0)
				for _, possibleField := range slices.Collect(maps.Keys(item)) {
					if !slices.Contains(AudioEventTableColumns, possibleField) {
						continue
					}

					forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[AudioEvent]{}, err
				}

				object := &req
				object.ID = pathParams.PrimaryKey

				objects, count, totalCount, _, _, err := handlePatchAudioEvent(arguments, db, waitForChange, object, forceSetValuesForFields)
				if err != nil {
					return server.Response[AudioEvent]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[AudioEvent]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    objects,
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}, nil
			},
			AudioEvent{},
			AudioEventIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Patch(patchHandler.FullPath, patchHandler.ServeHTTP)
	}()

	func() {
		deleteHandler, err := getHTTPHandler(
			http.MethodDelete,
			"/audio-events/{primaryKey}",
			http.StatusNoContent,
			func(
				ctx context.Context,
				pathParams AudioEventOnePathParams,
				queryParams AudioEventLoadQueryParams,
				req server.EmptyRequest,
				rawReq any,
			) (server.EmptyResponse, error) {
				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.EmptyResponse{}, err
				}

				object := &AudioEvent{}
				object.ID = pathParams.PrimaryKey

				err = handleDeleteAudioEvent(arguments, db, waitForChange, object)
				if err != nil {
					return server.EmptyResponse{}, err
				}

				return server.EmptyResponse{}, nil
			},
			AudioEvent{},
			AudioEventIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Delete(deleteHandler.FullPath, deleteHandler.ServeHTTP)
	}()
}

func NewAudioEventFromItem(item map[string]any) (any, error) {
	object := &AudioEvent{}

	err := object.FromItem(item)
	if err != nil {
		return nil, err
	}

	return object, nil
}

func init() {
	register(
		AudioEventTable,
		AudioEvent{},
		NewAudioEventFromItem,
		"/audio-events",
		MutateRouterForAudioEvent,
	)
}
