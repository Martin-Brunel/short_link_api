// @generated automatically by Diesel CLI.

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
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    link,
    liste,
    user,
);
