// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Integer,
        challenge_proof -> Text,
        redirect_url -> Text,
    }
}
