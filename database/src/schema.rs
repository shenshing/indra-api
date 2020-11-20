table! {
    test (id) {
        id -> Int4,
        password -> Varchar,
        description -> Nullable<Text>,
        active -> Nullable<Bool>,
        item -> Nullable<Varchar>,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        first_name -> Nullable<Varchar>,
        mid_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        email -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        profile_img -> Nullable<Text>,
        wallet -> Nullable<Varchar>,
        seed -> Nullable<Varchar>,
        password -> Varchar,
        temp_token -> Nullable<Text>,
        pin -> Nullable<Varchar>,
        user_status -> Nullable<Varchar>,
        is_partner -> Bool,
        nationality -> Nullable<Varchar>,
        occupation -> Nullable<Varchar>,
        phonenumber -> Nullable<Varchar>,
        documents_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        address -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        updated_by -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    test,
    users,
);

