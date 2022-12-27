// @generated automatically by Diesel CLI.

diesel::table! {
    link (id) {
        id -> Integer,
        user_id -> Integer,
        url -> Varchar,
        code -> Varchar,
        is_deleted -> Tinyint,
        created_at -> Datetime,
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
