package api

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"math"
	"net/http"
	"net/netip"
	"slices"
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

type Camera struct {
	ID                                   uuid.UUID    `json:"id"`
	CreatedAt                            time.Time    `json:"created_at"`
	UpdatedAt                            time.Time    `json:"updated_at"`
	DeletedAt                            *time.Time   `json:"deleted_at"`
	Name                                 string       `json:"name"`
	StreamURL                            string       `json:"stream_url"`
	LastSeen                             time.Time    `json:"last_seen"`
	SegmentProducerClaimedUntil          time.Time    `json:"segment_producer_claimed_until"`
	StreamProducerClaimedUntil           time.Time    `json:"stream_producer_claimed_until"`
	ReferencedByDetectionCameraIDObjects []*Detection `json:"referenced_by_detection_camera_id_objects"`
	ReferencedByVideoCameraIDObjects     []*Video     `json:"referenced_by_video_camera_id_objects"`
}

var CameraTable = "camera"

var CameraTableWithSchema = fmt.Sprintf("%s.%s", schema, CameraTable)

var CameraTableNamespaceID int32 = 1337 + 1

var (
	CameraTableIDColumn                          = "id"
	CameraTableCreatedAtColumn                   = "created_at"
	CameraTableUpdatedAtColumn                   = "updated_at"
	CameraTableDeletedAtColumn                   = "deleted_at"
	CameraTableNameColumn                        = "name"
	CameraTableStreamURLColumn                   = "stream_url"
	CameraTableLastSeenColumn                    = "last_seen"
	CameraTableSegmentProducerClaimedUntilColumn = "segment_producer_claimed_until"
	CameraTableStreamProducerClaimedUntilColumn  = "stream_producer_claimed_until"
)

var (
	CameraTableIDColumnWithTypeCast                          = `"id" AS id`
	CameraTableCreatedAtColumnWithTypeCast                   = `"created_at" AS created_at`
	CameraTableUpdatedAtColumnWithTypeCast                   = `"updated_at" AS updated_at`
	CameraTableDeletedAtColumnWithTypeCast                   = `"deleted_at" AS deleted_at`
	CameraTableNameColumnWithTypeCast                        = `"name" AS name`
	CameraTableStreamURLColumnWithTypeCast                   = `"stream_url" AS stream_url`
	CameraTableLastSeenColumnWithTypeCast                    = `"last_seen" AS last_seen`
	CameraTableSegmentProducerClaimedUntilColumnWithTypeCast = `"segment_producer_claimed_until" AS segment_producer_claimed_until`
	CameraTableStreamProducerClaimedUntilColumnWithTypeCast  = `"stream_producer_claimed_until" AS stream_producer_claimed_until`
)

var CameraTableColumns = []string{
	CameraTableIDColumn,
	CameraTableCreatedAtColumn,
	CameraTableUpdatedAtColumn,
	CameraTableDeletedAtColumn,
	CameraTableNameColumn,
	CameraTableStreamURLColumn,
	CameraTableLastSeenColumn,
	CameraTableSegmentProducerClaimedUntilColumn,
	CameraTableStreamProducerClaimedUntilColumn,
}

var CameraTableColumnsWithTypeCasts = []string{
	CameraTableIDColumnWithTypeCast,
	CameraTableCreatedAtColumnWithTypeCast,
	CameraTableUpdatedAtColumnWithTypeCast,
	CameraTableDeletedAtColumnWithTypeCast,
	CameraTableNameColumnWithTypeCast,
	CameraTableStreamURLColumnWithTypeCast,
	CameraTableLastSeenColumnWithTypeCast,
	CameraTableSegmentProducerClaimedUntilColumnWithTypeCast,
	CameraTableStreamProducerClaimedUntilColumnWithTypeCast,
}

var CameraIntrospectedTable *introspect.Table

var CameraTableColumnLookup map[string]*introspect.Column

var (
	CameraTablePrimaryKeyColumn = CameraTableIDColumn
)

func init() {
	CameraIntrospectedTable = tableByName[CameraTable]

	/* only needed during templating */
	if CameraIntrospectedTable == nil {
		CameraIntrospectedTable = &introspect.Table{}
	}

	CameraTableColumnLookup = CameraIntrospectedTable.ColumnByName
}

type CameraOnePathParams struct {
	PrimaryKey uuid.UUID `json:"primaryKey"`
}

type CameraLoadQueryParams struct {
	Depth *int `json:"depth"`
}

type CameraSegmentProducerClaimRequest struct {
	Until          time.Time `json:"until"`
	TimeoutSeconds float64   `json:"timeout_seconds"`
}

type CameraStreamProducerClaimRequest struct {
	Until          time.Time `json:"until"`
	TimeoutSeconds float64   `json:"timeout_seconds"`
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

func (m *Camera) GetPrimaryKeyColumn() string {
	return CameraTablePrimaryKeyColumn
}

func (m *Camera) GetPrimaryKeyValue() any {
	return m.ID
}

func (m *Camera) FromItem(item map[string]any) error {
	if item == nil {
		return fmt.Errorf(
			"item unexpectedly nil during CameraFromItem",
		)
	}

	if len(item) == 0 {
		return fmt.Errorf(
			"item unexpectedly empty during CameraFromItem",
		)
	}

	wrapError := func(k string, v any, err error) error {
		return fmt.Errorf("%v: %#+v; error; %v", k, v, err)
	}

	for k, v := range item {
		_, ok := CameraTableColumnLookup[k]
		if !ok {
			return fmt.Errorf(
				"item contained unexpected key %#+v during CameraFromItem; item: %#+v",
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

		case "name":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uuname.UUID", temp1))
				}
			}

			m.Name = temp2

		case "stream_url":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uustream_url.UUID", temp1))
				}
			}

			m.StreamURL = temp2

		case "last_seen":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uulast_seen.UUID", temp1))
				}
			}

			m.LastSeen = temp2

		case "segment_producer_claimed_until":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uusegment_producer_claimed_until.UUID", temp1))
				}
			}

			m.SegmentProducerClaimedUntil = temp2

		case "stream_producer_claimed_until":
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
					return wrapError(k, v, fmt.Errorf("failed to cast %#+v to uustream_producer_claimed_until.UUID", temp1))
				}
			}

			m.StreamProducerClaimedUntil = temp2

		}
	}

	return nil
}

func (m *Camera) ToItem() map[string]any {
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

func (m *Camera) Reload(ctx context.Context, tx pgx.Tx, includeDeleteds ...bool) error {
	extraWhere := ""
	if len(includeDeleteds) > 0 && includeDeleteds[0] {
		if slices.Contains(CameraTableColumns, "deleted_at") {
			extraWhere = "\n    AND (deleted_at IS null OR deleted_at IS NOT null)"
		}
	}

	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	o, _, _, _, _, err := SelectCamera(
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
	m.Name = o.Name
	m.StreamURL = o.StreamURL
	m.LastSeen = o.LastSeen
	m.SegmentProducerClaimedUntil = o.SegmentProducerClaimedUntil
	m.StreamProducerClaimedUntil = o.StreamProducerClaimedUntil
	m.ReferencedByDetectionCameraIDObjects = o.ReferencedByDetectionCameraIDObjects
	m.ReferencedByVideoCameraIDObjects = o.ReferencedByVideoCameraIDObjects

	return nil
}

func (m *Camera) GetColumnsAndValues(setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) ([]string, []any, error) {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setPrimaryKey && (setZeroValues || !types.IsZeroUUID(m.ID) || slices.Contains(forceSetValuesForFields, CameraTableIDColumn) || isRequired(CameraTableColumnLookup, CameraTableIDColumn)) {
		columns = append(columns, CameraTableIDColumn)

		v, err := types.FormatUUID(m.ID)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.ID; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, CameraTableCreatedAtColumn) || isRequired(CameraTableColumnLookup, CameraTableCreatedAtColumn) {
		columns = append(columns, CameraTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.CreatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, CameraTableUpdatedAtColumn) || isRequired(CameraTableColumnLookup, CameraTableUpdatedAtColumn) {
		columns = append(columns, CameraTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.UpdatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, CameraTableDeletedAtColumn) || isRequired(CameraTableColumnLookup, CameraTableDeletedAtColumn) {
		columns = append(columns, CameraTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.DeletedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Name) || slices.Contains(forceSetValuesForFields, CameraTableNameColumn) || isRequired(CameraTableColumnLookup, CameraTableNameColumn) {
		columns = append(columns, CameraTableNameColumn)

		v, err := types.FormatString(m.Name)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.Name; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.StreamURL) || slices.Contains(forceSetValuesForFields, CameraTableStreamURLColumn) || isRequired(CameraTableColumnLookup, CameraTableStreamURLColumn) {
		columns = append(columns, CameraTableStreamURLColumn)

		v, err := types.FormatString(m.StreamURL)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.StreamURL; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.LastSeen) || slices.Contains(forceSetValuesForFields, CameraTableLastSeenColumn) || isRequired(CameraTableColumnLookup, CameraTableLastSeenColumn) {
		columns = append(columns, CameraTableLastSeenColumn)

		v, err := types.FormatTime(m.LastSeen)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.LastSeen; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.SegmentProducerClaimedUntil) || slices.Contains(forceSetValuesForFields, CameraTableSegmentProducerClaimedUntilColumn) || isRequired(CameraTableColumnLookup, CameraTableSegmentProducerClaimedUntilColumn) {
		columns = append(columns, CameraTableSegmentProducerClaimedUntilColumn)

		v, err := types.FormatTime(m.SegmentProducerClaimedUntil)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.SegmentProducerClaimedUntil; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.StreamProducerClaimedUntil) || slices.Contains(forceSetValuesForFields, CameraTableStreamProducerClaimedUntilColumn) || isRequired(CameraTableColumnLookup, CameraTableStreamProducerClaimedUntilColumn) {
		columns = append(columns, CameraTableStreamProducerClaimedUntilColumn)

		v, err := types.FormatTime(m.StreamProducerClaimedUntil)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to handle m.StreamProducerClaimedUntil; %v", err)
		}

		values = append(values, v)
	}

	return columns, values, nil
}

func (m *Camera) Insert(ctx context.Context, tx pgx.Tx, setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) error {
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
		CameraTableWithSchema,
		columns,
		nil,
		false,
		false,
		CameraTableColumns,
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to insert %#+v; %v", m, err)
	}
	v := (*item)[CameraTableIDColumn]

	if v == nil {
		return fmt.Errorf("failed to find %v in %#+v", CameraTableIDColumn, item)
	}

	wrapError := func(err error) error {
		return fmt.Errorf(
			"failed to treat %v: %#+v as uuid.UUID: %v",
			CameraTableIDColumn,
			(*item)[CameraTableIDColumn],
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

func (m *Camera) Update(ctx context.Context, tx pgx.Tx, setZeroValues bool, forceSetValuesForFields ...string) error {
	columns := make([]string, 0)
	values := make([]any, 0)

	if setZeroValues || !types.IsZeroTime(m.CreatedAt) || slices.Contains(forceSetValuesForFields, CameraTableCreatedAtColumn) {
		columns = append(columns, CameraTableCreatedAtColumn)

		v, err := types.FormatTime(m.CreatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.CreatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.UpdatedAt) || slices.Contains(forceSetValuesForFields, CameraTableUpdatedAtColumn) {
		columns = append(columns, CameraTableUpdatedAtColumn)

		v, err := types.FormatTime(m.UpdatedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.UpdatedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.DeletedAt) || slices.Contains(forceSetValuesForFields, CameraTableDeletedAtColumn) {
		columns = append(columns, CameraTableDeletedAtColumn)

		v, err := types.FormatTime(m.DeletedAt)
		if err != nil {
			return fmt.Errorf("failed to handle m.DeletedAt; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.Name) || slices.Contains(forceSetValuesForFields, CameraTableNameColumn) {
		columns = append(columns, CameraTableNameColumn)

		v, err := types.FormatString(m.Name)
		if err != nil {
			return fmt.Errorf("failed to handle m.Name; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroString(m.StreamURL) || slices.Contains(forceSetValuesForFields, CameraTableStreamURLColumn) {
		columns = append(columns, CameraTableStreamURLColumn)

		v, err := types.FormatString(m.StreamURL)
		if err != nil {
			return fmt.Errorf("failed to handle m.StreamURL; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.LastSeen) || slices.Contains(forceSetValuesForFields, CameraTableLastSeenColumn) {
		columns = append(columns, CameraTableLastSeenColumn)

		v, err := types.FormatTime(m.LastSeen)
		if err != nil {
			return fmt.Errorf("failed to handle m.LastSeen; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.SegmentProducerClaimedUntil) || slices.Contains(forceSetValuesForFields, CameraTableSegmentProducerClaimedUntilColumn) {
		columns = append(columns, CameraTableSegmentProducerClaimedUntilColumn)

		v, err := types.FormatTime(m.SegmentProducerClaimedUntil)
		if err != nil {
			return fmt.Errorf("failed to handle m.SegmentProducerClaimedUntil; %v", err)
		}

		values = append(values, v)
	}

	if setZeroValues || !types.IsZeroTime(m.StreamProducerClaimedUntil) || slices.Contains(forceSetValuesForFields, CameraTableStreamProducerClaimedUntilColumn) {
		columns = append(columns, CameraTableStreamProducerClaimedUntilColumn)

		v, err := types.FormatTime(m.StreamProducerClaimedUntil)
		if err != nil {
			return fmt.Errorf("failed to handle m.StreamProducerClaimedUntil; %v", err)
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
		CameraTableWithSchema,
		columns,
		fmt.Sprintf("%v = $$??", CameraTableIDColumn),
		CameraTableColumns,
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

func (m *Camera) Delete(ctx context.Context, tx pgx.Tx, hardDeletes ...bool) error {
	hardDelete := false
	if len(hardDeletes) > 0 {
		hardDelete = hardDeletes[0]
	}

	if !hardDelete && slices.Contains(CameraTableColumns, "deleted_at") {
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
		CameraTableWithSchema,
		fmt.Sprintf("%v = $$??", CameraTableIDColumn),
		values...,
	)
	if err != nil {
		return fmt.Errorf("failed to delete %#+v; %v", m, err)
	}

	_ = m.Reload(ctx, tx, true)

	return nil
}

func (m *Camera) LockTable(ctx context.Context, tx pgx.Tx, timeouts ...time.Duration) error {
	return query.LockTable(ctx, tx, CameraTableWithSchema, timeouts...)
}

func (m *Camera) LockTableWithRetries(ctx context.Context, tx pgx.Tx, overallTimeout time.Duration, individualAttempttimeout time.Duration) error {
	return query.LockTableWithRetries(ctx, tx, CameraTableWithSchema, overallTimeout, individualAttempttimeout)
}

func (m *Camera) AdvisoryLock(ctx context.Context, tx pgx.Tx, key int32, timeouts ...time.Duration) error {
	return query.AdvisoryLock(ctx, tx, CameraTableNamespaceID, key, timeouts...)
}

func (m *Camera) AdvisoryLockWithRetries(ctx context.Context, tx pgx.Tx, key int32, overallTimeout time.Duration, individualAttempttimeout time.Duration) error {
	return query.AdvisoryLockWithRetries(ctx, tx, CameraTableNamespaceID, key, overallTimeout, individualAttempttimeout)
}

func (m *Camera) SegmentProducerClaim(ctx context.Context, tx pgx.Tx, until time.Time, timeout time.Duration) error {
	err := m.AdvisoryLockWithRetries(ctx, tx, math.MinInt32, timeout, time.Second*1)
	if err != nil {
		return fmt.Errorf("failed to claim (advisory lock): %s", err.Error())
	}

	_, _, _, _, _, err = SelectCamera(
		ctx,
		tx,
		fmt.Sprintf(
			"%s = $$?? AND (segment_producer_claimed_until IS null OR segment_producer_claimed_until < now())",
			CameraTablePrimaryKeyColumn,
		),
		m.GetPrimaryKeyValue(),
	)
	if err != nil {
		return fmt.Errorf("failed to claim (select): %s", err.Error())
	}

	m.SegmentProducerClaimedUntil = until

	err = m.Update(ctx, tx, false)
	if err != nil {
		return fmt.Errorf("failed to claim (update): %s", err.Error())
	}

	return nil
}

func (m *Camera) StreamProducerClaim(ctx context.Context, tx pgx.Tx, until time.Time, timeout time.Duration) error {
	err := m.AdvisoryLockWithRetries(ctx, tx, math.MinInt32, timeout, time.Second*1)
	if err != nil {
		return fmt.Errorf("failed to claim (advisory lock): %s", err.Error())
	}

	_, _, _, _, _, err = SelectCamera(
		ctx,
		tx,
		fmt.Sprintf(
			"%s = $$?? AND (stream_producer_claimed_until IS null OR stream_producer_claimed_until < now())",
			CameraTablePrimaryKeyColumn,
		),
		m.GetPrimaryKeyValue(),
	)
	if err != nil {
		return fmt.Errorf("failed to claim (select): %s", err.Error())
	}

	m.StreamProducerClaimedUntil = until

	err = m.Update(ctx, tx, false)
	if err != nil {
		return fmt.Errorf("failed to claim (update): %s", err.Error())
	}

	return nil
}

func SelectCameras(ctx context.Context, tx pgx.Tx, where string, orderBy *string, limit *int, offset *int, values ...any) ([]*Camera, int64, int64, int64, int64, error) {
	before := time.Now()

	if config.Debug() {
		log.Printf("entered SelectCameras")

		defer func() {
			log.Printf("exited SelectCameras in %s", time.Since(before))
		}()
	}
	if slices.Contains(CameraTableColumns, "deleted_at") {
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

	shouldLoad := query.ShouldLoad(ctx, CameraTable) || query.ShouldLoad(ctx, fmt.Sprintf("referenced_by_%s", CameraTable))

	var ok bool
	ctx, ok = query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("%s{%v}", CameraTable, nil), !isLoadQuery)
	if !ok && !shouldLoad {
		if config.Debug() {
			log.Printf("skipping SelectCamera early (query.ShouldLoad(): %v, query.HandleQueryPathGraphCycles(): %v)", shouldLoad, ok)
		}
		return []*Camera{}, 0, 0, 0, 0, nil
	}

	var items *[]map[string]any
	var count int64
	var totalCount int64
	var page int64
	var totalPages int64
	var err error

	useInstead, shouldSkip := query.ShouldSkip[Camera](ctx)
	if !shouldSkip {
		items, count, totalCount, page, totalPages, err = query.Select(
			ctx,
			tx,
			CameraTableColumnsWithTypeCasts,
			CameraTableWithSchema,
			where,
			orderBy,
			limit,
			offset,
			values...,
		)
		if err != nil {
			return nil, 0, 0, 0, 0, fmt.Errorf("failed to call SelectCameras; %v", err)
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

	objects := make([]*Camera, 0)

	for _, item := range *items {
		var object *Camera

		if !shouldSkip {
			object = &Camera{}
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

		err = func() error {
			shouldLoad := query.ShouldLoad(ctx, fmt.Sprintf("referenced_by_%s", DetectionTable))
			ctx, ok := query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("__ReferencedBy__%s{%v}", DetectionTable, object.GetPrimaryKeyValue()), true)
			if ok || shouldLoad {
				thisBefore := time.Now()

				if config.Debug() {
					log.Printf("loading SelectCameras->SelectDetections for object.ReferencedByDetectionCameraIDObjects")
				}

				object.ReferencedByDetectionCameraIDObjects, _, _, _, _, err = SelectDetections(
					ctx,
					tx,
					fmt.Sprintf("%v = $1", DetectionTableCameraIDColumn),
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
					log.Printf("loaded SelectCameras->SelectDetections for object.ReferencedByDetectionCameraIDObjects in %s", time.Since(thisBefore))
				}

			}

			return nil
		}()
		if err != nil {
			return nil, 0, 0, 0, 0, err
		}

		err = func() error {
			shouldLoad := query.ShouldLoad(ctx, fmt.Sprintf("referenced_by_%s", VideoTable))
			ctx, ok := query.HandleQueryPathGraphCycles(ctx, fmt.Sprintf("__ReferencedBy__%s{%v}", VideoTable, object.GetPrimaryKeyValue()), true)
			if ok || shouldLoad {
				thisBefore := time.Now()

				if config.Debug() {
					log.Printf("loading SelectCameras->SelectVideos for object.ReferencedByVideoCameraIDObjects")
				}

				object.ReferencedByVideoCameraIDObjects, _, _, _, _, err = SelectVideos(
					ctx,
					tx,
					fmt.Sprintf("%v = $1", VideoTableCameraIDColumn),
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
					log.Printf("loaded SelectCameras->SelectVideos for object.ReferencedByVideoCameraIDObjects in %s", time.Since(thisBefore))
				}

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

func SelectCamera(ctx context.Context, tx pgx.Tx, where string, values ...any) (*Camera, int64, int64, int64, int64, error) {
	ctx, cleanup := query.WithQueryID(ctx)
	defer cleanup()

	ctx = query.WithMaxDepth(ctx, nil)

	objects, _, _, _, _, err := SelectCameras(
		ctx,
		tx,
		where,
		nil,
		helpers.Ptr(2),
		helpers.Ptr(0),
		values...,
	)
	if err != nil {
		return nil, 0, 0, 0, 0, fmt.Errorf("failed to call SelectCamera; %v", err)
	}

	if len(objects) > 1 {
		return nil, 0, 0, 0, 0, fmt.Errorf("attempt to call SelectCamera returned more than 1 row")
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

func InsertCameras(ctx context.Context, tx pgx.Tx, objects []*Camera, setPrimaryKey bool, setZeroValues bool, forceSetValuesForFields ...string) ([]*Camera, error) {
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
		CameraTableWithSchema,
		columns,
		nil,
		false,
		false,
		CameraTableColumns,
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to bulk insert %d objects; %v", len(objects), err)
	}

	returnedObjects := make([]*Camera, 0)

	for _, item := range items {
		v := &Camera{}
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

func SegmentProducerClaimCamera(ctx context.Context, tx pgx.Tx, until time.Time, timeout time.Duration, where string, values ...any) (*Camera, error) {
	m := &Camera{}

	err := m.AdvisoryLockWithRetries(ctx, tx, math.MinInt32, timeout, time.Second*1)
	if err != nil {
		return nil, fmt.Errorf("failed to claim: %s", err.Error())
	}

	if strings.TrimSpace(where) != "" {
		where += " AND\n"
	}

	where += "    (segment_producer_claimed_until IS null OR segment_producer_claimed_until < now())"

	ms, _, _, _, _, err := SelectCameras(
		ctx,
		tx,
		where,
		helpers.Ptr(
			"segment_producer_claimed_until ASC",
		),
		helpers.Ptr(1),
		nil,
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to claim: %s", err.Error())
	}

	if len(ms) == 0 {
		return nil, nil
	}

	m = ms[0]

	m.SegmentProducerClaimedUntil = until

	err = m.Update(ctx, tx, false)
	if err != nil {
		return nil, fmt.Errorf("failed to claim: %s", err.Error())
	}

	return m, nil
}

func StreamProducerClaimCamera(ctx context.Context, tx pgx.Tx, until time.Time, timeout time.Duration, where string, values ...any) (*Camera, error) {
	m := &Camera{}

	err := m.AdvisoryLockWithRetries(ctx, tx, math.MinInt32, timeout, time.Second*1)
	if err != nil {
		return nil, fmt.Errorf("failed to claim: %s", err.Error())
	}

	if strings.TrimSpace(where) != "" {
		where += " AND\n"
	}

	where += "    (stream_producer_claimed_until IS null OR stream_producer_claimed_until < now())"

	ms, _, _, _, _, err := SelectCameras(
		ctx,
		tx,
		where,
		helpers.Ptr(
			"stream_producer_claimed_until ASC",
		),
		helpers.Ptr(1),
		nil,
		values...,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to claim: %s", err.Error())
	}

	if len(ms) == 0 {
		return nil, nil
	}

	m = ms[0]

	m.StreamProducerClaimedUntil = until

	err = m.Update(ctx, tx, false)
	if err != nil {
		return nil, fmt.Errorf("failed to claim: %s", err.Error())
	}

	return m, nil
}

func handleGetCameras(arguments *server.SelectManyArguments, db *pgxpool.Pool) ([]*Camera, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	objects, count, totalCount, page, totalPages, err := SelectCameras(arguments.Ctx, tx, arguments.Where, arguments.OrderBy, arguments.Limit, arguments.Offset, arguments.Values...)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	return objects, count, totalCount, page, totalPages, nil
}

func handleGetCamera(arguments *server.SelectOneArguments, db *pgxpool.Pool, primaryKey uuid.UUID) ([]*Camera, int64, int64, int64, int64, error) {
	tx, err := db.Begin(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	defer func() {
		_ = tx.Rollback(arguments.Ctx)
	}()

	object, count, totalCount, page, totalPages, err := SelectCamera(arguments.Ctx, tx, arguments.Where, arguments.Values...)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	err = tx.Commit(arguments.Ctx)
	if err != nil {
		return nil, 0, 0, 0, 0, err
	}

	return []*Camera{object}, count, totalCount, page, totalPages, nil
}

func handlePostCamera(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, objects []*Camera, forceSetValuesForFieldsByObjectIndex [][]string) ([]*Camera, int64, int64, int64, int64, error) {
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

	returnedObjects, err := InsertCameras(arguments.Ctx, tx, objects, false, false, slices.Collect(maps.Keys(forceSetValuesForFieldsByObjectIndexMaximal))...)
	if err != nil {
		err = fmt.Errorf("failed to insert %d objects; %v", len(objects), err)
		return nil, 0, 0, 0, 0, err
	}

	copy(objects, returnedObjects)

	errs := make(chan error, 1)
	go func() {
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.INSERT}, CameraTable, xid)
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

func handlePutCamera(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Camera) ([]*Camera, int64, int64, int64, int64, error) {
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
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, CameraTable, xid)
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

	return []*Camera{object}, count, totalCount, page, totalPages, nil
}

func handlePatchCamera(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Camera, forceSetValuesForFields []string) ([]*Camera, int64, int64, int64, int64, error) {
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
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.UPDATE, stream.SOFT_DELETE, stream.SOFT_RESTORE, stream.SOFT_UPDATE}, CameraTable, xid)
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

	return []*Camera{object}, count, totalCount, page, totalPages, nil
}

func handleDeleteCamera(arguments *server.LoadArguments, db *pgxpool.Pool, waitForChange server.WaitForChange, object *Camera) error {
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
		_, err := waitForChange(arguments.Ctx, []stream.Action{stream.DELETE, stream.SOFT_DELETE}, CameraTable, xid)
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

func MutateRouterForCamera(r chi.Router, db *pgxpool.Pool, redisPool *redis.Pool, objectMiddlewares []server.ObjectMiddleware, waitForChange server.WaitForChange) {

	func() {
		postHandlerForSegmentProducerClaim, err := getHTTPHandler(
			http.MethodPost,
			"/segment-producer-claim-camera",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams server.EmptyQueryParams,
				req CameraSegmentProducerClaimRequest,
				rawReq any,
			) (server.Response[Camera], error) {
				tx, err := db.Begin(ctx)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				defer func() {
					_ = tx.Rollback(ctx)
				}()

				object, err := SegmentProducerClaimCamera(ctx, tx, req.Until, time.Millisecond*time.Duration(req.TimeoutSeconds*1000), "")
				if err != nil {
					return server.Response[Camera]{}, err
				}

				count := int64(0)

				totalCount := int64(0)

				limit := int64(0)

				offset := int64(0)

				if object == nil {
					return server.Response[Camera]{
						Status:     http.StatusOK,
						Success:    true,
						Error:      nil,
						Objects:    []*Camera{},
						Count:      count,
						TotalCount: totalCount,
						Limit:      limit,
						Offset:     offset,
					}, nil
				}

				err = tx.Commit(ctx)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				return server.Response[Camera]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    []*Camera{object},
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}, nil
			},
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Post(postHandlerForSegmentProducerClaim.FullPath, postHandlerForSegmentProducerClaim.ServeHTTP)

		postHandlerForSegmentProducerClaimOne, err := getHTTPHandler(
			http.MethodPost,
			"/cameras/{primaryKey}/segment-producer-claim",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams CameraOnePathParams,
				queryParams CameraLoadQueryParams,
				req CameraSegmentProducerClaimRequest,
				rawReq any,
			) (server.Response[Camera], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectOneArguments(ctx, queryParams.Depth, CameraIntrospectedTable, pathParams.PrimaryKey, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				/* note: deliberately no attempt at a cache hit */

				var object *Camera
				var count int64
				var totalCount int64

				err = func() error {
					tx, err := db.Begin(arguments.Ctx)
					if err != nil {
						return err
					}

					defer func() {
						_ = tx.Rollback(arguments.Ctx)
					}()

					object, count, totalCount, _, _, err = SelectCamera(arguments.Ctx, tx, arguments.Where, arguments.Values...)
					if err != nil {
						return fmt.Errorf("failed to select object to claim: %s", err.Error())
					}

					err = object.SegmentProducerClaim(arguments.Ctx, tx, req.Until, time.Millisecond*time.Duration(req.TimeoutSeconds*1000))
					if err != nil {
						return err
					}

					err = tx.Commit(arguments.Ctx)
					if err != nil {
						return err
					}

					return nil
				}()
				if err != nil {
					if config.Debug() {
						log.Printf("request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				response := server.Response[Camera]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    []*Camera{object},
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}

				return response, nil
			},
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Post(postHandlerForSegmentProducerClaimOne.FullPath, postHandlerForSegmentProducerClaimOne.ServeHTTP)
	}()

	func() {
		postHandlerForStreamProducerClaim, err := getHTTPHandler(
			http.MethodPost,
			"/stream-producer-claim-camera",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams server.EmptyQueryParams,
				req CameraStreamProducerClaimRequest,
				rawReq any,
			) (server.Response[Camera], error) {
				tx, err := db.Begin(ctx)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				defer func() {
					_ = tx.Rollback(ctx)
				}()

				object, err := StreamProducerClaimCamera(ctx, tx, req.Until, time.Millisecond*time.Duration(req.TimeoutSeconds*1000), "")
				if err != nil {
					return server.Response[Camera]{}, err
				}

				count := int64(0)

				totalCount := int64(0)

				limit := int64(0)

				offset := int64(0)

				if object == nil {
					return server.Response[Camera]{
						Status:     http.StatusOK,
						Success:    true,
						Error:      nil,
						Objects:    []*Camera{},
						Count:      count,
						TotalCount: totalCount,
						Limit:      limit,
						Offset:     offset,
					}, nil
				}

				err = tx.Commit(ctx)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				return server.Response[Camera]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    []*Camera{object},
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}, nil
			},
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Post(postHandlerForStreamProducerClaim.FullPath, postHandlerForStreamProducerClaim.ServeHTTP)

		postHandlerForStreamProducerClaimOne, err := getHTTPHandler(
			http.MethodPost,
			"/cameras/{primaryKey}/stream-producer-claim",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams CameraOnePathParams,
				queryParams CameraLoadQueryParams,
				req CameraStreamProducerClaimRequest,
				rawReq any,
			) (server.Response[Camera], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectOneArguments(ctx, queryParams.Depth, CameraIntrospectedTable, pathParams.PrimaryKey, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				/* note: deliberately no attempt at a cache hit */

				var object *Camera
				var count int64
				var totalCount int64

				err = func() error {
					tx, err := db.Begin(arguments.Ctx)
					if err != nil {
						return err
					}

					defer func() {
						_ = tx.Rollback(arguments.Ctx)
					}()

					object, count, totalCount, _, _, err = SelectCamera(arguments.Ctx, tx, arguments.Where, arguments.Values...)
					if err != nil {
						return fmt.Errorf("failed to select object to claim: %s", err.Error())
					}

					err = object.StreamProducerClaim(arguments.Ctx, tx, req.Until, time.Millisecond*time.Duration(req.TimeoutSeconds*1000))
					if err != nil {
						return err
					}

					err = tx.Commit(arguments.Ctx)
					if err != nil {
						return err
					}

					return nil
				}()
				if err != nil {
					if config.Debug() {
						log.Printf("request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				response := server.Response[Camera]{
					Status:     http.StatusOK,
					Success:    true,
					Error:      nil,
					Objects:    []*Camera{object},
					Count:      count,
					TotalCount: totalCount,
					Limit:      limit,
					Offset:     offset,
				}

				return response, nil
			},
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Post(postHandlerForStreamProducerClaimOne.FullPath, postHandlerForStreamProducerClaimOne.ServeHTTP)
	}()

	func() {
		getManyHandler, err := getHTTPHandler(
			http.MethodGet,
			"/cameras",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams map[string]any,
				req server.EmptyRequest,
				rawReq any,
			) (server.Response[Camera], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectManyArguments(ctx, queryParams, CameraIntrospectedTable, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache not yet reached; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				cachedResponseAsJSON, cacheHit, err := server.GetCachedResponseAsJSON(arguments.RequestHash, redisConn)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache failed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				if cacheHit {
					var cachedResponse server.Response[Camera]

					/* TODO: it'd be nice to be able to avoid this (i.e. just pass straight through) */
					err = json.Unmarshal(cachedResponseAsJSON, &cachedResponse)
					if err != nil {
						if config.Debug() {
							log.Printf("request cache hit but failed unmarshal; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
						}

						return server.Response[Camera]{}, err
					}

					if config.Debug() {
						log.Printf("request cache hit; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return cachedResponse, nil
				}

				objects, count, totalCount, _, _, err := handleGetCameras(arguments, db)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				limit := int64(0)
				if arguments.Limit != nil {
					limit = int64(*arguments.Limit)
				}

				offset := int64(0)
				if arguments.Offset != nil {
					offset = int64(*arguments.Offset)
				}

				response := server.Response[Camera]{
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

					return server.Response[Camera]{}, err
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
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Get(getManyHandler.FullPath, getManyHandler.ServeHTTP)
	}()

	func() {
		getOneHandler, err := getHTTPHandler(
			http.MethodGet,
			"/cameras/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams CameraOnePathParams,
				queryParams CameraLoadQueryParams,
				req server.EmptyRequest,
				rawReq any,
			) (server.Response[Camera], error) {
				before := time.Now()

				redisConn := redisPool.Get()
				defer func() {
					_ = redisConn.Close()
				}()

				arguments, err := server.GetSelectOneArguments(ctx, queryParams.Depth, CameraIntrospectedTable, pathParams.PrimaryKey, nil, nil)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache not yet reached; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				cachedResponseAsJSON, cacheHit, err := server.GetCachedResponseAsJSON(arguments.RequestHash, redisConn)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache failed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				if cacheHit {
					var cachedResponse server.Response[Camera]

					/* TODO: it'd be nice to be able to avoid this (i.e. just pass straight through) */
					err = json.Unmarshal(cachedResponseAsJSON, &cachedResponse)
					if err != nil {
						if config.Debug() {
							log.Printf("request cache hit but failed unmarshal; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
						}

						return server.Response[Camera]{}, err
					}

					if config.Debug() {
						log.Printf("request cache hit; request succeeded in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return cachedResponse, nil
				}

				objects, count, totalCount, _, _, err := handleGetCamera(arguments, db, pathParams.PrimaryKey)
				if err != nil {
					if config.Debug() {
						log.Printf("request cache missed; request failed in %s %s path: %#+v query: %#+v req: %#+v", time.Since(before), http.MethodGet, pathParams, queryParams, req)
					}

					return server.Response[Camera]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				response := server.Response[Camera]{
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

					return server.Response[Camera]{}, err
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
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Get(getOneHandler.FullPath, getOneHandler.ServeHTTP)
	}()

	func() {
		postHandler, err := getHTTPHandler(
			http.MethodPost,
			"/cameras",
			http.StatusCreated,
			func(
				ctx context.Context,
				pathParams server.EmptyPathParams,
				queryParams CameraLoadQueryParams,
				req []*Camera,
				rawReq any,
			) (server.Response[Camera], error) {
				allRawItems, ok := rawReq.([]any)
				if !ok {
					return server.Response[Camera]{}, fmt.Errorf("failed to cast %#+v to []map[string]any", rawReq)
				}

				allItems := make([]map[string]any, 0)
				for _, rawItem := range allRawItems {
					item, ok := rawItem.(map[string]any)
					if !ok {
						return server.Response[Camera]{}, fmt.Errorf("failed to cast %#+v to map[string]any", rawItem)
					}

					allItems = append(allItems, item)
				}

				forceSetValuesForFieldsByObjectIndex := make([][]string, 0)
				for _, item := range allItems {
					forceSetValuesForFields := make([]string, 0)
					for _, possibleField := range slices.Collect(maps.Keys(item)) {
						if !slices.Contains(CameraTableColumns, possibleField) {
							continue
						}

						forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
					}
					forceSetValuesForFieldsByObjectIndex = append(forceSetValuesForFieldsByObjectIndex, forceSetValuesForFields)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				objects, count, totalCount, _, _, err := handlePostCamera(arguments, db, waitForChange, req, forceSetValuesForFieldsByObjectIndex)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[Camera]{
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
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Post(postHandler.FullPath, postHandler.ServeHTTP)
	}()

	func() {
		putHandler, err := getHTTPHandler(
			http.MethodPatch,
			"/cameras/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams CameraOnePathParams,
				queryParams CameraLoadQueryParams,
				req Camera,
				rawReq any,
			) (server.Response[Camera], error) {
				item, ok := rawReq.(map[string]any)
				if !ok {
					return server.Response[Camera]{}, fmt.Errorf("failed to cast %#+v to map[string]any", item)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				object := &req
				object.ID = pathParams.PrimaryKey

				objects, count, totalCount, _, _, err := handlePutCamera(arguments, db, waitForChange, object)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[Camera]{
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
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Put(putHandler.FullPath, putHandler.ServeHTTP)
	}()

	func() {
		patchHandler, err := getHTTPHandler(
			http.MethodPatch,
			"/cameras/{primaryKey}",
			http.StatusOK,
			func(
				ctx context.Context,
				pathParams CameraOnePathParams,
				queryParams CameraLoadQueryParams,
				req Camera,
				rawReq any,
			) (server.Response[Camera], error) {
				item, ok := rawReq.(map[string]any)
				if !ok {
					return server.Response[Camera]{}, fmt.Errorf("failed to cast %#+v to map[string]any", item)
				}

				forceSetValuesForFields := make([]string, 0)
				for _, possibleField := range slices.Collect(maps.Keys(item)) {
					if !slices.Contains(CameraTableColumns, possibleField) {
						continue
					}

					forceSetValuesForFields = append(forceSetValuesForFields, possibleField)
				}

				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				object := &req
				object.ID = pathParams.PrimaryKey

				objects, count, totalCount, _, _, err := handlePatchCamera(arguments, db, waitForChange, object, forceSetValuesForFields)
				if err != nil {
					return server.Response[Camera]{}, err
				}

				limit := int64(0)

				offset := int64(0)

				return server.Response[Camera]{
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
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Patch(patchHandler.FullPath, patchHandler.ServeHTTP)
	}()

	func() {
		deleteHandler, err := getHTTPHandler(
			http.MethodDelete,
			"/cameras/{primaryKey}",
			http.StatusNoContent,
			func(
				ctx context.Context,
				pathParams CameraOnePathParams,
				queryParams CameraLoadQueryParams,
				req server.EmptyRequest,
				rawReq any,
			) (server.EmptyResponse, error) {
				arguments, err := server.GetLoadArguments(ctx, queryParams.Depth)
				if err != nil {
					return server.EmptyResponse{}, err
				}

				object := &Camera{}
				object.ID = pathParams.PrimaryKey

				err = handleDeleteCamera(arguments, db, waitForChange, object)
				if err != nil {
					return server.EmptyResponse{}, err
				}

				return server.EmptyResponse{}, nil
			},
			Camera{},
			CameraIntrospectedTable,
		)
		if err != nil {
			panic(err)
		}
		r.Delete(deleteHandler.FullPath, deleteHandler.ServeHTTP)
	}()
}

func NewCameraFromItem(item map[string]any) (any, error) {
	object := &Camera{}

	err := object.FromItem(item)
	if err != nil {
		return nil, err
	}

	return object, nil
}

func init() {
	register(
		CameraTable,
		Camera{},
		NewCameraFromItem,
		"/cameras",
		MutateRouterForCamera,
	)
}
