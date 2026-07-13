use mysql::params;
use mysql::prelude::Queryable;
use model_macro::traits::sql::GenerateTable;
use crate::config::cnn::get_connection;
use crate::models::client::Client;

pub fn create<T: GenerateTable>(name: &str, phone: &str) -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;

    conn.exec_drop(
        r"INSERT INTO clients (name, phone) values (:name, :phone)",
        params! {
            "name" => name,
            "phone" => phone,
        }
    )?;

    Ok(())
}

pub fn list() -> Result<Vec<Client>, mysql::Error> {
    let mut conn = get_connection()?;

    let clients = conn.query_map(
        "SELECT id, name, phone FROM clients",
        |(id, name, phone)| {
            Client {
                id,
                name,
                phone,
            }
        }
    )?;

    Ok(clients)
}

pub fn update(id: u32, name: &str, phone: &str) -> Result<(), mysql::Error> {
    let conn = get_connection();

    conn?.exec_drop(
        r"UPDATE clients SET name=:name, phone=:phone WHERE id=:id",
        params! {
            "id" => id,
            "name" => name,
            "phone" => phone,
        }
    )?;

    Ok(())
}

pub fn delete(id: u32) -> Result<(), mysql::Error> {
    let conn = get_connection();

    conn?.exec_drop(
        "DELETE FROM clients WHERE id=:id",
        params! {
            "id" => id
        }
    )?;

    Ok(())
}
