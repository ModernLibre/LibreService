use diesel::table;

table! {
    book (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        description -> Text,
        status -> Int4,
        rating -> Float8,
        added_date -> Date,
    }
}
