use std::collections::HashMap;
use mysql::{params, Value};
use mysql::prelude::Queryable;
use model_macro::traits::sql::GenerateTable;
use crate::config::cnn::get_connection;

pub fn create_table<T: GenerateTable>() -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    let sql = T::generate_sql_create_table();
    conn.exec_drop(&sql, ())?;
    Ok(())
}

pub fn insert<T: GenerateTable>(entity: &T) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    let sql = T::generate_sql_insert();
    let params_map = entity.to_params();
    let params = params_map.iter()
        .map(|(k, v)| (k.as_str(), Value::from(v.clone())))
        .collect::<Vec<_>>();

    conn.exec_drop(&sql, params)?;
    Ok(())
}

pub fn list<T: GenerateTable>() -> Result<Vec<T>, mysql::Error> {
    let mut conn = get_connection()?;
    let sql = T::generate_sql_select();
    let mut entities = Vec::new();

    let query_results = conn.query_iter(sql)?;

    for result_row in query_results {
        let row = result_row?;
        let mut row_data = HashMap::<String, String>::new();

        for (column, value) in row.columns().iter().zip(row.unwrap()) {
            let column_name = column.name_str().to_string();
            let val_str = match value {
                mysql::Value::Bytes(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                _ => String::from("Unsupported value type"),
            };
            row_data.insert(column_name, val_str);
        }

        let entity = T::from_row(row_data);
        entities.push(entity);
    }

    Ok(entities)
}

pub fn update<T: GenerateTable>(id: u32, entity: &T) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    let sql = T::generate_sql_update();
    let mut params_map = entity.to_params();
    params_map.insert("id".to_string(), id.to_string());
    let params = params_map.iter()
        .map(|(k, v)| (k.as_str(), Value::from(v.clone())))
        .collect::<Vec<_>>();

    conn.exec_drop(&sql, params)?;

    Ok(())
}

pub fn delete<T: GenerateTable>(id: u32) -> Result<(), mysql::Error> {
    let conn = get_connection();
    let sql = T::generate_sql_delete();
    let params = params! {
        "id" => id
    };


    conn?.exec_drop(&sql, params)?;
    Ok(())
}

pub fn drop_table<T: GenerateTable>() -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    let sql = T::generate_sql_drop_table();

    conn.exec_drop(&sql, ())?;

    Ok(())

}