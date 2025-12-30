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
      let conn = format!("mysql://{}:{}@{}/{}?multi_statements=true", us_name ,db_pass, db_port, db_name);

      let pool = MySqlPoolOptions::new()
               .max_connections(5)
               .connect(conn.as_str()).await?;
      self.pool = Some(pool);
      Ok(())
   }

   pub async fn query(&self) -> Result<(), sqlx::Error>{
      let pool = &self.pool.as_ref().expect("ERROR").to_owned();
      let query = &self.query.show_query();

      let db_name = &self.environment.data_base_name;
      if query.contains("CREATE TABLE"){
         let table_name= extract_table_name(&self.query.show_query());

         sqlx::raw_sql(&format!("CREATE DATABASE IF NOT EXISTS `{}`", db_name))
            .execute(pool).await?;
         
         sqlx::raw_sql(&format!("USE `{}`", db_name))
            .execute(pool).await?;
         
         sqlx::raw_sql(&format!("DROP TABLE IF EXISTS `{}`", table_name))
            .execute(pool).await?;
      }

      sqlx::raw_sql(query).execute(pool).await?;
      Ok(())
   }
}

fn extract_table_name(query: &str) -> String {
   query.split_whitespace()
      .skip_while(|s| s.to_uppercase() != "TABLE")
      .nth(1)
      .unwrap_or("")
      .trim_end_matches('(')
      .to_string()
}