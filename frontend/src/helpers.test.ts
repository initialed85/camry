import {
  formatDate,
  formatScore,
  parseDate,
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
