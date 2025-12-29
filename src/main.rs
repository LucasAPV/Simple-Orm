mod errors;
mod types;
mod data_base;
mod blueprint;
mod query_builder;
mod migration;
mod environment;
use migration::Migration;
use crate::{blueprint::BluePrint, data_base::Table, environment::Env};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    let clients= Table::create("clients".to_string());
    let products = Table::create("products".to_string());

    //Queries to the tables
    let clients_query = clients.get_query().unwrap(); 
    let products_query = products.get_query().unwrap();
    
    let db_type = String::from("MySQL");
    let db_port = String::from("127.0.0.1:3306");
    let us_name = String::from("root");
    let db_pass = String::from("root");
    let db_name = String::from("test");
    
    //Environment for the data_base
    let env = Env
        ::create(
            db_type, 
            db_port, 
            db_name, 
            db_pass,
            us_name);

    println!("Foi para a criacao da mg_clients");
    //Migration of the clients table
    let mut mg_clients = Migration::create(clients_query, env.clone());
    println!("Saiu da criacao da mg_clients");
    println!("Foi para a conexao");
    mg_clients.connect().await?;
    println!("Saiu da conexao");
    println!("Foi para a query");
    mg_clients.query().await?;
    println!("Saiu da query");

    //Migration of the products table
    let mut mg_products = Migration::create(products_query, env);
    mg_products.connect().await?;
    mg_products.query().await?;
    
    Ok(())
}

