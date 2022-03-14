table! {
    accounts (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        profile -> Jsonb,
        plan -> Int4,
        is_active -> Bool,
        is_admin -> Bool,
        has_verified_email -> Bool,
        last_login -> Nullable<Timestamptz>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

table! {
    packages (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        downloads_count -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    packages,
);
