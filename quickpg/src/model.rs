use postgres::Connection;
use std::vec;

pub trait Queryable {
    fn sql(&self) -> String;
}

#[derive(Debug)]
pub struct TableList {
    pub tables: Vec<PostgresTable>,
}

impl TableList {
    pub fn new(conn: &Connection) -> TableList {
        get_tables(conn)
    }
}

fn get_tables(conn: &Connection) -> TableList {
    let mut results: vec::Vec<PostgresTable> = vec::Vec::new();
    for row in &conn.query(
        "SELECT schemaname, tablename FROM pg_catalog.pg_tables",
        &[],
    ).unwrap()
    {
        results.push(PostgresTable::new(row.get(0), row.get(1)))
    }
    TableList { tables: results }
}

#[derive(Debug)]
pub struct PostgresTable {
    pub schema_name: String,
    pub table_name: String,
}

#[derive(Debug)]
pub struct PostgresValue(String);

impl PostgresTable {
    pub fn new(schema_name: String, table_name: String) -> PostgresTable {
        PostgresTable {
            schema_name: schema_name,
            table_name: table_name,
        }
    }
    pub fn fullname(&self) -> String {
        format!("{}.{}", self.schema_name, self.table_name)
    }

    pub fn get(&self, conn: &Connection) -> Vec<String> {
        let rows = conn.query(&self.sql(), &[]).unwrap();
        let mut results: Vec<String> = Vec::new();
        for row in rows.into_iter() {
            results.push(row.get("content"))
        }
        results
    }
}

impl Queryable for PostgresTable {
    fn sql(&self) -> String {
        format!(
            "SELECT rtrim(ltrim(replace({}::text, ',', ', '), '('), ')') AS content FROM \"{}\".\"{}\" LIMIT 10",
            self.table_name, self.schema_name, self.table_name
        )
    }
}
