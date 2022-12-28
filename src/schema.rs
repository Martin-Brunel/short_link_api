// @generated automatically by Diesel CLI.

diesel::table! {
    brand (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
        is_deleted -> Tinyint,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    link (id) {
        id -> Integer,
        url -> Varchar,
        code -> Varchar,
        user_id -> Integer,
        created_at -> Datetime,
        is_deleted -> Tinyint,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        brand_id -> Integer,
    }
}

diesel::table! {
    liste (id) {
        id -> Integer,
        libelle -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        roles -> Varchar,
        created_at -> Datetime,
        is_deleted -> Tinyint,
        deleted_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        brand_id -> Integer,
    }
}

diesel::joinable!(link -> brand (brand_id));
diesel::joinable!(user -> brand (brand_id));

diesel::allow_tables_to_appear_in_same_query!(
    brand,
    link,
    liste,
    user,
);
