// @generated automatically by Diesel CLI.

diesel::table! {
    brands (id) {
        id -> Int4,
        name -> Varchar,
        image_url -> Varchar,
        info -> Varchar,
    }
}
