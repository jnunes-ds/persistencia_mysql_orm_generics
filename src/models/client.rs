use model_macro::create_struct_and_metadata;

create_struct_and_metadata! {
    "clients" => Client {
        id: u32, "autoincrement",
        name: String, "varchar(100)",
        phone: String, "varchar(15)",
    }
}