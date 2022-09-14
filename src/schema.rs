// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    accounts (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        is_active -> Bool,
        is_admin -> Bool,
        has_verified_email -> Bool,
        last_login -> Nullable<Timestamptz>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        uid -> Int4,
        name -> Text,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    accounts,
    items,
);
