table! {
    users (id) {
        id -> Uuid,
        credits -> Int8,
    }
}

table! {
    worlds (id) {
        id -> Int4,
        origin_time -> Timestamp,
        seed -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    worlds,
);
