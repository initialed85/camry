package object_tracker

import (
	"image/color"
	"slices"

	"github.com/jackc/pgx/v5/pgtype"
)

type Line struct {
	A float64
	B float64
}

type BoundingBox struct {
	TL            pgtype.Vec2
	BR            pgtype.Vec2
	Color         color.RGBA
	IDWithinFrame int64
}

func Intersects1D(a, b Line) bool {
	minA := slices.Min([]float64{a.A, a.B})
	maxA := slices.Max([]float64{a.A, a.B})
	minB := slices.Min([]float64{b.A, b.B})
	maxB := slices.Max([]float64{b.A, b.B})

	return maxA >= minB && maxB >= minA
}

func Intersects2D(a BoundingBox, b BoundingBox) bool {
	return Intersects1D(
		Line{a.TL.X, a.BR.X},
		Line{b.TL.X, b.BR.X},
	) && Intersects1D(
		Line{a.TL.Y, a.BR.Y},
		Line{b.TL.Y, b.BR.Y},
	)
}

func GetIntersection1D(a, b Line) *Line {
	minA := slices.Min([]float64{a.A, a.B})
	maxA := slices.Max([]float64{a.A, a.B})
	minB := slices.Min([]float64{b.A, b.B})
	maxB := slices.Max([]float64{b.A, b.B})

	if !(maxA >= minB && maxB >= minA) {
		return nil
	}

	return &Line{
		slices.Max([]float64{minA, minB}),
		slices.Min([]float64{maxA, maxB}),
	}
}

func GetIntersection2D(a BoundingBox, b BoundingBox) *BoundingBox {
	x := GetIntersection1D(Line{a.TL.X, a.BR.X}, Line{b.TL.X, b.BR.X})
	if x == nil {
		return nil
	}

	y := GetIntersection1D(Line{a.TL.Y, a.BR.Y}, Line{b.TL.Y, b.BR.Y})
	if y == nil {
		return nil
	}

	return &BoundingBox{
		TL: pgtype.Vec2{
			X: x.A,
			Y: y.A,
		},
		BR: pgtype.Vec2{
			X: x.B,
			Y: y.B,
		},
	}
}
