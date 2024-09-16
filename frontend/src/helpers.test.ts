import { formatDate, parseDate, truncateDate } from "./helpers";

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
