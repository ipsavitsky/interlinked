// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Integer,
        challenge_proof -> Text,
        payload -> Text,
        record_type -> Text,
    }
}
