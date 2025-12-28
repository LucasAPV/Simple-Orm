mod errors;
mod types;
mod data_base;
mod blueprint;
mod query_builder;
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
        clients.add_data(String::from("id"), Types::Int(i+1))?;
    }

    for i in 0..=2{
        clients.add_data(String::from("product_id"), Types::Int(i+1))?;
    }

    for i in 0..=2{
        clients.add_data(String::from("name"), Types::Text(clt[i].clone()))?;
    }

    for i in 0..=2{
        products.add_data(String::from("id"), Types::Int(i+1))?;
    }

    for i in 0..=2{
        products.add_data(String::from("name"), Types::Text(prods[i].clone()))?;
    }
    
    // let mut joined_table = 
    //         clients.join_table(&[String::from("name"), String::from("name")], products)?;
    
    // let name_clients =  joined_table.select(String::from("name.clients"), None)?;
    // let name_products = joined_table.select(String::from("name.products"), None)?;
    // println!("{name_clients}");
    // println!("{name_products}");

    // let found = joined_table.find_by_id(2, String::from("name.clients"))?;
    // println!("{found}");

    let clients_query = clients.get_query()?; 
    let products_query = products.get_query()?; 
    
    println!("{}", clients_query.show_query());
    println!("{}",format!("-").repeat(70));
    println!();
    println!("{}", products_query.show_query());
    Ok(())
}

