table! {
    apiacc (id) {
        id -> Varchar,
        apikey -> Nullable<Varchar>,
        apisec -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Varchar>,
    }
}

table! {
    branches (id) {
        id -> Varchar,
        merchants_id -> Nullable<Varchar>,
        branches_name -> Nullable<Varchar>,
        address -> Nullable<Text>,
        reward_rates -> Nullable<Numeric>,
        asset_code -> Nullable<Varchar>,
        minimum_spend -> Nullable<Numeric>,
        approval_code -> Nullable<Varchar>,
        logo_uri -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        granted_for -> Nullable<Varchar>,
    }
}

table! {
    documents (id) {
        id -> Varchar,
        documents_no -> Nullable<Varchar>,
        documenttype_id -> Nullable<Varchar>,
        document_uri -> Text,
        face_uri -> Text,
        issue_date -> Text,
        expire_date -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        updated_by -> Nullable<Varchar>,
    }
}

table! {
    documenttype (id) {
        id -> Varchar,
        document_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    hashval (id) {
        id -> Varchar,
        hashs -> Nullable<Text>,
        is_valid -> Bool,
        created_at -> Timestamp,
        created_by -> Nullable<Varchar>,
        updated_by -> Nullable<Varchar>,
    }
}

table! {
    merchants (id) {
        id -> Varchar,
        merchant_name -> Nullable<Varchar>,
        shortname -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
    }
}

table! {
    receipts (id) {
        id -> Varchar,
        receipt_no -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        location -> Nullable<Text>,
        image_uri -> Nullable<Text>,
        rewards -> Nullable<Numeric>,
        remark -> Nullable<Text>,
        status -> Nullable<Varchar>,
        created_at -> Timestamp,
        created_by -> Nullable<Varchar>,
        updated_by -> Nullable<Varchar>,
    }
}

table! {
    status (id) {
        id -> Varchar,
        status_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

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
    trxarchive (id) {
        id -> Varchar,
        block -> Nullable<Numeric>,
        hash -> Nullable<Varchar>,
        sender -> Nullable<Varchar>,
        destination -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        fee -> Nullable<Numeric>,
        memo -> Nullable<Text>,
        created_at -> Timestamp,
        created_by -> Nullable<Varchar>,
        updated_by -> Nullable<Varchar>,
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
    apiacc,
    branches,
    documents,
    documenttype,
    hashval,
    merchants,
    receipts,
    status,
    test,
    trxarchive,
    users,
);
