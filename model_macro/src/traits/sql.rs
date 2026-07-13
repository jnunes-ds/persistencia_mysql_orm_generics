use std::collections::HashMap;

pub trait GenerateTable {
    fn to_params(&self) -> HashMap<String, String>;
    fn from_row(row: HashMap<String, String>) -> Self;
    fn generate_sql_create_table() -> String;
    fn generate_sql_drop_table() -> String;
    fn generate_sql_insert() -> String;
    fn generate_sql_update() -> String;
    fn generate_sql_delete() -> String;
    fn generate_sql_select() -> String;
}