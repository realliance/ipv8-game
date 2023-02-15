// @generated automatically by Diesel CLI.

diesel::table! {
    chunks (x, y) {
        x -> Int8,
        y -> Int8,
        tiles -> Bytea,
    }
}

diesel::table! {
    complex_tiles (chunk_x, chunk_y, x, y) {
        chunk_x -> Int8,
        chunk_y -> Int8,
        x -> Int4,
        y -> Int4,
        metadata -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        credits -> Int8,
    }
}

diesel::table! {
    worlds (id) {
        id -> Int4,
        origin_time -> Timestamp,
        seed -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(chunks, complex_tiles, users, worlds,);
