use crate::{environment::Env, query_builder::Query};
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};
pub struct Migration{
   query: Query,
   environment: Env,
   pool: Option<Pool<MySql>>
}

impl Migration{
   pub fn create(
      query: Query,
      environment: Env
      ) -> Self{
         Self { query, environment, pool: None }
   }

   pub async fn connect(&mut self) -> Result<(), sqlx::Error>{
      let env = &self.environment;
      let db_name = env.data_base_name.clone();
      let db_pass = env.data_base_password.clone();
      let db_port = env.data_base_port.clone();
      let us_name: String = env.us_name.clone();
      let conn = format!("mysql://{}:{}@{}/{}", us_name ,db_pass, db_port, db_name);
      
      println!(
         "DADOS:\n 
         \t db_name: {db_name}\n
         \t db_pass: {db_pass}\n
         \t db_port: {db_port}\n
         \t conn: {conn}"
      );

      println!("Criando a pool");
      let pool = MySqlPoolOptions::new()
               .max_connections(5)
               .connect(conn.as_str()).await?;
      println!("Saindo da criacao a pool");
      self.pool = Some(pool);
      Ok(())
   }

   pub async fn query(&self) -> Result<(), sqlx::Error>{
      let pool = &self.pool.as_ref().expect("ERROR");
      let query = &self.query.show_query();

      sqlx::query(query).execute( pool.to_owned()).await?;
      Ok(())
   }
}