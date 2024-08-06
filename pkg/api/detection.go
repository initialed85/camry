package api

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/netip"
	"slices"
	"strconv"
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
	"github.com/initialed85/djangolang/pkg/types"
	_pgtype "github.com/jackc/pgtype"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/jmoiron/sqlx"
	"github.com/lib/pq"
	"github.com/lib/pq/hstore"
	"github.com/paulmach/orb/geojson"
	"golang.org/x/exp/maps"
)

type Detection struct {
	ID             uuid.UUID     `json:"id"`
	CreatedAt      time.Time     `json:"created_at"`
	UpdatedAt      time.Time     `json:"updated_at"`
	DeletedAt      *time.Time    `json:"deleted_at"`
	SeenAt         time.Time     `json:"seen_at"`
	ClassID        int64         `json:"class_id"`
	ClassName      string        `json:"class_name"`
	Score          float64       `json:"score"`
	Centroid       pgtype.Vec2   `json:"centroid"`
	BoundingBox    []pgtype.Vec2 `json:"bounding_box"`
	CameraID       uuid.UUID     `json:"camera_id"`
	CameraIDObject *Camera       `json:"camera_id_object"`
}

var DetectionTable = "detection"

var (
	DetectionTableIDColumn          = "id"
	DetectionTableCreatedAtColumn   = "created_at"
	DetectionTableUpdatedAtColumn   = "updated_at"
	DetectionTableDeletedAtColumn   = "deleted_at"
	DetectionTableSeenAtColumn      = "seen_at"
	DetectionTableClassIDColumn     = "class_id"
	DetectionTableClassNameColumn   = "class_name"
	DetectionTableScoreColumn       = "score"
	DetectionTableCentroidColumn    = "centroid"
	DetectionTableBoundingBoxColumn = "bounding_box"
	DetectionTableCameraIDColumn    = "camera_id"
)

var (
	DetectionTableIDColumnWithTypeCast          = fmt.Sprintf(`"id" AS id`)
	DetectionTableCreatedAtColumnWithTypeCast   = fmt.Sprintf(`"created_at" AS created_at`)
	DetectionTableUpdatedAtColumnWithTypeCast   = fmt.Sprintf(`"updated_at" AS updated_at`)
	DetectionTableDeletedAtColumnWithTypeCast   = fmt.Sprintf(`"deleted_at" AS deleted_at`)
	DetectionTableSeenAtColumnWithTypeCast      = fmt.Sprintf(`"seen_at" AS seen_at`)
	DetectionTableClassIDColumnWithTypeCast     = fmt.Sprintf(`"class_id" AS class_id`)
	DetectionTableClassNameColumnWithTypeCast   = fmt.Sprintf(`"class_name" AS class_name`)
	DetectionTableScoreColumnWithTypeCast       = fmt.Sprintf(`"score" AS score`)
	DetectionTableCentroidColumnWithTypeCast    = fmt.Sprintf(`"centroid" AS centroid`)
	DetectionTableBoundingBoxColumnWithTypeCast = fmt.Sprintf(`"bounding_box" AS bounding_box`)
	DetectionTableCameraIDColumnWithTypeCast    = fmt.Sprintf(`"camera_id" AS camera_id`)
)

var DetectionTableColumns = []string{
	DetectionTableIDColumn,
	DetectionTableCreatedAtColumn,
	DetectionTableUpdatedAtColumn,
	DetectionTableDeletedAtColumn,
	DetectionTableSeenAtColumn,
	DetectionTableClassIDColumn,
	DetectionTableClassNameColumn,
	DetectionTableScoreColumn,
	DetectionTableCentroidColumn,
	DetectionTableBoundingBoxColumn,
	DetectionTableCameraIDColumn,
}

var DetectionTableColumnsWithTypeCasts = []string{
	DetectionTableIDColumnWithTypeCast,
	DetectionTableCreatedAtColumnWithTypeCast,
	DetectionTableUpdatedAtColumnWithTypeCast,
	DetectionTableDeletedAtColumnWithTypeCast,
	DetectionTableSeenAtColumnWithTypeCast,
	DetectionTableClassIDColumnWithTypeCast,
	DetectionTableClassNameColumnWithTypeCast,
	DetectionTableScoreColumnWithTypeCast,
	DetectionTableCentroidColumnWithTypeCast,
	DetectionTableBoundingBoxColumnWithTypeCast,
	DetectionTableCameraIDColumnWithTypeCast,
}

var DetectionTableColumnLookup = map[string]*introspect.Column{
	DetectionTableIDColumn:          new(introspect.Column),
	DetectionTableCreatedAtColumn:   new(introspect.Column),
	DetectionTableUpdatedAtColumn:   new(introspect.Column),
	DetectionTableDeletedAtColumn:   new(introspect.Column),
	DetectionTableSeenAtColumn:      new(introspect.Column),
	DetectionTableClassIDColumn:     new(introspect.Column),
	DetectionTableClassNameColumn:   new(introspect.Column),
	DetectionTableScoreColumn:       new(introspect.Column),
	DetectionTableCentroidColumn:    new(introspect.Column),
	DetectionTableBoundingBoxColumn: new(introspect.Column),
	DetectionTableCameraIDColumn:    new(introspect.Column),
}

var (
	DetectionTablePrimaryKeyColumn = DetectionTableIDColumn
)

var (
	_ = time.Time{}
	_ = uuid.UUID{}
	_ = pq.StringArray{}
	_ = hstore.Hstore{}
	_ = geojson.Point{}
	_ = pgtype.Point{}
	_ = _pgtype.Point{}
	_ = postgis.PointZ{}
	_ = netip.Prefix{}
)

func (m *Detection) GetPrimaryKeyColumn() string {
	return DetectionTablePrimaryKeyColumn
}

func (m *Detection) GetPrimaryKeyValue() any {
	return m.ID
}

func (m *Detection) FromItem(item map[string]any) error {
	if item == nil {
		return fmt.Errorf(
			"item unexpectedly nil during DetectionFromItem",
		)
	}

	if len(item) == 0 {
		return fmt.Errorf(
			"item unexpectedly empty during DetectionFromItem",
		)
	}

	wrapError := func(k string, v any, err error) error {
		return fmt.Errorf("%v: %#+v; error: %v", k, v, err)
	}

	for k, v := range item {
		_, ok := DetectionTableColumnLookup[k]
		if !ok {
			return fmt.Errorf(
				"item contained unexpected key %#+v during DetectionFromItem; item: %#+v",
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to time.Time", temp1))
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to time.Time", temp1))
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to time.Time", temp1))
				}
			}

			m.DeletedAt = &temp2

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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to time.Time", temp1))
				}
			}

			m.SeenAt = temp2

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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to int64", temp1))
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to string", temp1))
				}
			}

			m.ClassName = temp2

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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to float64", temp1))
				}
			}

			m.Score = temp2

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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to pgtype.Vec2", temp1))
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to []pgtype.Vec2", temp1))
				}
			}

			m.BoundingBox = temp2

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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuid.UUID", temp1))
				}
			}

			m.CameraID = temp2

		}
	}

	return nil
}

func (m *Detection) Reload(
	ctx context.Context,
	tx *sqlx.Tx,
	includeDeleteds ...bool,
) error {
	extraWhere := ""
	if len(includeDeleteds) > 0 && includeDeleteds[0] {
		if slices.Contains(DetectionTableColumns, "deleted_at") {
			extraWhere = "\n    AND (deleted_at IS null OR deleted_at IS NOT null)"
		}
	}

	t, err := SelectDetection(
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
	m.SeenAt = t.SeenAt
	m.ClassID = t.ClassID
	m.ClassName = t.ClassName
	m.Score = t.Score
	m.Centroid = t.Centroid
	m.BoundingBox = t.BoundingBox
	m.CameraID = t.CameraID
	m.CameraIDObject = t.CameraIDObject

	return nil
}

func (m *Detection) Insert(
	ctx context.Context,
	tx *sqlx.Tx,
	setPrimaryKey bool,
	setZeroValues bool,
) error {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setPrimaryKey && (setZeroValues || !types.IsZeroUUID(m.ID)) {
		columns = append(columns, DetectionTableIDColumn)

		v, err := types.FormatUUID(m.ID)
		if err != nil {
			return fmt.Errorf("failed to handle m.ID: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) {
		columns = append(columns, DetectionTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.CreatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) {
		columns = append(columns, DetectionTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.UpdatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) {
		columns = append(columns, DetectionTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.DeletedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.SeenAt) {
		columns = append(columns, DetectionTableSeenAtColumn)

		v, err := types.FormatTime(m.SeenAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.SeenAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.ClassID) {
		columns = append(columns, DetectionTableClassIDColumn)

		v, err := types.FormatInt(m.ClassID)
		if err != nil {
			return fmt.Errorf("failed to handle m.ClassID: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.ClassName) {
		columns = append(columns, DetectionTableClassNameColumn)

		v, err := types.FormatString(m.ClassName)
		if err != nil {
			return fmt.Errorf("failed to handle m.ClassName: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.Score) {
		columns = append(columns, DetectionTableScoreColumn)

		v, err := types.FormatFloat(m.Score)
		if err != nil {
			return fmt.Errorf("failed to handle m.Score: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPoint(m.Centroid) {
		columns = append(columns, DetectionTableCentroidColumn)

		v, err := types.FormatPoint(m.Centroid)
		if err != nil {
			return fmt.Errorf("failed to handle m.Centroid: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPolygon(m.BoundingBox) {
		columns = append(columns, DetectionTableBoundingBoxColumn)

		v, err := types.FormatPolygon(m.BoundingBox)
		if err != nil {
			return fmt.Errorf("failed to handle m.BoundingBox: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) {
		columns = append(columns, DetectionTableCameraIDColumn)

		v, err := types.FormatUUID(m.CameraID)
		if err != nil {
			return fmt.Errorf("failed to handle m.CameraID: %v", err)
		}

		values = append(values, v)
	}

	item, err := query.Insert(
		ctx,
		tx,
		DetectionTable,
		columns,
		nil,
		false,
		false,
		DetectionTableColumns,
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to insert %#+v: %v", m, err)
	}
	v := item[DetectionTableIDColumn]

	if v == nil {
		return fmt.Errorf("failed to find %v in %#+v", DetectionTableIDColumn, item)
	}

	wrapError := func(err error) error {
		return fmt.Errorf(
			"failed to treat %v: %#+v as uuid.UUID: %v",
			DetectionTableIDColumn,
			item[DetectionTableIDColumn],
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

	err = m.Reload(ctx, tx)
	if err != nil {
		return fmt.Errorf("failed to reload after insert")
	}

	return nil
}

func (m *Detection) Update(
	ctx context.Context,
	tx *sqlx.Tx,
	setZeroValues bool,
	forceSetValuesForFields ...string,
) error {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, DetectionTableCreatedAtColumn) {
		columns = append(columns, DetectionTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.CreatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, DetectionTableUpdatedAtColumn) {
		columns = append(columns, DetectionTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.UpdatedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, DetectionTableDeletedAtColumn) {
		columns = append(columns, DetectionTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.DeletedAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.SeenAt) || slices.Contains(forceSetValuesForFields, DetectionTableSeenAtColumn) {
		columns = append(columns, DetectionTableSeenAtColumn)

		v, err := types.FormatTime(m.SeenAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.SeenAt: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroInt(m.ClassID) || slices.Contains(forceSetValuesForFields, DetectionTableClassIDColumn) {
		columns = append(columns, DetectionTableClassIDColumn)

		v, err := types.FormatInt(m.ClassID)
		if err != nil {
			return fmt.Errorf("failed to handle m.ClassID: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.ClassName) || slices.Contains(forceSetValuesForFields, DetectionTableClassNameColumn) {
		columns = append(columns, DetectionTableClassNameColumn)

		v, err := types.FormatString(m.ClassName)
		if err != nil {
			return fmt.Errorf("failed to handle m.ClassName: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroFloat(m.Score) || slices.Contains(forceSetValuesForFields, DetectionTableScoreColumn) {
		columns = append(columns, DetectionTableScoreColumn)

		v, err := types.FormatFloat(m.Score)
		if err != nil {
			return fmt.Errorf("failed to handle m.Score: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPoint(m.Centroid) || slices.Contains(forceSetValuesForFields, DetectionTableCentroidColumn) {
		columns = append(columns, DetectionTableCentroidColumn)

		v, err := types.FormatPoint(m.Centroid)
		if err != nil {
			return fmt.Errorf("failed to handle m.Centroid: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroPolygon(m.BoundingBox) || slices.Contains(forceSetValuesForFields, DetectionTableBoundingBoxColumn) {
		columns = append(columns, DetectionTableBoundingBoxColumn)

		v, err := types.FormatPolygon(m.BoundingBox)
		if err != nil {
			return fmt.Errorf("failed to handle m.BoundingBox: %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroUUID(m.CameraID) || slices.Contains(forceSetValuesForFields, DetectionTableCameraIDColumn) {
		columns = append(columns, DetectionTableCameraIDColumn)

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

	_, err = query.Update(
		ctx,
		tx,
		DetectionTable,
		columns,
		fmt.Sprintf("%v = $$??", DetectionTableIDColumn),
		DetectionTableColumns,
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to update %#+v: %v", m, err)
	}

	err = m.Reload(ctx, tx, slices.Contains(forceSetValuesForFields, "deleted_at"))
	if err != nil {
		return fmt.Errorf("failed to reload after update")
	}

	return nil
}

func (m *Detection) Delete(
	ctx context.Context,
	tx *sqlx.Tx,
	hardDeletes ...bool,
) error {
	hardDelete := false
	if len(hardDeletes) > 0 {
		hardDelete = hardDeletes[0]
	}

	if !hardDelete && slices.Contains(DetectionTableColumns, "deleted_at") {
		m.DeletedAt = helpers.Ptr(time.Now().UTC())
		err := m.Update(ctx, tx, false, "deleted_at")
		if err != nil {
			return fmt.Errorf("failed to soft-delete (update) %#+v: %v", m, err)
		}
	}

	values := make([]any, 0)
	v, err := types.FormatUUID(m.ID)
	if err != nil {
		return fmt.Errorf("failed to handle m.ID: %v", err)
	}

	values = append(values, v)

	err = query.Delete(
		ctx,
		tx,
		DetectionTable,
		fmt.Sprintf("%v = $$??", DetectionTableIDColumn),
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to delete %#+v: %v", m, err)
	}

	_ = m.Reload(ctx, tx, true)

	return nil
}

func SelectDetections(
	ctx context.Context,
	tx *sqlx.Tx,
	where string,
	orderBy *string,
	limit *int,
	offset *int,
	values ...any,
) ([]*Detection, error) {
	if slices.Contains(DetectionTableColumns, "deleted_at") {
		if !strings.Contains(where, "deleted_at") {
			if where != "" {
				where += "\n    AND "
			}

			where += "deleted_at IS null"
		}
	}

	items, err := query.Select(
		ctx,
		tx,
		DetectionTableColumnsWithTypeCasts,
		DetectionTable,
		where,
		orderBy,
		limit,
		offset,
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to call SelectDetections; err: %v", err)
	}

	objects := make([]*Detection, 0)

	for _, item := range items {
		object := &Detection{}

		err = object.FromItem(item)
		if err != nil {
			return nil, fmt.Errorf("failed to call Detection.FromItem; err: %v", err)
		}

		if !types.IsZeroUUID(object.CameraID) {
			object.CameraIDObject, _ = SelectCamera(
				ctx,
				tx,
				fmt.Sprintf("%v = $1", CameraTablePrimaryKeyColumn),
				object.CameraID,
			)
		}

		objects = append(objects, object)
	}

	return objects, nil
}

func SelectDetection(
	ctx context.Context,
	tx *sqlx.Tx,
	where string,
	values ...any,
) (*Detection, error) {
	objects, err := SelectDetections(
		ctx,
		tx,
		where,
		nil,
		helpers.Ptr(2),
		helpers.Ptr(0),
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to call SelectDetection; err: %v", err)
	}

	if len(objects) > 1 {
		return nil, fmt.Errorf("attempt to call SelectDetection returned more than 1 row")
	}

	if len(objects) < 1 {
		return nil, fmt.Errorf("attempt to call SelectDetection returned no rows")
	}

	object := objects[0]

	return object, nil
}

func handleGetDetections(w http.ResponseWriter, r *http.Request, db *sqlx.DB, redisConn redis.Conn, modelMiddlewares []server.ModelMiddleware) {
	ctx := r.Context()

	insaneOrderParams := make([]string, 0)
	hadInsaneOrderParams := false

	unrecognizedParams := make([]string, 0)
	hadUnrecognizedParams := false

	unparseableParams := make([]string, 0)
	hadUnparseableParams := false

	var orderByDirection *string
	orderBys := make([]string, 0)
	for rawKey, rawValues := range r.URL.Query() {
		if !(rawKey == "order_by__desc" || rawKey == "order_by__asc") {
			continue
		}

		for _, rawValue := range rawValues {
			switch rawKey {
			case "order_by__desc":
				if orderByDirection != nil && *orderByDirection != "DESC" {
					insaneOrderParams = append(insaneOrderParams, fmt.Sprintf("%s=%s", rawKey, rawValue))
					hadInsaneOrderParams = true
					continue
				}

				orderByDirection = helpers.Ptr("DESC")
				orderBys = append(orderBys, strings.Split(rawValue, ",")...)
			case "order_by__asc":
				if orderByDirection != nil && *orderByDirection != "ASC" {
					insaneOrderParams = append(insaneOrderParams, fmt.Sprintf("%s=%s", rawKey, rawValue))
					hadInsaneOrderParams = true
					continue
				}

				orderByDirection = helpers.Ptr("ASC")
				orderBys = append(orderBys, strings.Split(rawValue, ",")...)
			}
		}
	}

	if hadInsaneOrderParams {
		helpers.HandleErrorResponse(
			w,
			http.StatusInternalServerError,
			fmt.Errorf("insane order params (e.g. conflicting asc / desc) %s", strings.Join(insaneOrderParams, ", ")),
		)
		return
	}

	values := make([]any, 0)
	wheres := make([]string, 0)
	for rawKey, rawValues := range r.URL.Query() {
		if rawKey == "limit" || rawKey == "offset" || rawKey == "order_by__desc" || rawKey == "order_by__asc" {
			continue
		}

		parts := strings.Split(rawKey, "__")
		isUnrecognized := len(parts) != 2

		comparison := ""
		isSliceComparison := false
		isNullComparison := false
		IsLikeComparison := false

		if !isUnrecognized {
			column := DetectionTableColumnLookup[parts[0]]
			if column == nil {
				isUnrecognized = true
			} else {
				switch parts[1] {
				case "eq":
					comparison = "="
				case "ne":
					comparison = "!="
				case "gt":
					comparison = ">"
				case "gte":
					comparison = ">="
				case "lt":
					comparison = "<"
				case "lte":
					comparison = "<="
				case "in":
					comparison = "IN"
					isSliceComparison = true
				case "nin", "notin":
					comparison = "NOT IN"
					isSliceComparison = true
				case "isnull":
					comparison = "IS NULL"
					isNullComparison = true
				case "nisnull", "isnotnull":
					comparison = "IS NOT NULL"
					isNullComparison = true
				case "l", "like":
					comparison = "LIKE"
					IsLikeComparison = true
				case "nl", "nlike", "notlike":
					comparison = "NOT LIKE"
					IsLikeComparison = true
				case "il", "ilike":
					comparison = "ILIKE"
					IsLikeComparison = true
				case "nil", "nilike", "notilike":
					comparison = "NOT ILIKE"
					IsLikeComparison = true
				default:
					isUnrecognized = true
				}
			}
		}

		if isNullComparison {
			wheres = append(wheres, fmt.Sprintf("%s %s", parts[0], comparison))
			continue
		}

		for _, rawValue := range rawValues {
			if isUnrecognized {
				unrecognizedParams = append(unrecognizedParams, fmt.Sprintf("%s=%s", rawKey, rawValue))
				hadUnrecognizedParams = true
				continue
			}

			if hadUnrecognizedParams {
				continue
			}

			attempts := make([]string, 0)

			if !IsLikeComparison {
				attempts = append(attempts, rawValue)
			}

			if isSliceComparison {
				attempts = append(attempts, fmt.Sprintf("[%s]", rawValue))

				vs := make([]string, 0)
				for _, v := range strings.Split(rawValue, ",") {
					vs = append(vs, fmt.Sprintf("\"%s\"", v))
				}

				attempts = append(attempts, fmt.Sprintf("[%s]", strings.Join(vs, ",")))
			}

			if IsLikeComparison {
				attempts = append(attempts, fmt.Sprintf("\"%%%s%%\"", rawValue))
			} else {
				attempts = append(attempts, fmt.Sprintf("\"%s\"", rawValue))
			}

			var err error

			for _, attempt := range attempts {
				var value any
				err = json.Unmarshal([]byte(attempt), &value)
				if err == nil {
					if isSliceComparison {
						sliceValues, ok := value.([]any)
						if !ok {
							err = fmt.Errorf("failed to cast %#+v to []string", value)
							break
						}

						values = append(values, sliceValues...)

						sliceWheres := make([]string, 0)
						for range values {
							sliceWheres = append(sliceWheres, "$$??")
						}

						wheres = append(wheres, fmt.Sprintf("%s %s (%s)", parts[0], comparison, strings.Join(sliceWheres, ", ")))
					} else {
						values = append(values, value)
						wheres = append(wheres, fmt.Sprintf("%s %s $$??", parts[0], comparison))
					}

					break
				}
			}

			if err != nil {
				unparseableParams = append(unparseableParams, fmt.Sprintf("%s=%s", rawKey, rawValue))
				hadUnparseableParams = true
				continue
			}
		}
	}

	if hadUnrecognizedParams {
		helpers.HandleErrorResponse(
			w,
			http.StatusInternalServerError,
			fmt.Errorf("unrecognized params %s", strings.Join(unrecognizedParams, ", ")),
		)
		return
	}

	if hadUnparseableParams {
		helpers.HandleErrorResponse(
			w,
			http.StatusInternalServerError,
			fmt.Errorf("unparseable params %s", strings.Join(unparseableParams, ", ")),
		)
		return
	}

	limit := 2000
	rawLimit := r.URL.Query().Get("limit")
	if rawLimit != "" {
		possibleLimit, err := strconv.ParseInt(rawLimit, 10, 64)
		if err != nil {
			helpers.HandleErrorResponse(
				w,
				http.StatusInternalServerError,
				fmt.Errorf("failed to parse param limit=%s as int: %v", rawLimit, err),
			)
			return
		}

		limit = int(possibleLimit)
	}

	offset := 0
	rawOffset := r.URL.Query().Get("offset")
	if rawOffset != "" {
		possibleOffset, err := strconv.ParseInt(rawOffset, 10, 64)
		if err != nil {
			helpers.HandleErrorResponse(
				w,
				http.StatusInternalServerError,
				fmt.Errorf("failed to parse param offset=%s as int: %v", rawOffset, err),
			)
			return
		}

		offset = int(possibleOffset)
	}

	hashableOrderBy := ""
	var orderBy *string
	if len(orderBys) > 0 {
		hashableOrderBy = strings.Join(orderBys, ", ")
		if len(orderBys) > 1 {
			hashableOrderBy = fmt.Sprintf("(%v)", hashableOrderBy)
		}
		hashableOrderBy = fmt.Sprintf("%v %v", hashableOrderBy, *orderByDirection)
		orderBy = &hashableOrderBy
	}

	requestHash, err := helpers.GetRequestHash(DetectionTable, wheres, hashableOrderBy, limit, offset, values, nil)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	cacheHit, err := helpers.AttemptCachedResponse(requestHash, redisConn, w)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	if cacheHit {
		return
	}

	tx, err := db.BeginTxx(r.Context(), nil)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	defer func() {
		_ = tx.Rollback()
	}()

	where := strings.Join(wheres, "\n    AND ")

	objects, err := SelectDetections(ctx, tx, where, orderBy, &limit, &offset, values...)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	err = tx.Commit()
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	returnedObjectsAsJSON := helpers.HandleObjectsResponse(w, http.StatusOK, objects)

	err = helpers.StoreCachedResponse(requestHash, redisConn, string(returnedObjectsAsJSON))
	if err != nil {
		log.Printf("warning: %v", err)
	}
}

func handleGetDetection(w http.ResponseWriter, r *http.Request, db *sqlx.DB, redisConn redis.Conn, modelMiddlewares []server.ModelMiddleware, primaryKey string) {
	ctx := r.Context()

	wheres := []string{fmt.Sprintf("%s = $$??", DetectionTablePrimaryKeyColumn)}
	values := []any{primaryKey}

	requestHash, err := helpers.GetRequestHash(DetectionTable, wheres, "", 2, 0, values, primaryKey)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	cacheHit, err := helpers.AttemptCachedResponse(requestHash, redisConn, w)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	if cacheHit {
		return
	}

	tx, err := db.BeginTxx(r.Context(), nil)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	defer func() {
		_ = tx.Rollback()
	}()

	where := strings.Join(wheres, "\n    AND ")

	object, err := SelectDetection(ctx, tx, where, values...)
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	err = tx.Commit()
	if err != nil {
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	returnedObjectsAsJSON := helpers.HandleObjectsResponse(w, http.StatusOK, []*Detection{object})

	err = helpers.StoreCachedResponse(requestHash, redisConn, string(returnedObjectsAsJSON))
	if err != nil {
		log.Printf("warning: %v", err)
	}
}

func handlePostDetections(w http.ResponseWriter, r *http.Request, db *sqlx.DB, redisConn redis.Conn, modelMiddlewares []server.ModelMiddleware) {
	_ = redisConn

	b, err := io.ReadAll(r.Body)
	if err != nil {
		err = fmt.Errorf("failed to read body of HTTP request: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	var allItems []map[string]any
	err = json.Unmarshal(b, &allItems)
	if err != nil {
		err = fmt.Errorf("failed to unmarshal %#+v as JSON list of objects: %v", string(b), err)
		helpers.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	objects := make([]*Detection, 0)
	for _, item := range allItems {
		object := &Detection{}
		err = object.FromItem(item)
		if err != nil {
			err = fmt.Errorf("failed to interpret %#+v as Detection in item form: %v", item, err)
			helpers.HandleErrorResponse(w, http.StatusBadRequest, err)
			return
		}

		objects = append(objects, object)
	}

	tx, err := db.BeginTxx(r.Context(), nil)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	defer func() {
		_ = tx.Rollback()
	}()

	for i, object := range objects {
		err = object.Insert(r.Context(), tx, false, false)
		if err != nil {
			err = fmt.Errorf("failed to insert %#+v: %v", object, err)
			helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
			return
		}

		objects[i] = object
	}

	err = tx.Commit()
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	helpers.HandleObjectsResponse(w, http.StatusCreated, objects)
}

func handlePutDetection(w http.ResponseWriter, r *http.Request, db *sqlx.DB, redisConn redis.Conn, modelMiddlewares []server.ModelMiddleware, primaryKey string) {
	_ = redisConn

	b, err := io.ReadAll(r.Body)
	if err != nil {
		err = fmt.Errorf("failed to read body of HTTP request: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	var item map[string]any
	err = json.Unmarshal(b, &item)
	if err != nil {
		err = fmt.Errorf("failed to unmarshal %#+v as JSON object: %v", string(b), err)
		helpers.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	item[DetectionTablePrimaryKeyColumn] = primaryKey

	object := &Detection{}
	err = object.FromItem(item)
	if err != nil {
		err = fmt.Errorf("failed to interpret %#+v as Detection in item form: %v", item, err)
		helpers.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	tx, err := db.BeginTxx(r.Context(), nil)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	defer func() {
		_ = tx.Rollback()
	}()

	err = object.Update(r.Context(), tx, true)
	if err != nil {
		err = fmt.Errorf("failed to update %#+v: %v", object, err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	err = tx.Commit()
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	helpers.HandleObjectsResponse(w, http.StatusOK, []*Detection{object})
}

func handlePatchDetection(w http.ResponseWriter, r *http.Request, db *sqlx.DB, redisConn redis.Conn, modelMiddlewares []server.ModelMiddleware, primaryKey string) {
	_ = redisConn

	b, err := io.ReadAll(r.Body)
	if err != nil {
		err = fmt.Errorf("failed to read body of HTTP request: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	var item map[string]any
	err = json.Unmarshal(b, &item)
	if err != nil {
		err = fmt.Errorf("failed to unmarshal %#+v as JSON object: %v", string(b), err)
		helpers.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	forceSetValuesForFields := make([]string, 0)
	for _, possibleField := range maps.Keys(item) {
		if !slices.Contains(DetectionTableColumns, possibleField) {
			continue
		}

		forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
	}

	item[DetectionTablePrimaryKeyColumn] = primaryKey

	object := &Detection{}
	err = object.FromItem(item)
	if err != nil {
		err = fmt.Errorf("failed to interpret %#+v as Detection in item form: %v", item, err)
		helpers.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	tx, err := db.BeginTxx(r.Context(), nil)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	defer func() {
		_ = tx.Rollback()
	}()

	err = object.Update(r.Context(), tx, false, forceSetValuesForFields...)
	if err != nil {
		err = fmt.Errorf("failed to update %#+v: %v", object, err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	err = tx.Commit()
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	helpers.HandleObjectsResponse(w, http.StatusOK, []*Detection{object})
}

func handleDeleteDetection(w http.ResponseWriter, r *http.Request, db *sqlx.DB, redisConn redis.Conn, modelMiddlewares []server.ModelMiddleware, primaryKey string) {
	_ = redisConn

	var item = make(map[string]any)

	item[DetectionTablePrimaryKeyColumn] = primaryKey

	object := &Detection{}
	err := object.FromItem(item)
	if err != nil {
		err = fmt.Errorf("failed to interpret %#+v as Detection in item form: %v", item, err)
		helpers.HandleErrorResponse(w, http.StatusBadRequest, err)
		return
	}

	tx, err := db.BeginTxx(r.Context(), nil)
	if err != nil {
		err = fmt.Errorf("failed to begin DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	defer func() {
		_ = tx.Rollback()
	}()

	err = object.Delete(r.Context(), tx)
	if err != nil {
		err = fmt.Errorf("failed to delete %#+v: %v", object, err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	err = tx.Commit()
	if err != nil {
		err = fmt.Errorf("failed to commit DB transaction: %v", err)
		helpers.HandleErrorResponse(w, http.StatusInternalServerError, err)
		return
	}

	helpers.HandleObjectsResponse(w, http.StatusNoContent, nil)
}

func GetDetectionRouter(db *sqlx.DB, redisConn redis.Conn, httpMiddlewares []server.HTTPMiddleware, modelMiddlewares []server.ModelMiddleware) chi.Router {
	r := chi.NewRouter()

	for _, m := range httpMiddlewares {
		r.Use(m)
	}

	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		handleGetDetections(w, r, db, redisConn, modelMiddlewares)
	})

	r.Get("/{primaryKey}", func(w http.ResponseWriter, r *http.Request) {
		handleGetDetection(w, r, db, redisConn, modelMiddlewares, chi.URLParam(r, "primaryKey"))
	})

	r.Post("/", func(w http.ResponseWriter, r *http.Request) {
		handlePostDetections(w, r, db, redisConn, modelMiddlewares)
	})

	r.Put("/{primaryKey}", func(w http.ResponseWriter, r *http.Request) {
		handlePutDetection(w, r, db, redisConn, modelMiddlewares, chi.URLParam(r, "primaryKey"))
	})

	r.Patch("/{primaryKey}", func(w http.ResponseWriter, r *http.Request) {
		handlePatchDetection(w, r, db, redisConn, modelMiddlewares, chi.URLParam(r, "primaryKey"))
	})

	r.Delete("/{primaryKey}", func(w http.ResponseWriter, r *http.Request) {
		handleDeleteDetection(w, r, db, redisConn, modelMiddlewares, chi.URLParam(r, "primaryKey"))
	})

	return r
}

func NewDetectionFromItem(item map[string]any) (any, error) {
	object := &Detection{}

	err := object.FromItem(item)
	if err != nil {
		return nil, err
	}

	return object, nil
}

func init() {
	register(
		DetectionTable,
		Detection{},
		NewDetectionFromItem,
		"/detections",
		GetDetectionRouter,
	)
}
