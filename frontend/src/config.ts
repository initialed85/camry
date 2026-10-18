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
