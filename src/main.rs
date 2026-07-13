#[macro_use]
extern crate model_macro;

mod models;
mod screen;
mod config;
mod repository;

use screen::display;

use std::io;
use model_macro::traits::sql::GenerateTable;

fn main() {
    // generic_repository::drop_table::<Client>();
    // generic_repository::create_table::<Client>();
    // return;

    loop {
        println!("CRUD Clients");
        println!("\
1. Create Clients
2. List Clients
3. Update Client by id
4. Delete Client by id
5. Quit\n\n
        ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => display::create_client(),
            "2" => {
                if let Err(e) = display::list_clients() {
                    eprintln!("Error listing clients: {}", e);
                }
            },
            "3" => {
                if let Err(e) = display::update_clients() {
                    eprintln!("Error updating clients: {}", e);
                }
            },
            "4" => {
                if let Err(e) = display::delete_client() {
                    eprintln!("Error deleting clients: {}", e);
                }
            },
            "5" => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice"),
        }
    }
}
