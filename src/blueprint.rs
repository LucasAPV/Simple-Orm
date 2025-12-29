use crate::{data_base::{Column, Table}, errors::Errors, query_builder::Query, types::Types};

#[allow(dead_code)]
pub trait BluePrint {
   fn create                  (table_name: String                              ) -> Self;
   fn get_table_name          (&mut self                                       ) -> Result<String, Errors>; 
   fn get_table_columns       (&mut self                                       ) -> Result<Vec<Column>, Errors>; 
   fn add_column              (&mut self, col_name: String                     ) -> Result<String, Errors>;
   fn add_data                (&mut self, col_name: String, data: Types        ) -> Result<String, Errors>;
   fn select                  (&mut self, col_name: String, data: Option<Types>) -> Result<String, Errors>;
   fn find_by_id              (&mut self, id: usize, col: String               ) -> Result<String, Errors>;
   fn delete_data             (&mut self, col_name: String, data: Types        ) -> Result<String, Errors>;
   fn delete_column           (&mut self, col_name: String                     ) -> Result<String, Errors>; 
   fn join_table<T: BluePrint>(&mut self, cols_name: &[String], table: T       ) -> Result<Table , Errors>;
   fn contains_column         (&mut self, col_name: String                     ) -> Result<Column, Errors>;
   fn get_query               (&self                                           ) -> Result<Query, Errors>;
}