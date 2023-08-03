// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        username -> Text,
    }
}
