// @generated automatically by Diesel CLI.

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
    liste,
    user,
);
