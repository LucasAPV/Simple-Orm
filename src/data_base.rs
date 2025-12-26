use std::mem::discriminant;

use crate::blueprint::BluePrint;
use crate::errors::Errors;
use crate::types::{Types, get_type_name};

#[derive(Clone, Debug)]
pub struct Column {
   pub name: String,
   pub values: Vec<Types>,
}

#[derive(Clone, Debug)]
pub struct Table {
   table_name: String,
   cols: Vec<Column>,
}

impl BluePrint for Table {
   fn create(table_name: String) -> Self {
      Self {
         table_name,
         cols: Vec::new(),
      }
   }

   fn add_column(&mut self, col: Column) -> Result<(), Errors> {
      if self.table_name.is_empty() {
         let error = format!("ERROR: TABLE NOT FOUND");
         return Err(Errors::TableNotFound(error));
      }
      self.cols.push(col);
      Ok(())
   }

   fn add_data(&mut self, col_name: String, data: Types) -> Result<(), Errors> {
      // Encontra a coluna ou retorna erro
      let column = self
         .cols
         .iter_mut()
         .find(|c| c.name == col_name)
         .ok_or_else(|| {
               Errors::ColumnNotFound(format!("ERROR: COLUMN {:?} NOT FOUND", col_name))
         })?;

      // Se a coluna está vazia, adiciona direto
      if column.values.is_empty() {
         column.values.push(data);
         return Ok(());
      }

      // Verifica se o tipo é compatível comparando as variantes do enum
      if discriminant(&column.values[0]) == discriminant(&data) {
         column.values.push(data);
         Ok(())
      } else {
         let expected_type = get_type_name(&column.values[0]);
         let received_type = get_type_name(&data);
         Err(Errors::TypeNotEqual(format!(
               "ERROR: COLUMN {} ACCEPTS {}, TRIED TO PUSH {}",
               col_name, expected_type, received_type
         )))
      }
   }

   fn select(&self, col_name: String, data: Option<Types>) -> Result<(), Errors> {
      match data {
         Some(value) => {
               if let Some(column) = self.cols.iter().find(|c| c.name == col_name) {
                  if let Some(found) = column.values.iter().find(|&v| v == &value) {
                     println!("'{}': \n\t{:?}", col_name, found);
                     return Ok(());
                  } else {
                     let error = format!("INDEX {:?} IS OUT OF BOUNDS IN {}", value, col_name);
                     return Err(Errors::IndexOutOfBounds(error));
                  }
               }
               Ok(())
         }

         None => {
               if let Some(column) = self.cols.iter().find(|c| c.name == col_name) {
                  println!("'{}':", col_name);
                  for (idx, value) in column.values.iter().enumerate() {
                     println!("   [{}] {:?}", idx + 1, value);
                     
                  }
               }
               Ok(())
         }
      }
   }

   fn find_by_id(&mut self, id: usize, col: String) -> Result<(), Errors> {
      if let Some(column) = self.cols.iter().find(|c| c.name == col) {
         if self.cols.len() < id {
               let error = format!("ID {id} IS OUT OF BOUNDS");
               return Err(Errors::IndexOutOfBounds(error));
         }

         if column.values.is_empty() && column.name.is_empty() {
               let error = format!("INDEX {id} NOT FOUND");
               return Err(Errors::IndexNotFound(error));
         }

         println!("{:?}", column.values[id - 1]);
      } else {
         let error = format!("COLUMN {col} NOT FOUND");
         return Err(Errors::ColumnNotFound(error));
      }

      return Ok(());
   }

   fn delete_column(&mut self, col: String) -> Result<(), Errors> {
      if self.table_name.is_empty() {
         let error = format!("ERROR: COLUMN NOT FOUND");
         return Err(Errors::ColumnNotFound(error));
      }
      
      let (index, _) = self.cols.iter()
         .enumerate().find(|(_, c)| *c.name == col).unwrap();

      self.cols.remove(index);
      Ok(())

   }
   
   fn delete_data (&mut self, col_name: String, data: Types) -> Result<(), Errors> {
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
         Ok(())
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
