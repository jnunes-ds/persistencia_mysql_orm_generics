use crate::models::client::Client;
use crate::repository::generic_repository;

pub fn init_tables() -> Result<(), mysql::Error> {
    generic_repository::create_table::<Client>()?;
    // futuramente: generic_repository::create_table::<Product>()?;
    // futuramente: generic_repository::create_table::<Order>()?;
    Ok(())
}