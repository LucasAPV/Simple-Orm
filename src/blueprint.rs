use crate::{data_base::Column, errors::Errors, types::Types};

pub trait BluePrint {
   fn create         (table_name: String                              )  -> Self;
   fn add_column     (&mut self, col: Column                          )  -> Result<(), Errors>;
   fn add_data       (&mut self, col_name: String, data: Types        )  -> Result<(), Errors>;
   fn select         (&self, col_name: String, data: Option<Types>    )  -> Result<(), Errors>;
   fn find_by_id     (&mut self, id: usize, col: String               )  -> Result<(), Errors>;
   fn delete_data    (&mut self, col_name: String, data: Option<Types>)  -> Result<(), Errors>;
   fn delete_column  (&mut self, col: Column                          )  -> Result<(), Errors>; 
}
