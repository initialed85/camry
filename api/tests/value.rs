//! Filter value parsing, checked against what the Go reference accepts,
//! refuses, and prints. No database needed.

use api::crud::Param;
use api::crud::value;
use api::pg_types::PgDuration;

#[test]
fn float_lists_are_refused() {
    // `__in` turns the value back into text, and ParseFloat has no string
    // branch, so the reference fails a float list whatever it held.
    assert!(value::f64s("score__in", &Param::decode("3")).is_err());
    assert!(value::f64s("score__in", &Param::decode("0.5,1.5")).is_err());
    assert!(
        value::f64s("score__in", &Param::decode("299123456000"))
            .unwrap_err()
            .contains("could not be identified for ParseFloat")
    );

    // A JSON array is textualised the same way, so it fails with the `%v` of
    // itself, spaces and all, as the refused piece.
    assert_eq!(
        value::f64s("score__in", &Param::decode("[1, 2.5]")).unwrap_err(),
        "unparseable params score__in=[%!s(float64=1) %!s(float64=2.5)] \
 (\"[1 2.5]\" (string) could not be identified for ParseFloat)"
    );
}

#[test]
fn bool_lists_are_split_into_text() {
    assert_eq!(
        value::booleans("is_low_res__in", &Param::decode("true,false")).unwrap(),
        vec![true, false]
    );

    // `%v` separates array elements with a space, so one unparseable piece is
    // left, and ParseBool knows nothing about brackets.
    assert!(value::booleans("is_low_res__in", &Param::decode("[true]")).is_err());
    assert!(value::booleans("is_low_res__in", &Param::decode("yes")).is_err());
}

#[test]
fn intervals_are_microseconds_only_whole() {
    // A scalar number is the one spelling that means microseconds.
    assert_eq!(
        value::duration("duration__gte", &Param::decode("299123456000")).unwrap(),
        PgDuration(299123456000)
    );

    // Split into pieces it is text, and text names an interval, not a count.
    assert_eq!(
        value::duration("duration__gte", &Param::decode("1:2:3")).unwrap(),
        value::duration("duration__gte", &Param::decode("01:02:03")).unwrap()
    );
    assert!(value::durations("duration__in", &Param::decode("299123456000")).is_err());
    assert_eq!(
        value::durations("duration__in", &Param::decode("1:2:3")).unwrap(),
        vec![value::duration("duration__gte", &Param::decode("1:2:3")).unwrap()]
    );
}

#[test]
fn values_print_like_go() {
    // The reference prints a refused param with Go's %v, which switches to
    // exponential notation outside -4 <= exponent < 6.
    for (raw, want) in [
        ("0.5", "0.5"),
        ("3", "3"),
        ("1234567", "1.234567e+06"),
        ("150000", "150000"),
        ("1e+06", "1e+06"),
        ("299123456000", "2.99123456e+11"),
        ("0.0001", "0.0001"),
        ("0.00001", "1e-05"),
        ("-1234.5", "-1234.5"),
        ("1.5e+21", "1.5e+21"),
    ] {
        assert_eq!(Param::decode(raw).value_text(), want, "`%%v` of {raw}");
    }

    // `%s` refuses the number, which is what a bad params message echoes back.
    assert_eq!(Param::decode("0.5").display(), "%!s(float64=0.5)");
    assert_eq!(Param::decode("true").display(), "%!s(bool=true)");
    assert_eq!(Param::decode("null").display(), "%!s(<nil>)");
}

#[test]
fn empty_pieces_are_values() {
    assert_eq!(
        value::texts("file_name__in", &Param::decode(",")).unwrap(),
        vec!["", ""]
    );
    assert_eq!(
        value::texts("file_name__in", &Param::decode("a,,b")).unwrap(),
        vec!["a", "", "b"]
    );

    // A string parse is happy with an empty piece, an integer parse is not.
    assert!(value::i64s("score__in", &Param::decode("3,")).is_err());
}

#[test]
fn points_read_pgxs_text_form() {
    let ok = |value: &str| value::point("centroid__eq", &Param::decode(value)).unwrap();
    let err = |value: &str| value::point("centroid__eq", &Param::decode(value)).unwrap_err();

    assert_eq!(ok("(1,2)").x, 1.0);
    assert_eq!(ok(" 1,2 ").y, 2.0);
    assert_eq!(ok(r#""(1,2)""#).x, 1.0);
    // One byte is cut from each end, so the second half keeps its space.
    assert!(err("(1, 2)").contains(r#"parsing " 2": invalid syntax"#));
    assert!(err("(1)").ends_with("invalid length for point: 3)"));
    assert!(err("(1 2)").ends_with("invalid format for point)"));
}

#[test]
fn a_point_list_blames_the_first_piece() {
    let err = value::points("centroid__in", &Param::decode("(1,2),(3,4)")).unwrap_err();

    assert_eq!(
        err,
        r#"unparseable params centroid__in=(1,2),(3,4) ("(1" (string) could not be parsed with pgtype.Point.UnmarshalJSON for ParsePoint, invalid length for point: 2)"#
    );
}
