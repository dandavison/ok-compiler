// This was a bug. Should the compiler detect that prev_trip_id was never changing?

    let mut prev_trip_id = 0;
    for postgres_row in postgres_rows {
        let trip_id: i32 = postgres_row.get("trip_id");
        dbg!(&prev_trip_id);
        dbg!(&trip_id);
        if prev_trip_id > 0 && trip_id != prev_trip_id {
            trips.push(Trip {
                name: postgres_row.get("trip_name"),
                days,
            });
            days = Vec::<SiteDay>::new();
            prev_trip_id = trip_id;
        } else {
            days.push(SiteDay {
                day: postgres_row.get("day"),
                site: models::Site {
                    id: postgres_row.get("site_id"),
                    name: postgres_row.get("site_name"),
                    lat: postgres_row.get("site_lat"),
                    lng: postgres_row.get("site_lng"),
                },
            });
        }
    }
