//! Wire-format and JSON tests for the hand-written `point`/`polygon`/`interval` codecs.
//!
//! These talk to a real PostgreSQL (see `establish_connection`), so `cargo test` needs the
//! local database up.

use api::establish_connection;
use api::pg_types::{PgDuration, PgPoint, PgPolygon};
use api::schema::sql_types::{Point, Polygon};
use api::schema::{detection, video};
use diesel::prelude::*;
use diesel::sql_types::{Interval, Nullable};

#[derive(QueryableByName)]
struct RoundTrip {
    #[diesel(sql_type = Point)]
    p: PgPoint,
    #[diesel(sql_type = Polygon)]
    b: PgPolygon,
    #[diesel(sql_type = Nullable<Interval>)]
    d: Option<PgDuration>,
}

#[derive(QueryableByName)]
struct OnePolygon {
    #[diesel(sql_type = Polygon)]
    b: PgPolygon,
}

#[test]
fn pg_wire_round_trip() {
    let mut conn = establish_connection();
    let row = diesel::sql_query("SELECT $1::point AS p, $2::polygon AS b, $3::interval AS d")
        .bind::<Point, _>(PgPoint::new(1.5, -2.25))
        .bind::<Polygon, _>(PgPolygon::new(vec![
            PgPoint::new(0.0, 0.0),
            PgPoint::new(640.0, 0.0),
            PgPoint::new(640.0, 384.0),
            PgPoint::new(0.0, 384.0),
        ]))
        .bind::<Interval, _>(PgDuration(1_500_000_000))
        .get_result::<RoundTrip>(&mut conn)
        .expect("point/polygon/interval round trip");

    assert_eq!(row.p, PgPoint::new(1.5, -2.25));
    assert_eq!(row.b.0.len(), 4);
    assert_eq!(row.b.0[2], PgPoint::new(640.0, 384.0));
    assert_eq!(row.d, Some(PgDuration(1_500_000_000)));
}

#[test]
fn five_point_polygon_keeps_its_vertices() {
    let mut conn = establish_connection();
    let row = diesel::sql_query("SELECT '((1,1),(2,1),(2,2),(1,2),(1,1))'::polygon AS b")
        .get_result::<OnePolygon>(&mut conn)
        .expect("closed polygon");
    assert_eq!(row.b.0.len(), 5);
    assert_eq!(*row.b.0.last().unwrap(), PgPoint::new(1.0, 1.0));
}

#[test]
fn generated_columns_accept_the_custom_types() {
    let mut conn = establish_connection();
    // No rows in the local development database yet; this covers the generated
    // point/polygon/interval columns binding, selecting and decoding. Postgres has no
    // `=` for point/polygon (geometric `~=`, `@>`, `&&`, `<->` instead), so the point
    // column is only selected, never compared with `eq`.
    detection::table
        .select((detection::centroid, detection::bounding_box))
        .filter(detection::score.gt(0.5))
        .load::<(PgPoint, PgPolygon)>(&mut conn)
        .expect("select centroid/bounding_box");
    video::table
        .select(video::duration)
        .filter(video::duration.eq(PgDuration(1_000_000)))
        .load::<Option<PgDuration>>(&mut conn)
        .expect("select duration");
}

#[test]
fn json_matches_openapi_contract() {
    let point = PgPoint::new(1.5, -2.25);
    assert_eq!(
        serde_json::to_string(&point).unwrap(),
        r#"{"X":1.5,"Y":-2.25}"#
    );
    let polygon = PgPolygon::new(vec![point]);
    assert_eq!(
        serde_json::to_string(&polygon).unwrap(),
        r#"[{"X":1.5,"Y":-2.25}]"#
    );
    assert_eq!(
        serde_json::to_string(&PgDuration(2_000_000_000)).unwrap(),
        "2000000000"
    );
}
