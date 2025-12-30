mod blueprint;
mod data_base;
mod environment;
mod errors;
mod migration;
mod query_builder;
mod types;
use crate::{blueprint::BluePrint, data_base::Table, environment::Env, types::Types};
use migration::Migration;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut clients = Table::create("clients".to_string());
    let products = Table::create("products".to_string());
    clients.add_column("name".to_string(), Types::Text("".to_string())).expect("ERRORS");
    clients.add_column("cpf".to_string(), Types::Int(0)).expect("ERROS");
    clients.add_data("name".to_string(), Types::Text("clt1".to_string())).expect("ERROS");
    
    //Queries to the tables
    let clients_query = clients.get_query().unwrap();
    let products_query = products.get_query().unwrap();

    let db_type = String::from("MySQL");
    let db_port = String::from("127.0.0.1:3306");
    let us_name = String::from("root");
    let db_pass = String::from("root");
    let db_name = String::from("test");

    let env = create_env(db_type, db_port, db_name, db_pass, us_name);

    //Migration of the clients table
    let mut mg_clients = Migration::create(clients_query, env.clone());
    println!("ENTERING CONNECT");
    mg_clients.connect().await?;
    println!("ENTERING QUERY");
    mg_clients.query().await?;
    println!("EXITING QUERY");

    //Migration of the products table
    let mut mg_products = Migration::create(products_query, env);
    mg_products.connect().await?;
    mg_products.query().await?;

    Ok(())
}

fn create_env(
    db_type: String,
    db_port: String,
    db_name: String,
    db_pass: String,
    us_name: String,
) -> Env {
    Env::create(db_type, db_port, db_name, db_pass, us_name)
}
