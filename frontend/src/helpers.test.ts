import {
  formatDate,
  formatScore,
  liveQueryCeiling,
  parseDate,
  pickNearestStartedAt,
  recordingSiblingQuery,
  scoreOf,
  truncateDate,
} from "./helpers";

describe("date helpers should work", () => {
  test("just after midnight", () => {
    const testISOString = "2024-07-19T00:00:00.000+08:00";
    const testDate = new Date(testISOString);

    const parsed = parseDate(testISOString);
    expect(parsed).toStrictEqual(testDate);

    const formatted = formatDate(testDate);
    expect(formatted).toStrictEqual(testISOString);

    const truncated = truncateDate(testDate);
    expect(truncated).toStrictEqual(new Date("2024-07-19T00:00:00.000+08:00"));
  });

  test("at midday", () => {
    const testISOString = "2024-07-19T12:00:00.000+08:00";
    const testDate = new Date(testISOString);

    const parsed = parseDate(testISOString);
    expect(parsed).toStrictEqual(testDate);

    const formatted = formatDate(testDate);
    expect(formatted).toStrictEqual(testISOString);

    const truncated = truncateDate(testDate);
    expect(truncated).toStrictEqual(new Date("2024-07-19T00:00:00.000+08:00"));
  });

  test("just before midnight", () => {
    const testISOString = "2024-07-19T23:59:59.999+08:00";
    const testDate = new Date(testISOString);

    const parsed = parseDate(testISOString);
    expect(parsed).toStrictEqual(testDate);

    const formatted = formatDate(testDate);
    expect(formatted).toStrictEqual(testISOString);

    const truncated = truncateDate(testDate);
    expect(truncated).toStrictEqual(new Date("2024-07-19T00:00:00.000+08:00"));
  });
});

describe("summary scores should format without crashing", () => {
  test("prefers mean_score over average_score", () => {
    expect(scoreOf({ mean_score: 0.25, average_score: 0.9 })).toStrictEqual(
      0.25,
    );
    expect(scoreOf({ average_score: 0.9 })).toStrictEqual(0.9);
    expect(scoreOf({ mean_score: 0 })).toStrictEqual(0);
    expect(scoreOf(undefined)).toStrictEqual(undefined);
  });

  test("formats a number with two decimals", () => {
    expect(formatScore({ mean_score: 0.42 })).toStrictEqual("0.42");
    expect(formatScore({ mean_score: 0.5 })).toStrictEqual("0.50");
    expect(formatScore({ average_score: 1 })).toStrictEqual("1.00");
    expect(formatScore({ mean_score: 0 })).toStrictEqual("0.00");
    expect(formatScore({ mean_score: 12.345 })).toStrictEqual("12.35");
  });

  test("falls back to a dash for a score that is not a finite number", () => {
    expect(formatScore(undefined)).toStrictEqual("-");
    expect(formatScore(null)).toStrictEqual("-");
    expect(formatScore({})).toStrictEqual("-");
    expect(
      formatScore({ mean_score: null, average_score: null }),
    ).toStrictEqual("-");
    expect(formatScore({ mean_score: NaN })).toStrictEqual("-");
    expect(formatScore({ mean_score: Infinity })).toStrictEqual("-");
    expect(formatScore({ mean_score: "0.42" })).toStrictEqual("-");
    expect(formatScore({ average_score: "0.42" })).toStrictEqual("-");
  });
});

// Real dev rows: one minute, cut by two independent segmenters. The low-res
// encoder opened its file 1.5 s before the high-res one noticed its own.
const highResStartedAt = "2026-10-27T16:01:03.181263+08:00";
const cameraId = "326da5dd-8a6a-4534-9598-effeb03abe61";

// A window can still reach the recordings on either side, so nearest wins.
const siblingCandidates = [
  { started_at: "2026-10-27T16:00:01.653999+08:00" },
  { started_at: "2026-10-27T16:01:01.653999+08:00" },
  { started_at: "2026-10-27T16:02:01.653999+08:00" },
];

describe("recording pairing should not depend on file names", () => {
  test("pairs by camera and a start-time window in strict RFC 3339", () => {
    expect(recordingSiblingQuery(cameraId, highResStartedAt)).toStrictEqual({
      camera_id__eq: cameraId,
      is_low_res__eq: true,
      started_at__gte: "2026-10-27T08:00:43.181Z",
      started_at__lte: "2026-10-27T08:01:23.181Z",
      limit: 10,
    });
  });

  test("refuses to guess a sibling without a camera or a start time", () => {
    expect(recordingSiblingQuery(undefined, highResStartedAt)).toBeUndefined();
    expect(recordingSiblingQuery("", highResStartedAt)).toBeUndefined();
    expect(recordingSiblingQuery(cameraId, undefined)).toBeUndefined();
    expect(recordingSiblingQuery(cameraId, "")).toBeUndefined();
  });

  test("picks the candidate that starts nearest the recording being played", () => {
    expect(
      pickNearestStartedAt(siblingCandidates, highResStartedAt),
    ).toStrictEqual(siblingCandidates[1]);
  });

  test("ignores candidates with no start time, and finds nothing in an empty set", () => {
    expect(pickNearestStartedAt([], highResStartedAt)).toBeUndefined();
    expect(pickNearestStartedAt(undefined, highResStartedAt)).toBeUndefined();
    expect(
      pickNearestStartedAt(
        [{ started_at: undefined }, ...siblingCandidates],
        highResStartedAt,
      ),
    ).toStrictEqual(siblingCandidates[1]);
  });
});

describe("a live query ceiling should move with time, and stay ahead of now", () => {
  test("rounds up to the next minute and sits one to two minutes ahead", () => {
    const now = new Date("2024-07-19T11:59:30.000+08:00");
    const ceiling = liveQueryCeiling(now);

    expect(Date.parse(ceiling)).toBeGreaterThan(now.getTime() + 60_000);
    expect(Date.parse(ceiling)).toBeLessThanOrEqual(now.getTime() + 120_000);
    expect(Date.parse(ceiling) % 60_000).toBe(0);
  });

  test("answers with strict RFC 3339, which the API's filter parser accepts", () => {
    const now = new Date("2024-07-19T11:59:30.000+08:00");

    expect(liveQueryCeiling(now)).toStrictEqual("2024-07-19T04:01:00.000Z");
  });

  test("changes the moment the minute rolls over", () => {
    expect(
      liveQueryCeiling(new Date("2024-07-19T12:00:30.000+08:00")),
    ).toStrictEqual("2024-07-19T04:02:00.000Z");
  });
});
