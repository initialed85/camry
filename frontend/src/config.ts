export const responsiveWidth = 992;
export const dateSliderWidthMillis = 7 * 24 * 60 * 60 * 1000; // 7 days
export const dateSliderStepMillis = 60 * 60 * 1000; // 1 hour
export const audioStripMinScore = 0.2;
// Strip bar height is events-per-hour, not the hour's loudest score: at the hour grain
// min_score 0.2 filters nothing (504/504 live buckets clear it) and top_score p50 is 0.551,
// so score-heighted bars all land at 59-72%. Measured 2026-10-18 on the live 7-day window,
// events per bar: p10 63, p50 665, p90 1783, max 2840 - full height at p90 spreads the
// quiet hours to ~9%, the median to ~42%, and saturates the busiest 10%.
export const audioStripFullCountEvents = 1800;
export const detectionStripMinScore = 0.2;
// Same basis as the audio strip: bar height is the hour's count, full height at the busiest hour.
// Measured 2026-10-18 on the live 24-hour window, per (hour, camera): detections p50 38,994, p95
// 138,328, max 165,826 - full height at the max leaves the median hour at ~30% of the bar.
export const detectionStripFullCountDetections = 160_000;
// Objects are tracks, so the same window is two orders of magnitude sparser: p50 190, p95 777,
// max 1,495. Kept as its own constant so the objects core reads at its own scale inside the bar.
export const detectionStripFullCountObjects = 1_500;
// Each strip is a mirrored pair of rows (bars grow away from the strip's centre line).
export const stripRowHeightPx = 22;
// The video modal's strip covers the recording's own duration at one bar per second, capped so a
// long recording stays readable.
export const videoStripBucketMillis = 1_000;
export const videoStripMaxBuckets = 120;
// Full-height counts for a one-second bar, measured 2026-10-18 from the live per-hour volumes
// divided by the seconds in an hour: detections p50 11/s, max 46/s; objects p50 0.05/s, max
// 0.42/s; audio events p50 0.10/s, max 0.35/s. The object constant is deliberately small - a
// handful of objects in a second is the point of the row.
export const videoStripFullCountEvents = 4;
export const videoStripFullCountDetections = 45;
export const videoStripFullCountObjects = 3;
export const videoStripRowHeightPx = 14;
