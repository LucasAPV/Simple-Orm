mod errors;
mod types;
mod data_base;
mod blueprint;
use errors::Errors;
use types::{Types};

use crate::{blueprint::BluePrint, data_base::{Column, Table}};
fn main() -> Result<(), Errors>{
    let mut my_table = Table::create(String::from("my_table"));

    // Adicionar colunas
    my_table.add_column(Column {
        name: String::from("id"),
        values: Vec::new(),
    })?;

    my_table.add_column(Column {
        name: String::from("clients"),
        values: Vec::new(),
    })?;

    my_table.add_data(String::from("id"), Types::Int(1))?;
    my_table.add_data(String::from("id"), Types::Int(2))?;
    my_table.add_data(String::from("id"), Types::Int(3))?;

    let clients = vec![
        String::from("Carlos"),
        String::from("Amanda"),
        String::from("José"),
        String::from("Antonio"),
    ];

    for client in clients {
        my_table.add_data(String::from("clients"), Types::Text(client))?;
    }

    println!("\n=== Testando SELECT ===\n");
    
    my_table.select(String::from("id"), None)?;
    println!();
    my_table.select(String::from("clients"), None)?;
    println!();
    my_table.select(String::from("clients"), Some(Types::Text(String::from("Amanda"))))?;
    println!();
    //my_table.select(String::from("id"), Some(Types::Int(9000)))?;

    println!("\n=== Testando FIND_BY_ID ===\n");
    my_table.find_by_id(1, String::from("clients"))?;

    println!("\n=== Testando DELETES ===");
    my_table.delete_data(String::from("clients"), Types::Text(String::from("Amanda")))?;
    my_table.delete_column(String::from("id"))?;

    println!("\n=== Tabela Completa ===");
    println!("{:#?}", my_table);

    Ok(())
}