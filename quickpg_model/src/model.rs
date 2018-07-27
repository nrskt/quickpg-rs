use diesel::prelude::*;

table! {
    pg_tables {
        schemaname -> Text,
        tablename -> Text,
    }
}
