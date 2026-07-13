use std::io;
use std::process::Command;
use crate::repository::generic_repository;

pub fn create_client() {
    clear_screen();
    let mut name = String::new();
    let mut phone = String::new();

    println!("Client name");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Client phone");
    io::stdin().read_line(&mut phone).expect("Failed to read line");

    let name = name.trim().to_string();
    let phone = phone.trim().to_string();

    generic_repository::create::<crate::models::client::Client>(&name.trim(), &phone.trim()).expect("Failed to create client");

    println!("Client created successfully!");
    pause_for_seconds(2);
    clear_screen();
}

pub fn list_clients() -> Result<(), mysql::Error> {
    clear_screen();
    let clients = generic_repository::list()?;
    if clients.len() > 0 {
        for client in clients {
            println!("--------------------");
            println!("ID: {}", client.id);
            println!("Name: {}", client.name);
            println!("Phone: {}", client.phone);
        }
    } else {
        println!("No clients found");
    }
    pause_until_press_enter();
    clear_screen();
    Ok(())
}

pub fn update_clients() -> Result<(), mysql::Error> {
    clear_screen();
    let mut id = String::new();
    let mut name = String::new();
    let mut phone = String::new();

    println!("Type the id of the client to be updated");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id = id.trim().parse::<u32>().expect("Invalid ID");

    println!("New client name");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("New client phone");
    io::stdin().read_line(&mut phone).expect("Failed to read line");

    generic_repository::update(id, &name.trim(), &phone.trim()).expect("Failed to update client");

    println!("Client updated successfully!");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

pub fn delete_client() -> Result<(), mysql::Error> {
    clear_screen();
    let mut id = String::new();

    println!("Type the id of the client do be deleted");
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id = id.trim().parse::<u32>().expect("Invalid ID");

    generic_repository::delete(id).expect("Failed to delete client");

    println!("Client deleted successfully!");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

fn pause_until_press_enter() {
    println!("Press enter to continue...");
    let mut _discard = String::new();
    io::stdin().read_line(&mut _discard).expect("Failed to read line");
    clear_screen();
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}

fn pause_for_seconds(seconds: u64) {
    std::thread::sleep(std::time::Duration::from_secs(seconds));
}