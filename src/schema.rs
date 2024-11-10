use diesel::table;

table! {
    book (id) {
        id -> Int4,
        file_url -> Varchar,
        cover_url -> Varchar,
        title -> Varchar,
        author -> Varchar,
        description -> Text,
        status -> Int4,
        rating -> Float8,
        added_date -> Date,
    }
}

table! {
    chapter (id) {
        id -> Int4,
        title -> Varchar,
        index -> Int4,
        content -> Text,
        level -> Int4,
        parent_id -> Int4,
        book_id -> Int4,
        created_time -> Date,
        updated_time -> Date,
    }
}
