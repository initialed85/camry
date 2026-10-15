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

type Track struct {
	ID                                  uuid.UUID     `json:"id"`
	Key                                 uuid.UUID     `json:"key"`
	CreatedAt                           time.Time     `json:"created_at"`
	UpdatedAt                           time.Time     `json:"updated_at"`
	DeletedAt                           *time.Time    `json:"deleted_at"`
	CameraID                            uuid.UUID     `json:"camera_id"`
	CameraIDObject                      *Camera       `json:"camera_id_object"`
	VideoID                             uuid.UUID     `json:"video_id"`
	VideoIDObject                       *Video        `json:"video_id_object"`
	ClassID                             int64         `json:"class_id"`
	ClassName                           string        `json:"class_name"`
	StartedAt                           time.Time     `json:"started_at"`
	EndedAt                             time.Time     `json:"ended_at"`
	FrameCount                          int64         `json:"frame_count"`
	MeanScore                           float64       `json:"mean_score"`
	MaxScore                            float64       `json:"max_score"`
	Centroid                            pgtype.Vec2   `json:"centroid"`
	BoundingBox                         []pgtype.Vec2 `json:"bounding_box"`
	Trajectory                          any           `json:"trajectory"`
	IsStatic                            bool          `json:"is_static"`
	Status                              string        `json:"status"`
	ReferencedByDetectionTrackIDObjects []*Detection  `json:"referenced_by_detection_track_id_objects"`
}

var TrackTable = "track"

var TrackTableWithSchema = fmt.Sprintf("%s.%s", schema, TrackTable)

var TrackTableNamespaceID int32 = 1337 + 4

var (
	TrackTableIDColumn          = "id"
	TrackTableKeyColumn         = "key"
	TrackTableCreatedAtColumn   = "created_at"
	TrackTableUpdatedAtColumn   = "updated_at"
	TrackTableDeletedAtColumn   = "deleted_at"
	TrackTableCameraIDColumn    = "camera_id"
	TrackTableVideoIDColumn     = "video_id"
	TrackTableClassIDColumn     = "class_id"
	TrackTableClassNameColumn   = "class_name"
	TrackTableStartedAtColumn   = "started_at"
	TrackTableEndedAtColumn     = "ended_at"
	TrackTableFrameCountColumn  = "frame_count"
	TrackTableMeanScoreColumn   = "mean_score"
	TrackTableMaxScoreColumn    = "max_score"
	TrackTableCentroidColumn    = "centroid"
	TrackTableBoundingBoxColumn = "bounding_box"
	TrackTableTrajectoryColumn  = "trajectory"
	TrackTableIsStaticColumn    = "is_static"
	TrackTableStatusColumn      = "status"
)

var (
	TrackTableIDColumnWithTypeCast          = `"id" AS id`
	TrackTableKeyColumnWithTypeCast         = `"key" AS key`
	TrackTableCreatedAtColumnWithTypeCast   = `"created_at" AS created_at`
	TrackTableUpdatedAtColumnWithTypeCast   = `"updated_at" AS updated_at`
	TrackTableDeletedAtColumnWithTypeCast   = `"deleted_at" AS deleted_at`
	TrackTableCameraIDColumnWithTypeCast    = `"camera_id" AS camera_id`
	TrackTableVideoIDColumnWithTypeCast     = `"video_id" AS video_id`
	TrackTableClassIDColumnWithTypeCast     = `"class_id" AS class_id`
	TrackTableClassNameColumnWithTypeCast   = `"class_name" AS class_name`
	TrackTableStartedAtColumnWithTypeCast   = `"started_at" AS started_at`
	TrackTableEndedAtColumnWithTypeCast     = `"ended_at" AS ended_at`
	TrackTableFrameCountColumnWithTypeCast  = `"frame_count" AS frame_count`
	TrackTableMeanScoreColumnWithTypeCast   = `"mean_score" AS mean_score`
	TrackTableMaxScoreColumnWithTypeCast    = `"max_score" AS max_score`
	TrackTableCentroidColumnWithTypeCast    = `"centroid" AS centroid`
	TrackTableBoundingBoxColumnWithTypeCast = `"bounding_box" AS bounding_box`
	TrackTableTrajectoryColumnWithTypeCast  = `"trajectory" AS trajectory`
	TrackTableIsStaticColumnWithTypeCast    = `"is_static" AS is_static`
	TrackTableStatusColumnWithTypeCast      = `"status" AS status`
)

var TrackTableColumns = []string{
	TrackTableIDColumn,
	TrackTableKeyColumn,
	TrackTableCreatedAtColumn,
	TrackTableUpdatedAtColumn,
	TrackTableDeletedAtColumn,
	TrackTableCameraIDColumn,
	TrackTableVideoIDColumn,
	TrackTableClassIDColumn,
	TrackTableClassNameColumn,
	TrackTableStartedAtColumn,
	TrackTableEndedAtColumn,
	TrackTableFrameCountColumn,
	TrackTableMeanScoreColumn,
	TrackTableMaxScoreColumn,
	TrackTableCentroidColumn,
	TrackTableBoundingBoxColumn,
	TrackTableTrajectoryColumn,
	TrackTableIsStaticColumn,
	TrackTableStatusColumn,
}

var TrackTableColumnsWithTypeCasts = []string{
	TrackTableIDColumnWithTypeCast,
	TrackTableKeyColumnWithTypeCast,
	TrackTableCreatedAtColumnWithTypeCast,
	TrackTableUpdatedAtColumnWithTypeCast,
	TrackTableDeletedAtColumnWithTypeCast,
	TrackTableCameraIDColumnWithTypeCast,
	TrackTableVideoIDColumnWithTypeCast,
	TrackTableClassIDColumnWithTypeCast,
	TrackTableClassNameColumnWithTypeCast,
	TrackTableStartedAtColumnWithTypeCast,
	TrackTableEndedAtColumnWithTypeCast,
	TrackTableFrameCountColumnWithTypeCast,
	TrackTableMeanScoreColumnWithTypeCast,
	TrackTableMaxScoreColumnWithTypeCast,
	TrackTableCentroidColumnWithTypeCast,
	TrackTableBoundingBoxColumnWithTypeCast,
	TrackTableTrajectoryColumnWithTypeCast,
	TrackTableIsStaticColumnWithTypeCast,
	TrackTableStatusColumnWithTypeCast,
}

var TrackIntrospectedTable *introspect.Table

var TrackTableColumnLookup map[string]*introspect.Column

var (
	TrackTablePrimaryKeyColumn = TrackTableIDColumn
)

func init() {
	TrackIntrospectedTable = tableByName[TrackTable]

	/* only needed during templating */
	if TrackIntrospectedTable == nil {
		TrackIntrospectedTable = &introspect.Table{}
	}

	TrackTableColumnLookup = TrackIntrospectedTable.ColumnByName
}

type TrackOnePathParams struct {
	PrimaryKey uuid.UUID `json:"primaryKey"`
}

type TrackLoadQueryParams struct {
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

func (m *Track) GetPrimaryKeyColumn() string {
	return TrackTablePrimaryKeyColumn
}

func (m *Track) GetPrimaryKeyValue() any {
	return m.ID
}
func (Track) GetQueryInfo() query.ModelQueryInfo {
	return query.ModelQueryInfo{
		TableName:        "track",
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
			{
				Name:         "referenced_by_Detection_TrackID",
				Direction:    query.RelationDirectionInbound,
				TargetTable:  "detection",
				LocalColumn:  "track_id",
				TargetColumn: "id",
				Cardinality:  query.RelationCardinalityMany,
			},
		},
		Filters: []query.FilterDefinition{
			{
				Name:     "key",
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
				Name:     "class_id",
				DataType: "bigint",
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
				Name:     "class_name",
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
				Name:     "started_at",
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
				Name:     "frame_count",
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
				Name:     "mean_score",
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
				Name:     "max_score",
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
				Name:     "trajectory",
				DataType: "jsonb",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpContains,
					query.FilterOpNotContains,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "is_static",
				DataType: "boolean",
				Nullable: false,
				Operators: []query.FilterOperator{
					query.FilterOpEq,
					query.FilterOpNe,
					query.FilterOpIsFalse,
					query.FilterOpIsTrue,
					query.FilterOpAsc,
					query.FilterOpDesc,
				},
			},
			{
				Name:     "status",
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
		},
	}
}

func (m *Track) FromItem(item map[string]any) error {
	if item == nil {
		return fmt.Errorf(
			"item unexpectedly nil during TrackFromItem",
		)
	}

	if len(item) == 0 {
		return fmt.Errorf(
			"item unexpectedly empty during TrackFromItem",
		)
	}

	wrapError := func(k string, v any, err error) error {
		return fmt.Errorf("%v: %#+v; error; %v", k, v, err)
	}

	for k, v := range item {
		_, ok := TrackTableColumnLookup[k]
		if !ok {
			return fmt.Errorf(
				"item contained unexpected key %#+v during TrackFromItem; item: %#+v",
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

		case "key":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uukey.UUID", temp1))
				}
			}

			m.Key = temp2

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

		case "class_id":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuclass_id.UUID", temp1))
				}
			}

			m.ClassID = temp2

		case "class_name":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuclass_name.UUID", temp1))
				}
			}

			m.ClassName = temp2

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

			m.EndedAt = temp2

		case "frame_count":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuframe_count.UUID", temp1))
				}
			}

			m.FrameCount = temp2

		case "mean_score":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uumean_score.UUID", temp1))
				}
			}

			m.MeanScore = temp2

		case "max_score":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uumax_score.UUID", temp1))
				}
			}

			m.MaxScore = temp2

		case "centroid":
			if v == nil {
				continue
			}

			temp1, err := types.ParsePoint(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(pgtype.Vec2)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uucentroid.UUID", temp1))
				}
			}

			m.Centroid = temp2

		case "bounding_box":
			if v == nil {
				continue
			}

			temp1, err := types.ParsePolygon(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.([]pgtype.Vec2)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uubounding_box.UUID", temp1))
				}
			}

			m.BoundingBox = temp2

		case "trajectory":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uutrajectory.UUID", temp1))
				}
			}

			m.Trajectory = temp2

		case "is_static":
			if v == nil {
				continue
			}

			temp1, err := types.ParseBool(v)
			if err != nil {
				return wrapError(k, v, err)
			}

			temp2, ok := temp1.(bool)
			if !ok {
				if temp1 != nil {
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuis_static.UUID", temp1))
				}
			}

			m.IsStatic = temp2

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

			m.Status = temp2

		}
	}

	return nil
}

func (m *Track) ToItem() map[string]any {
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

func (m *Track) Reload(ctx context.Context, tx pgx.Tx, includeDeleteds ...bool) error {
	extraWhere := ""
	if len(includeDeleteds) > 0 && includeDeleteds[0] {
		if slices.Contains(TrackTableColumns, "deleted_at") {
			extraWhere = "\n    AND (deleted_at IS null OR deleted_at IS NOT null)"
		}
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	o, _, _, _, _, err := SelectTrack(
		ctx,
		tx,
		fmt.Sprintf("%v = $1%v", m.GetPrimaryKeyColumn(), extraWhere),
		m.GetPrimaryKeyValue(),
	)
	if err != nil {
		return err
	}

	m.ID = o.ID
	m.Key = o.Key
	m.CreatedAt = o.CreatedAt
	m.UpdatedAt = o.UpdatedAt
	m.DeletedAt = o.DeletedAt
	m.CameraID = o.CameraID
	m.CameraIDObject = o.CameraIDObject
	m.VideoID = o.VideoID
	m.VideoIDObject = o.VideoIDObject
	m.ClassID = o.ClassID
	m.ClassName = o.ClassName
	m.StartedAt = o.StartedAt
	m.EndedAt = o.EndedAt
	m.FrameCount = o.FrameCount
	m.MeanScore = o.MeanScore
	m.MaxScore = o.MaxScore
	m.Centroid = o.Centroid
	m.BoundingBox = o.BoundingBox
	m.Trajectory = o.Trajectory
	m.IsStatic = o.IsStatic
	m.Status = o.Status
	m.ReferencedByDetectionTrackIDObjects = o.ReferencedByDetectionTrackIDObjects

	return nil
}

func (m *Track) GetColumnsAndValues(setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) ([]string, []any, error) {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setPrimaryKey && (setZeroValues || !types.IsZeroUUID(m.ID) || slices.Contains(forceSetValuesForFields, TrackTableIDColumn) || isRequired(TrackTableColumnLookup, TrackTableIDColumn)) {
		columns = append(columns, TrackTableIDColumn)

		v, err := types.FormatUUID(m.ID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.ID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.Key) || slices.Contains(forceSetValuesForFields, TrackTableKeyColumn) || isRequired(TrackTableColumnLookup, TrackTableKeyColumn) {
		columns = append(columns, TrackTableKeyColumn)

		v, err := types.FormatUUID(m.Key)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Key; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, TrackTableCreatedAtColumn) || isRequired(TrackTableColumnLookup, TrackTableCreatedAtColumn) {
		columns = append(columns, TrackTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.CreatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, TrackTableUpdatedAtColumn) || isRequired(TrackTableColumnLookup, TrackTableUpdatedAtColumn) {
		columns = append(columns, TrackTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.UpdatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, TrackTableDeletedAtColumn) || isRequired(TrackTableColumnLookup, TrackTableDeletedAtColumn) {
		columns = append(columns, TrackTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.DeletedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) || slices.Contains(forceSetValuesForFields, TrackTableCameraIDColumn) || isRequired(TrackTableColumnLookup, TrackTableCameraIDColumn) {
		columns = append(columns, TrackTableCameraIDColumn)

		v, err := types.FormatUUID(m.CameraID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.CameraID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.VideoID) || slices.Contains(forceSetValuesForFields, TrackTableVideoIDColumn) || isRequired(TrackTableColumnLookup, TrackTableVideoIDColumn) {
		columns = append(columns, TrackTableVideoIDColumn)

		v, err := types.FormatUUID(m.VideoID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.VideoID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.ClassID) || slices.Contains(forceSetValuesForFields, TrackTableClassIDColumn) || isRequired(TrackTableColumnLookup, TrackTableClassIDColumn) {
		columns = append(columns, TrackTableClassIDColumn)

		v, err := types.FormatInt(m.ClassID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.ClassID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.ClassName) || slices.Contains(forceSetValuesForFields, TrackTableClassNameColumn) || isRequired(TrackTableColumnLookup, TrackTableClassNameColumn) {
		columns = append(columns, TrackTableClassNameColumn)

		v, err := types.FormatString(m.ClassName)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.ClassName; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.StartedAt) || slices.Contains(forceSetValuesForFields, TrackTableStartedAtColumn) || isRequired(TrackTableColumnLookup, TrackTableStartedAtColumn) {
		columns = append(columns, TrackTableStartedAtColumn)

		v, err := types.FormatTime(m.StartedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.StartedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.EndedAt) || slices.Contains(forceSetValuesForFields, TrackTableEndedAtColumn) || isRequired(TrackTableColumnLookup, TrackTableEndedAtColumn) {
		columns = append(columns, TrackTableEndedAtColumn)

		v, err := types.FormatTime(m.EndedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.EndedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.FrameCount) || slices.Contains(forceSetValuesForFields, TrackTableFrameCountColumn) || isRequired(TrackTableColumnLookup, TrackTableFrameCountColumn) {
		columns = append(columns, TrackTableFrameCountColumn)

		v, err := types.FormatInt(m.FrameCount)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.FrameCount; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.MeanScore) || slices.Contains(forceSetValuesForFields, TrackTableMeanScoreColumn) || isRequired(TrackTableColumnLookup, TrackTableMeanScoreColumn) {
		columns = append(columns, TrackTableMeanScoreColumn)

		v, err := types.FormatFloat(m.MeanScore)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.MeanScore; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.MaxScore) || slices.Contains(forceSetValuesForFields, TrackTableMaxScoreColumn) || isRequired(TrackTableColumnLookup, TrackTableMaxScoreColumn) {
		columns = append(columns, TrackTableMaxScoreColumn)

		v, err := types.FormatFloat(m.MaxScore)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.MaxScore; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPoint(m.Centroid) || slices.Contains(forceSetValuesForFields, TrackTableCentroidColumn) || isRequired(TrackTableColumnLookup, TrackTableCentroidColumn) {
		columns = append(columns, TrackTableCentroidColumn)

		v, err := types.FormatPoint(m.Centroid)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Centroid; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPolygon(m.BoundingBox) || slices.Contains(forceSetValuesForFields, TrackTableBoundingBoxColumn) || isRequired(TrackTableColumnLookup, TrackTableBoundingBoxColumn) {
		columns = append(columns, TrackTableBoundingBoxColumn)

		v, err := types.FormatPolygon(m.BoundingBox)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.BoundingBox; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroJSON(m.Trajectory) || slices.Contains(forceSetValuesForFields, TrackTableTrajectoryColumn) || isRequired(TrackTableColumnLookup, TrackTableTrajectoryColumn) {
		columns = append(columns, TrackTableTrajectoryColumn)

		v, err := types.FormatJSON(m.Trajectory)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Trajectory; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroBool(m.IsStatic) || slices.Contains(forceSetValuesForFields, TrackTableIsStaticColumn) || isRequired(TrackTableColumnLookup, TrackTableIsStaticColumn) {
		columns = append(columns, TrackTableIsStaticColumn)

		v, err := types.FormatBool(m.IsStatic)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.IsStatic; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Status) || slices.Contains(forceSetValuesForFields, TrackTableStatusColumn) || isRequired(TrackTableColumnLookup, TrackTableStatusColumn) {
		columns = append(columns, TrackTableStatusColumn)

		v, err := types.FormatString(m.Status)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Status; %v", err)
		}

		values = append(values, v)
	}

	return columns, values, nil
}

func (m *Track) Insert(ctx context.Context, tx pgx.Tx, setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) error {
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
		TrackTableWithSchema,
		columns,
		nil,
		nil,
		nil,
		false,
		false,
		TrackTableColumns,
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to insert %#+v; %v", m, err)
	}
	v := (*item)[TrackTableIDColumn]

	if v == nil {
		return fmt.Errorf("failed to find %v in %#+v", TrackTableIDColumn, item)
	}

	wrapError := func(err error) error {
		return fmt.Errorf(
			"failed to treat %v: %#+v as uuid.UUID: %v",
			TrackTableIDColumn,
			(*item)[TrackTableIDColumn],
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

func (m *Track) Update(ctx context.Context, tx pgx.Tx, setZeroValues bool, forceSetValuesForFields ...string) error {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setZeroValues || !types.IsZeroUUID(m.Key) || slices.Contains(forceSetValuesForFields, TrackTableKeyColumn) {
		columns = append(columns, TrackTableKeyColumn)

		v, err := types.FormatUUID(m.Key)
		if err != nil {
			return fmt.Errorf("failed to handle m.Key; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, TrackTableCreatedAtColumn) {
		columns = append(columns, TrackTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.CreatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, TrackTableUpdatedAtColumn) {
		columns = append(columns, TrackTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.UpdatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, TrackTableDeletedAtColumn) {
		columns = append(columns, TrackTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.DeletedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) || slices.Contains(forceSetValuesForFields, TrackTableCameraIDColumn) {
		columns = append(columns, TrackTableCameraIDColumn)

		v, err := types.FormatUUID(m.CameraID)
		if err != nil {
			return fmt.Errorf("failed to handle m.CameraID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.VideoID) || slices.Contains(forceSetValuesForFields, TrackTableVideoIDColumn) {
		columns = append(columns, TrackTableVideoIDColumn)

		v, err := types.FormatUUID(m.VideoID)
		if err != nil {
			return fmt.Errorf("failed to handle m.VideoID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.ClassID) || slices.Contains(forceSetValuesForFields, TrackTableClassIDColumn) {
		columns = append(columns, TrackTableClassIDColumn)

		v, err := types.FormatInt(m.ClassID)
		if err != nil {
			return fmt.Errorf("failed to handle m.ClassID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.ClassName) || slices.Contains(forceSetValuesForFields, TrackTableClassNameColumn) {
		columns = append(columns, TrackTableClassNameColumn)

		v, err := types.FormatString(m.ClassName)
		if err != nil {
			return fmt.Errorf("failed to handle m.ClassName; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.StartedAt) || slices.Contains(forceSetValuesForFields, TrackTableStartedAtColumn) {
		columns = append(columns, TrackTableStartedAtColumn)

		v, err := types.FormatTime(m.StartedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.StartedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.EndedAt) || slices.Contains(forceSetValuesForFields, TrackTableEndedAtColumn) {
		columns = append(columns, TrackTableEndedAtColumn)

		v, err := types.FormatTime(m.EndedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.EndedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.FrameCount) || slices.Contains(forceSetValuesForFields, TrackTableFrameCountColumn) {
		columns = append(columns, TrackTableFrameCountColumn)

		v, err := types.FormatInt(m.FrameCount)
		if err != nil {
			return fmt.Errorf("failed to handle m.FrameCount; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.MeanScore) || slices.Contains(forceSetValuesForFields, TrackTableMeanScoreColumn) {
		columns = append(columns, TrackTableMeanScoreColumn)

		v, err := types.FormatFloat(m.MeanScore)
		if err != nil {
			return fmt.Errorf("failed to handle m.MeanScore; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.MaxScore) || slices.Contains(forceSetValuesForFields, TrackTableMaxScoreColumn) {
		columns = append(columns, TrackTableMaxScoreColumn)

		v, err := types.FormatFloat(m.MaxScore)
		if err != nil {
			return fmt.Errorf("failed to handle m.MaxScore; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPoint(m.Centroid) || slices.Contains(forceSetValuesForFields, TrackTableCentroidColumn) {
		columns = append(columns, TrackTableCentroidColumn)

		v, err := types.FormatPoint(m.Centroid)
		if err != nil {
			return fmt.Errorf("failed to handle m.Centroid; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPolygon(m.BoundingBox) || slices.Contains(forceSetValuesForFields, TrackTableBoundingBoxColumn) {
		columns = append(columns, TrackTableBoundingBoxColumn)

		v, err := types.FormatPolygon(m.BoundingBox)
		if err != nil {
			return fmt.Errorf("failed to handle m.BoundingBox; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroJSON(m.Trajectory) || slices.Contains(forceSetValuesForFields, TrackTableTrajectoryColumn) {
		columns = append(columns, TrackTableTrajectoryColumn)

		v, err := types.FormatJSON(m.Trajectory)
		if err != nil {
			return fmt.Errorf("failed to handle m.Trajectory; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroBool(m.IsStatic) || slices.Contains(forceSetValuesForFields, TrackTableIsStaticColumn) {
		columns = append(columns, TrackTableIsStaticColumn)

		v, err := types.FormatBool(m.IsStatic)
		if err != nil {
			return fmt.Errorf("failed to handle m.IsStatic; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Status) || slices.Contains(forceSetValuesForFields, TrackTableStatusColumn) {
		columns = append(columns, TrackTableStatusColumn)

		v, err := types.FormatString(m.Status)
		if err != nil {
			return fmt.Errorf("failed to handle m.Status; %v", err)
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
		TrackTableWithSchema,
		columns,
		fmt.Sprintf("%v = $$??", TrackTableIDColumn),
		TrackTableColumns,
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
func (m *Track) UpdateField(ctx context.Context, tx pgx.Tx, fieldName string, value any) error {
	var columnName string
	switch fieldName {

	case "id":
		columnName = TrackTableIDColumn

	case "key":
		columnName = TrackTableKeyColumn

	case "created_at":
		columnName = TrackTableCreatedAtColumn

	case "updated_at":
		columnName = TrackTableUpdatedAtColumn

	case "deleted_at":
		columnName = TrackTableDeletedAtColumn

	case "camera_id":
		columnName = TrackTableCameraIDColumn

	case "video_id":
		columnName = TrackTableVideoIDColumn

	case "class_id":
		columnName = TrackTableClassIDColumn

	case "class_name":
		columnName = TrackTableClassNameColumn

	case "started_at":
		columnName = TrackTableStartedAtColumn

	case "ended_at":
		columnName = TrackTableEndedAtColumn

	case "frame_count":
		columnName = TrackTableFrameCountColumn

	case "mean_score":
		columnName = TrackTableMeanScoreColumn

	case "max_score":
		columnName = TrackTableMaxScoreColumn

	case "centroid":
		columnName = TrackTableCentroidColumn

	case "bounding_box":
		columnName = TrackTableBoundingBoxColumn

	case "trajectory":
		columnName = TrackTableTrajectoryColumn

	case "is_static":
		columnName = TrackTableIsStaticColumn

	case "status":
		columnName = TrackTableStatusColumn
	default:
		return fmt.Errorf("unknown field name: %v", fieldName)
	}

	var columnValue any
	var err error
	switch columnName {

	case TrackTableIDColumn:
		columnValue, err = types.FormatUUID(value)

	case TrackTableKeyColumn:
		columnValue, err = types.FormatUUID(value)

	case TrackTableCreatedAtColumn:
		columnValue, err = types.FormatTime(value)

	case TrackTableUpdatedAtColumn:
		columnValue, err = types.FormatTime(value)

	case TrackTableDeletedAtColumn:
		columnValue, err = types.FormatTime(value)

	case TrackTableCameraIDColumn:
		columnValue, err = types.FormatUUID(value)

	case TrackTableVideoIDColumn:
		columnValue, err = types.FormatUUID(value)

	case TrackTableClassIDColumn:
		columnValue, err = types.FormatInt(value)

	case TrackTableClassNameColumn:
		columnValue, err = types.FormatString(value)

	case TrackTableStartedAtColumn:
		columnValue, err = types.FormatTime(value)

	case TrackTableEndedAtColumn:
		columnValue, err = types.FormatTime(value)

	case TrackTableFrameCountColumn:
		columnValue, err = types.FormatInt(value)

	case TrackTableMeanScoreColumn:
		columnValue, err = types.FormatFloat(value)

	case TrackTableMaxScoreColumn:
		columnValue, err = types.FormatFloat(value)

	case TrackTableCentroidColumn:
		columnValue, err = types.FormatPoint(value)

	case TrackTableBoundingBoxColumn:
		columnValue, err = types.FormatPolygon(value)

	case TrackTableTrajectoryColumn:
		columnValue, err = types.FormatJSON(value)

	case TrackTableIsStaticColumn:
		columnValue, err = types.FormatBool(value)

	case TrackTableStatusColumn:
		columnValue, err = types.FormatString(value)
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
		TrackTableWithSchema,
		[]string{columnName},
		fmt.Sprintf("%v = $$??", TrackTablePrimaryKeyColumn),
		[]string{TrackTablePrimaryKeyColumn},
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

func (m *Track) UpdateFields(ctx context.Context, tx pgx.Tx, fields map[string]any) error {
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
			columnName = TrackTableIDColumn

		case "key":
			columnName = TrackTableKeyColumn

		case "created_at":
			columnName = TrackTableCreatedAtColumn

		case "updated_at":
			columnName = TrackTableUpdatedAtColumn

		case "deleted_at":
			columnName = TrackTableDeletedAtColumn

		case "camera_id":
			columnName = TrackTableCameraIDColumn

		case "video_id":
			columnName = TrackTableVideoIDColumn

		case "class_id":
			columnName = TrackTableClassIDColumn

		case "class_name":
			columnName = TrackTableClassNameColumn

		case "started_at":
			columnName = TrackTableStartedAtColumn

		case "ended_at":
			columnName = TrackTableEndedAtColumn

		case "frame_count":
			columnName = TrackTableFrameCountColumn

		case "mean_score":
			columnName = TrackTableMeanScoreColumn

		case "max_score":
			columnName = TrackTableMaxScoreColumn

		case "centroid":
			columnName = TrackTableCentroidColumn

		case "bounding_box":
			columnName = TrackTableBoundingBoxColumn

		case "trajectory":
			columnName = TrackTableTrajectoryColumn

		case "is_static":
			columnName = TrackTableIsStaticColumn

		case "status":
			columnName = TrackTableStatusColumn
		default:
			return fmt.Errorf("unknown field name: %v", fieldName)
		}

		var columnValue any
		var err error
		switch columnName {

		case TrackTableIDColumn:
			columnValue, err = types.FormatUUID(value)

		case TrackTableKeyColumn:
			columnValue, err = types.FormatUUID(value)

		case TrackTableCreatedAtColumn:
			columnValue, err = types.FormatTime(value)

		case TrackTableUpdatedAtColumn:
			columnValue, err = types.FormatTime(value)

		case TrackTableDeletedAtColumn:
			columnValue, err = types.FormatTime(value)

		case TrackTableCameraIDColumn:
			columnValue, err = types.FormatUUID(value)

		case TrackTableVideoIDColumn:
			columnValue, err = types.FormatUUID(value)

		case TrackTableClassIDColumn:
			columnValue, err = types.FormatInt(value)

		case TrackTableClassNameColumn:
			columnValue, err = types.FormatString(value)

		case TrackTableStartedAtColumn:
			columnValue, err = types.FormatTime(value)

		case TrackTableEndedAtColumn:
			columnValue, err = types.FormatTime(value)

		case TrackTableFrameCountColumn:
			columnValue, err = types.FormatInt(value)

		case TrackTableMeanScoreColumn:
			columnValue, err = types.FormatFloat(value)

		case TrackTableMaxScoreColumn:
			columnValue, err = types.FormatFloat(value)

		case TrackTableCentroidColumn:
			columnValue, err = types.FormatPoint(value)

		case TrackTableBoundingBoxColumn:
			columnValue, err = types.FormatPolygon(value)

		case TrackTableTrajectoryColumn:
			columnValue, err = types.FormatJSON(value)

		case TrackTableIsStaticColumn:
			columnValue, err = types.FormatBool(value)

		case TrackTableStatusColumn:
			columnValue, err = types.FormatString(value)
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
		TrackTableWithSchema,
		columns,
		fmt.Sprintf("%v = $$??", TrackTablePrimaryKeyColumn),
		[]string{TrackTablePrimaryKeyColumn},
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

func (m *Track) Delete(ctx context.Context, tx pgx.Tx, hardDeletes ...bool) error {
	hardDelete := false
	if len(hardDeletes) > 0 {
		hardDelete = hardDeletes[0]
	}

	if !hardDelete && slices.Contains(TrackTableColumns, "deleted_at") {
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
		TrackTableWithSchema,
		fmt.Sprintf("%v = $$??", TrackTableIDColumn),
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to delete %#+v; %v", m, err)
	}

	_ = m.Reload(ctx, tx, true)

	return nil
}

func (m *Track) LockTable(ctx context.Context, tx pgx.Tx, timeouts ...time.Duration) error {
	return query.LockTable(ctx, tx, TrackTableWithSchema, timeouts...)
}

func (m *Track) LockTableWithRetries(ctx context.Context, tx pgx.Tx, overallTimeout time.Duration, individualAttempttimeout time.Duration) error {
	return query.LockTableWithRetries(ctx, tx, TrackTableWithSchema, overallTimeout, individualAttempttimeout)
}

func (m *Track) AdvisoryLock(ctx context.Context, tx pgx.Tx, key int32, timeouts ...time.Duration) error {
	return query.AdvisoryLock(ctx, tx, TrackTableNamespaceID, key, timeouts...)
}

func (m *Track) AdvisoryLockWithRetries(ctx context.Context, tx pgx.Tx, key int32, overallTimeout time.Duration, individualAttempttimeout time.Duration) error {
	return query.AdvisoryLockWithRetries(ctx, tx, TrackTableNamespaceID, key, overallTimeout, individualAttempttimeout)
}

func SelectTracks(ctx context.Context, tx pgx.Tx, where string, orderBy *string, limit *int, offset *int, values ...any) ([]*Track, int64, int64, int64, int64, error) {
	before := time.Now()

	if config.Debug() {
		log.Printf("entered SelectTracks")

		defer func() {
			log.Printf("exited SelectTracks in %s", time.Since(before))
		}()
	}
	if slices.Contains(TrackTableColumns, "deleted_at") {
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

	shouldLoad := query.ShouldLoad(ctx, TrackTable) || query.ShouldLoad(ctx, fmt.Sprintf("referenced_by_%s", TrackTable))
	shouldLoad = shouldLoad || query.ShouldLoadFromSelectionTree(ctx, TrackTable) || query.ShouldLoadFromSelectionTree(ctx, fmt.Sprintf("referenced_by_%s", TrackTable))

	var action query.PathAction
	ctx, action = query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("%s{%v}", TrackTable, nil), !isLoadQuery)
	if (action == query.PathActionCycleDetected || action == query.PathActionDepthExceeded) && !shouldLoad {
		if config.Debug() {
			log.Printf("skipping SelectTrack early (query.ShouldLoad(): %v, action: %v)", shouldLoad, action)
		}
		return []*Track{}, 0, 0, 0, 0, nil
	}

	var items *[]map[string]any
	var count int64
	var totalCount int64
	var page int64
	var totalPages int64
	var err error

	useInstead, shouldSkip := query.ShouldSkip[Track](ctx)
	if !shouldSkip {
		items, count, totalCount, page, totalPages, err = query.Select(
			ctx,
			tx,
			TrackTableColumnsWithTypeCasts,
			TrackTableWithSchema,
			where,
			orderBy,
			limit,
			offset,
			values...,
		)
		if err != nil {
			return nil, 0, 0, 0, 0, fmt.Errorf("failed to call SelectTracks; %v", err)
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

	objects := make([]*Track, 0)

	for _, item := range *items {
		var object *Track

		if !shouldSkip {
			object = &Track{}
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
				LocalColumn: TrackTableCameraIDColumn,
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
					log.Printf("loading SelectTracks->SelectCamera for object.CameraIDObject{%s: %v}", CameraTablePrimaryKeyColumn, object.CameraID)
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
					log.Printf("loaded SelectTracks->SelectCamera for object.CameraIDObject in %s", time.Since(thisBefore))
				}
			}
		}
		if !types.IsZeroUUID(object.VideoID) {
			ctx, action := query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("%s{%v}", VideoTable, object.VideoID), true)
			childNode := query.GetSelectionNode(ctx, query.RelationKey{
				Direction:   query.RelationDirectionOutbound,
				TableName:   VideoTable,
				LocalColumn: TrackTableVideoIDColumn,
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
					log.Printf("loading SelectTracks->SelectVideo for object.VideoIDObject{%s: %v}", VideoTablePrimaryKeyColumn, object.VideoID)
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
					log.Printf("loaded SelectTracks->SelectVideo for object.VideoIDObject in %s", time.Since(thisBefore))
				}
			}
		}
		err = func() error {
			childNode := query.GetSelectionNode(ctx, query.RelationKey{
				Direction:   query.RelationDirectionInbound,
				TableName:   DetectionTable,
				LocalColumn: DetectionTableTrackIDColumn,
			})
			shouldLoad := query.ShouldLoad(ctx, fmt.Sprintf("referenced_by_%s", DetectionTable))
			shouldLoad = shouldLoad || (childNode != nil && childNode.Selected) || query.ShouldLoadFromSelectionTree(ctx, fmt.Sprintf("referenced_by_%s", DetectionTable))
			ctx, action := query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("__ReferencedBy__%s{%v}", DetectionTable, object.GetPrimaryKeyValue()), true)
			switch action {
			case query.PathActionCycleDetected:
				return nil
			case query.PathActionDepthExceeded:
				if !shouldLoad {
					return nil
				}
			case query.PathActionContinue:
				if query.HasSelectionTree(ctx) && !shouldLoad {
					return nil
				}
			}

			thisBefore := time.Now()

			if config.Debug() {
				log.Printf("loading SelectTracks->SelectDetections for object.ReferencedByDetectionTrackIDObjects")
			}

			object.ReferencedByDetectionTrackIDObjects, _, _, _, _, err = SelectDetections(
				ctx,
				tx,
				fmt.Sprintf("%v = $1", DetectionTableTrackIDColumn),
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

			if config.Debug() {
				log.Printf("loaded SelectTracks->SelectDetections for object.ReferencedByDetectionTrackIDObjects in %s", time.Since(thisBefore))
			}

			return nil
		}()
		if err != nil {
			return nil, 0, 0, 0, 0, err
		}

		objects = append(objects, object)
	}

	return objects, count, totalCount, page, totalPages, nil
}

func SelectTrack(ctx context.Context, tx pgx.Tx, where string, values ...any) (*Track, int64, int64, int64, int64, error) {
	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	objects, _, _, _, _, err := SelectTracks(
		ctx,
		tx,
		where,
		nil,
		helpers.Ptr(2),
		helpers.Ptr(0),
		values...,
	)
	if err != nil {
		return nil, 0, 0, 0, 0, fmt.Errorf("failed to call SelectTrack; %v", err)
	}

	if len(objects) > 1 {
		return nil, 0, 0, 0, 0, fmt.Errorf("attempt to call SelectTrack returned more than 1 row")
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

func InsertTracks(ctx context.Context, tx pgx.Tx, objects []*Track, setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) ([]*Track, error) {
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
		TrackTableWithSchema,
		columns,
		nil,
		nil,
		nil,
		false,
		false,
		TrackTableColumns,
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to bulk insert %d objects; %v", len(objects), err)
	}

	returnedObjects := make([]*Track, 0)

	for _, item := range items {
		v := &Track{}
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

func handleGetTracks(arguments *server.SelectManyArguments, db *pgxpool.Pool) ([]*Track, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	objects, count, totalCount, page, totalPages, err := SelectTracks(arguments.Ctx, tx, arguments.Where, arguments.OrderBy, arguments.Limit, arguments.Offset, arguments.Values...)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	return objects, count, totalCount, page, totalPages, nil
}

func handleGetTrack(arguments *server.SelectOneArguments, db *pgxpool.Pool, primaryKey uuid.UUID) ([]*Track, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	object, count, totalCount, page, totalPages, err := SelectTrack(arguments.Ctx, tx, arguments.Where, arguments.Values...)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	return []*Track{object}, count, totalCount, page, totalPages, nil
}

func handlePostTrack(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, objects []*Track, forceSetValuesForFieldsByObjectIndex [][]string) ([]*Track, int64, int64, int64, int64, error) {
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

	returnedObjects, err := InsertTracks(arguments.Ctx, tx, objects, false, false, slices.Collect(maps.Keys(forceSetValuesForFieldsByObjectIndexMaximal))...)
	if err != nil {
		err = fmt.Errorf("failed to insert %d objects; %v", len(objects), err)
		return nil, 0, 0, 0, 0, err
	}

	copy(objects, returnedObjects)

	errs := make(chan error, 1)
	go func() {
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.INSERT}, TrackTable, xid)
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

func handlePutTrack(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Track) ([]*Track, int64, int64, int64, int64, error) {
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
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, TrackTable, xid)
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

	return []*Track{object}, count, totalCount, page, totalPages, nil
}

func handlePatchTrack(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Track, forceSetValuesForFields []string) ([]*Track, int64, int64, int64, int64, error) {
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
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, TrackTable, xid)
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

	return []*Track{object}, count, totalCount, page, totalPages, nil
}

func handleDeleteTrack(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Track) error {
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
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.DELETE, stream.SOFT_DELETE}, TrackTable, xid)
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

func MutateRouterForTrack(r chi.Router, db *pgxpool.Pool, redisPool *redis.Pool, objectMiddlewares []server.ObjectMiddleware, waitForChange server.WaitForChange) {

	func() {
		getManyHandler, err := getHTTPHandler(
			http.MethodGet,
			"/tracks",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams map[string]any,
				req server.EmptyRequest,
				rawReq any,
			) (server.Response[Track], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectManyArguments(ctx, queryParams, TrackIntrospectedTable, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache not yet reached; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Track]{}, err
				}

				cachedResponseAsJSON, cacheHit, err := server.GetCachedResponseAsJSON(arguments.RequestHash, redisConn)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache failed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Track]{}, err
				}

				if cacheHit {
					var cachedResponse server.Response[Track]

					/* TODO: it'd be nice to be able to avoid this (i.e. just pass straight through) */
					err = json.Unmarshal(cachedResponseAsJSON, &cachedResponse)
					if err != nil {
						if config.Debug() {
							log.Printf("request cache hit but failed unmarshal; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
						}

						return server.Response[Track]{}, err
					}

					if config.Debug() {
						log.Printf("request cache hit; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return cachedResponse, nil
				}

				objects, count, totalCount, _, _, err := handleGetTracks(arguments, db)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Track]{}, err
				}

				limit := int64(0)
				if arguments.Limit != nil {
					limit = int64(*arguments.Limit)
				}

				offset := int64(0)
				if arguments.Offset != nil {
					offset = int64(*arguments.Offset)
				}

				response := server.Response[Track]{
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

					return server.Response[Track]{}, err
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
			Track{},
			TrackIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Get(getManyHandler.FullPath, getManyHandler.ServeHTTP)
	}()

	func() {
		getOneHandler, err := getHTTPHandler(
			http.MethodGet,
			"/tracks/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams TrackOnePathParams,
				queryParams TrackLoadQueryParams,
				req server.EmptyRequest,
				rawReq any,
			) (server.Response[Track], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectOneArguments(ctx, queryParams.Depth, TrackIntrospectedTable, pathParams.PrimaryKey, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache not yet reached; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Track]{}, err
				}

				cachedResponseAsJSON, cacheHit, err := server.GetCachedResponseAsJSON(arguments.RequestHash, redisConn)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache failed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Track]{}, err
				}

				if cacheHit {
					var cachedResponse server.Response[Track]

					/* TODO: it'd be nice to be able to avoid this (i.e. just pass straight through) */
					err = json.Unmarshal(cachedResponseAsJSON, &cachedResponse)
					if err != nil {
						if config.Debug() {
							log.Printf("request cache hit but failed unmarshal; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
						}

						return server.Response[Track]{}, err
					}

					if config.Debug() {
						log.Printf("request cache hit; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return cachedResponse, nil
				}

				objects, count, totalCount, _, _, err := handleGetTrack(arguments, db, pathParams.PrimaryKey)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Track]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				response := server.Response[Track]{
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

					return server.Response[Track]{}, err
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
			Track{},
			TrackIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Get(getOneHandler.FullPath, getOneHandler.ServeHTTP)
	}()

	func() {
		postHandler, err := getHTTPHandler(
			http.MethodPost,
			"/tracks",
			http.StatusCreated,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams TrackLoadQueryParams,
				req []*Track,
				rawReq any,
			) (server.Response[Track], error) {
				allRawItems, ok := rawReq.([]any)
				if !ok {
					return server.Response[Track]{}, fmt.Errorf("failed to cast %#+v to []map[string]any", rawReq)
				}

				allItems := make([]map[string]any, 0)
				for _, rawItem := range allRawItems {
					item, ok := rawItem.(map[string]any)
					if !ok {
						return server.Response[Track]{}, fmt.Errorf("failed to cast %#+v to map[string]any", rawItem)
					}

					allItems = append(allItems, item)
				}

				forceSetValuesForFieldsByObjectIndex := make([][]string, 0)
				for _, item := range allItems {
					forceSetValuesForFields := make([]string, 0)
					for _, possibleField := range slices.Collect(maps.Keys(item)) {
						if !slices.Contains(TrackTableColumns, possibleField) {
							continue
						}

						forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
					}
					forceSetValuesForFieldsByObjectIndex = append(forceSetValuesForFieldsByObjectIndex, forceSetValuesForFields)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[Track]{}, err
				}

				objects, count, totalCount, _, _, err := handlePostTrack(arguments, db, waitForChange, req, forceSetValuesForFieldsByObjectIndex)
				if err != nil {
					return server.Response[Track]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[Track]{
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
			Track{},
			TrackIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Post(postHandler.FullPath, postHandler.ServeHTTP)
	}()

	func() {
		putHandler, err := getHTTPHandler(
			http.MethodPatch,
			"/tracks/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams TrackOnePathParams,
				queryParams TrackLoadQueryParams,
				req Track,
				rawReq any,
			) (server.Response[Track], error) {
				item, ok := rawReq.(map[string]any)
				if !ok {
					return server.Response[Track]{}, fmt.Errorf("failed to cast %#+v to map[string]any", item)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[Track]{}, err
				}

				object := &req
				object.ID = pathParams.PrimaryKey

				objects, count, totalCount, _, _, err := handlePutTrack(arguments, db, waitForChange, object)
				if err != nil {
					return server.Response[Track]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[Track]{
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
			Track{},
			TrackIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Put(putHandler.FullPath, putHandler.ServeHTTP)
	}()

	func() {
		patchHandler, err := getHTTPHandler(
			http.MethodPatch,
			"/tracks/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams TrackOnePathParams,
				queryParams TrackLoadQueryParams,
				req Track,
				rawReq any,
			) (server.Response[Track], error) {
				item, ok := rawReq.(map[string]any)
				if !ok {
					return server.Response[Track]{}, fmt.Errorf("failed to cast %#+v to map[string]any", item)
				}

				forceSetValuesForFields := make([]string, 0)
				for _, possibleField := range slices.Collect(maps.Keys(item)) {
					if !slices.Contains(TrackTableColumns, possibleField) {
						continue
					}

					forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[Track]{}, err
				}

				object := &req
				object.ID = pathParams.PrimaryKey

				objects, count, totalCount, _, _, err := handlePatchTrack(arguments, db, waitForChange, object, forceSetValuesForFields)
				if err != nil {
					return server.Response[Track]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[Track]{
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
			Track{},
			TrackIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Patch(patchHandler.FullPath, patchHandler.ServeHTTP)
	}()

	func() {
		deleteHandler, err := getHTTPHandler(
			http.MethodDelete,
			"/tracks/{primaryKey}",
			http.StatusNoContent,
			func(
				ctx context.Context,
				pathParams TrackOnePathParams,
				queryParams TrackLoadQueryParams,
				req server.EmptyRequest,
				rawReq any,
			) (server.EmptyResponse, error) {
				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.EmptyResponse{}, err
				}

				object := &Track{}
				object.ID = pathParams.PrimaryKey

				err = handleDeleteTrack(arguments, db, waitForChange, object)
				if err != nil {
					return server.EmptyResponse{}, err
				}

				return server.EmptyResponse{}, nil
			},
			Track{},
			TrackIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Delete(deleteHandler.FullPath, deleteHandler.ServeHTTP)
	}()
}

func NewTrackFromItem(item map[string]any) (any, error) {
	object := &Track{}

	err := object.FromItem(item)
	if err != nil {
		return nil, err
	}

	return object, nil
}

func init() {
	register(
		TrackTable,
		Track{},
		NewTrackFromItem,
		"/tracks",
		MutateRouterForTrack,
	)
}
