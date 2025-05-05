use diesel::table;

table! {
    descriptions (key) {
        key -> Text,
        value -> Text,
    }
}
