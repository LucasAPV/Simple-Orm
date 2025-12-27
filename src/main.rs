mod errors;
mod types;
mod data_base;
mod blueprint;
use errors::Errors;

use crate::{blueprint::BluePrint, data_base::Table, types::Types};
fn main() -> Result<(), Errors>{
    
    let mut clients= Table::create("clients".to_string());
    clients.add_column("id".to_string())?;
    clients.add_column("name".to_string())?;
    clients.add_column("product_id".to_string())?;

    let mut products = Table::create("products".to_string());
    products.add_column("id".to_string())?;
    products.add_column("name".to_string())?;

    let prods = vec![
        String::from("prod1"),
        String::from("prod2"),
        String::from("prod3"),
    ];

    let clt = vec![
        String::from("clt1"),
        String::from("clt2"),
        String::from("clt3"),
    ];

    for i in 0..=2{
        clients.add_data(String::from("id"), Types::Int(i))?;
    }

    for i in 0..=2{
        clients.add_data(String::from("product_id"), Types::Int(i))?;
    }

    for i in 0..=2{
        clients.add_data(String::from("name"), Types::Text(clt[i].clone()))?;
    }

    for i in 0..=2{
        products.add_data(String::from("id"), Types::Int(i))?;
    }

    for i in 0..=2{
        products.add_data(String::from("name"), Types::Text(prods[i].clone()))?;
    }
    
    let output = clients
            .join_table(&[String::from("product_id"), String::from("name")], products)?;

    println!("{output}");
    Ok(())
}

