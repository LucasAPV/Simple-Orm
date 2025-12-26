use std::mem::discriminant;

use crate::blueprint::BluePrint;
use crate::errors::Errors;
use crate::types::{Types, get_type_name};

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
   pub name: String,
   pub values: Vec<Types>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
   pub table_name: String,
   pub cols: Vec<Column>,
}

impl BluePrint for Table {
   fn create(table_name: String) -> Self {
      Self {
         table_name,
         cols: Vec::new(),
      }
   }

   fn add_column(&mut self, col: Column) -> Result<String, Errors> {
      if self.table_name.is_empty() {
         let error = format!("ERROR: TABLE NOT FOUND");
         return Err(Errors::TableNotFound(error));
      }
      let success = format!("{} IS CREATED", col.name);
      self.cols.push(col);
      Ok(success)
   }

   fn add_data(&mut self, col_name: String, data: Types) -> Result<String, Errors> {
      let column = self
         .cols
         .iter_mut()
         .find(|c| c.name == col_name)
         .ok_or_else(|| {
               Errors::ColumnNotFound(format!("ERROR: COLUMN {:?} NOT FOUND", col_name))
         })?;

      if column.values.is_empty() {
         column.values.push(data);
         let success = format!("DATA ADDED");
         return Ok(success);
      }

      if discriminant(&column.values[0]) == discriminant(&data) {
         column.values.push(data);
         let success = format!("DATA ADDED");
         return Ok(success);      
      } else {
         let expected_type = get_type_name(&column.values[0]);
         let received_type = get_type_name(&data);
         Err(Errors::TypeNotEqual(format!(
               "ERROR: COLUMN {} ACCEPTS {}, TRIED TO PUSH {}",
               col_name, expected_type, received_type
         )))
      }
   }

   fn select(&self, col_name: String, data: Option<Types>) -> Result<String, Errors> {
      match data {
         Some(value) => {
               if let Some(column) = self.cols.iter()
               .find(|c| c.name == col_name) {
                  if let Some(found) = column.values.iter().find(|&v| v == &value) {
                     let res = format!("'{}': \n\t{:?}", col_name, found);
                     return Ok(res);
                  } else {
                     let error = format!("INDEX {:?} IS OUT OF BOUNDS IN {}", value, col_name);
                     return Err(Errors::IndexOutOfBounds(error));
                  }
               } else {
                  return Err(Errors::ColumnNotFound(format!("COLUMN NOT FOUND")))
               }
         }

         None => {
            let mut res = format!("'{}':", col_name);
               if let Some(column) = self.cols.iter().find(|c| c.name == col_name) {
                  for (idx, value) in column.values.iter().enumerate() {
                     res.push_str(format!("   [{}] {:?}", idx + 1, value).as_str());   
                  }
                  return Ok(res);
               }else {
                  return Err(Errors::TableNotFound(format!("TABLE NOT FOUND")))
               }
         }
      }
   }

   fn find_by_id(&mut self, id: usize, col: String) -> Result<String, Errors> {
      let mut res = String::new();
      
      if let Some(column) = self.cols.iter()
         .find(|c| c.name == col) {
            if self.cols.len() < id {
                  let error = format!("ID {id} IS OUT OF BOUNDS");
                  return Err(Errors::IndexOutOfBounds(error));
            }

            if column.values.is_empty() && column.name.is_empty() {
                  let error = format!("INDEX {id} NOT FOUND");
                  return Err(Errors::IndexNotFound(error));
            }
            let ap = format!("{:?}", column.values[id - 1]); 
            res.push_str(ap.as_str());
         } else {
            let error = format!("COLUMN {col} NOT FOUND");
            return Err(Errors::ColumnNotFound(error));
         }

      return Ok(res);
   }

   fn delete_column(&mut self, col: String) -> Result<String, Errors> {
      if self.table_name.is_empty() {
         let error = format!("ERROR: COLUMN NOT FOUND");
         return Err(Errors::ColumnNotFound(error));
      }
      
      let index = match self.cols.iter()
         .enumerate().find(|(_, c)| *c.name == col){
            Some((index, _)) => index,
            None => { return Err(Errors::TableNotFound(format!("TABLE NOT FOUND")))}
         };

      self.cols.remove(index);
      Ok(format!("{} REMOVED", col))

   }
   
   fn delete_data (&mut self, col_name: String, data: Types) -> Result<String, Errors> {
      let (index, _) = self.cols.iter()
         .enumerate().find(|(_, c)| *c.name == col_name).unwrap();
            
      let column = self
         .cols
         .iter_mut()
         .find(|c| c.name == col_name)
         .ok_or_else(|| {
               Errors::ColumnNotFound(format!("ERROR: COLUMN {:?} NOT FOUND", col_name))
         })?;

      if column.values.is_empty() {
         return Err(Errors::IndexNotFound(format!("ERROR: COLUMN {:?} IS EMPTY", col_name)));
      }
      
      if discriminant(&column.values[0]) == discriminant(&data) {
         column.values.remove(index);
         Ok(format!("{:?} REMOVED FROM {} COLUMN", data, col_name))
      } else {
         let expected_type = get_type_name(&column.values[0]);
         let received_type = get_type_name(&data);
         Err(Errors::TypeNotEqual(format!(
               "ERROR: COLUMN {} ACCEPTS {}, TRIED TO PUSH {}",
               col_name, expected_type, received_type
         )))
      }
   }
}
