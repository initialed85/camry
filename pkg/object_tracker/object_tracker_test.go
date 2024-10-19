package object_tracker

import (
	"testing"

	"github.com/jackc/pgx/v5/pgtype"
	"github.com/stretchr/testify/require"
)

// 2024/10/06 19:52:52 bbA: {288.61859130859375 752.6517333984375}, {521.0448608398438 944.6531982421875}
// 2024/10/06 19:52:52 bbB: {288.6160888671875 749.9412841796875}, {521.6822509765625 944.9613037109375}
// 2024/10/06 19:52:52 bbI: {288.61859130859375 752.6517333984375}, {521.0448608398438 944.6531982421875}

func TestObjectTracker(t *testing.T) {
	require.True(
		t,
		Intersects1D(
			Line{1.0, 3.0},
			Line{2.5, 4.5},
		),
	)

	require.False(
		t,
		Intersects1D(
			Line{1.0, 3.0},
			Line{3.5, 5.5},
		),
	)

	require.True(
		t,
		Intersects2D(
			BoundingBox{
				TL: pgtype.Vec2{X: 1.0, Y: 1.0},
				BR: pgtype.Vec2{X: 3.0, Y: 3.0},
			},
			BoundingBox{
				TL: pgtype.Vec2{X: 2.5, Y: 2.5},
				BR: pgtype.Vec2{X: 4.5, Y: 4.5},
			},
		),
	)

	require.False(
		t,
		Intersects2D(
			BoundingBox{
				TL: pgtype.Vec2{X: 1.0, Y: 1.0},
				BR: pgtype.Vec2{X: 3.0, Y: 3.0},
			},
			BoundingBox{
				TL: pgtype.Vec2{X: 3.5, Y: 3.5},
				BR: pgtype.Vec2{X: 5.5, Y: 5.5},
			},
		),
	)

	require.Equal(
		t,
		&Line{A: 2.5, B: 3},
		GetIntersection1D(
			Line{1.0, 3.0},
			Line{2.5, 4.5},
		),
	)

	require.Equal(
		t,
		&BoundingBox{
			TL: pgtype.Vec2{X: 3.0, Y: 7.0},
			BR: pgtype.Vec2{X: 3.0, Y: 7.5},
		},
		GetIntersection2D(
			BoundingBox{
				TL: pgtype.Vec2{X: 2.5, Y: 7.0},
				BR: pgtype.Vec2{X: 3.0, Y: 7.5},
			},
			BoundingBox{
				TL: pgtype.Vec2{X: 3.0, Y: 6.0},
				BR: pgtype.Vec2{X: 10.0, Y: 11.0},
			},
		),
	)
}
