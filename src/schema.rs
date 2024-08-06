// @generated automatically by Diesel CLI.

diesel::table! {
    board (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        #[max_length = 50]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    comment (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        post_id -> Nullable<Int4>,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    post (id) {
        id -> Int4,
        board_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        hash_tags -> Nullable<Array<Nullable<Text>>>,
        like_count -> Nullable<Int4>,
        view_count -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 50]
        nickname -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(comment -> post (post_id));
diesel::joinable!(comment -> user (user_id));
diesel::joinable!(post -> board (board_id));
diesel::joinable!(post -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    board,
    comment,
    post,
    user,
);
