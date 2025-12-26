use crate::{data_base::Column, errors::Errors, types::Types};

pub trait BluePrint {
   fn create         (table_name: String                              )  -> Self;
   fn add_column     (&mut self, col: Column                          )  -> Result<String, Errors>;
   fn add_data       (&mut self, col_name: String, data: Types        )  -> Result<String, Errors>;
   fn select         (&self, col_name: String, data: Option<Types>    )  -> Result<String, Errors>;
   fn find_by_id     (&mut self, id: usize, col: String               )  -> Result<String, Errors>;
   fn delete_data    (&mut self, col_name: String, data: Types        )  -> Result<String, Errors>;
   fn delete_column  (&mut self, col: String                          )  -> Result<String, Errors>; 
}
