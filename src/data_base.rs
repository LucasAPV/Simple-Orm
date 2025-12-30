use std::mem::discriminant;
use crate::blueprint::BluePrint;
use crate::errors::Errors;
use crate::types::{Types};
use crate::query_builder::Query;
#[derive(Clone, Debug, PartialEq)]
pub struct Column {
   pub name: String,
   pub values: Vec<Types>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Table{
   query: Query,
   table_name: String,
   cols: Vec<Column>,
}

impl BluePrint for Table {
   fn create(table_name: String) -> Self {
      let mut query = Query::initialize_query();
      query.append_add_table(table_name.clone());
         Self {
            table_name,
            cols: Vec::new(),
            query: query
         }
   }

   fn get_query(&self) -> Result<Query, Errors>{
      Ok(self.query.clone())
   }

   fn get_table_name(&mut self) -> Result<String, Errors> {
      if self.table_name.is_empty(){
         return Err(Errors::TableNotFound(format!("TABLE NOT FOUND")));
      }

      self.query.append_select_table_name(self.table_name.clone());
      return Ok(self.table_name.to_owned());
   }
   
   fn get_table_columns(&mut self) -> Result<Vec<Column>, Errors> {
      if self.table_name.is_empty(){
         return Err(Errors::TableNotFound(format!("TABLE NOT FOUND")));
      }

      self.query.append_select_table_columns(self.table_name.clone());
      return Ok(self.cols.to_owned());
   }

   fn add_column(&mut self, col_name: String, col_type: Types) -> Result<String, Errors> {
      if self.table_name.is_empty() {
         let error = format!("ERROR: TABLE NOT FOUND");
         return Err(Errors::TableNotFound(error));
      }
      let col = Column { name: col_name.clone(), values: Vec::new() };
      let success = format!("{} IS CREATED", col.name);
      self.cols.push(col);
      self.query.append_add_column(self.table_name.clone(), col_name, col_type);
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
         column.values.push(data.clone());
         let success = format!("DATA ADDED");
         
         self.query.append_add_data_to_column( 
            self.table_name.clone(), col_name, data.clone()
         );

         return Ok(success);
      }

      if discriminant(&column.values[0]) == discriminant(&data) {
         column.values.push(data.clone());
         let success = format!("DATA ADDED");
         
         self.query.append_add_data_to_column( 
            self.table_name.clone(), col_name, data.clone()
         );

         return Ok(success);      
      } else {
         let expected_type = &column.values[0].get_type_name().unwrap();
         let received_type = &data.get_type_name().unwrap();
         Err(Errors::TypeNotEqual(format!(
               "ERROR: COLUMN {} ACCEPTS {}, TRIED TO PUSH {}",
               col_name, expected_type, received_type
         )))
      }
   }

   fn select(&mut self, col_name: String, data: Option<Types>) -> Result<String, Errors> {
      match data.clone() {
         Some(value) => {
               if let Some(column) = self.cols.iter()
               .find(|c| c.name == col_name) {
                  if let Some(found) = column.values.iter().find(|&v| v == &value) {
                     let res = format!("'{}': \n\t{:?}", col_name, found);
                     self.query.append_select(data, self.table_name.clone());
                     return Ok(res);
                  } else {
                     let error = format!("INDEX {:?} IS OUT OF BOUNDS IN {}", value, col_name);
                     return Err(Errors::IndexOutOfBounds(error));
                  }
               } else {
                  return Err(Errors::ColumnNotFound(format!("COLUMN {} NOT FOUND", col_name)))
               }
         }

         None => {
            let mut res = format!("'{}':", col_name);
               if let Some(column) = self.cols.iter().find(|c| c.name == col_name) {
                  for (idx, value) in column.values.iter().enumerate() {
                     res.push_str(format!("   [{}] {:?}", idx + 1, value).as_str());   
                  }
                  self.query.append_select(None, self.table_name.clone());
                  return Ok(res);
               }else {
                  return Err(Errors::TableNotFound(format!("TABLE NOT FOUND")))
               }
         }
      }
   }

   fn find_by_id(&mut self, id: usize, col_name: String) -> Result<String, Errors> {
      let mut res = String::new();
      
      if let Some(column) = self.cols.iter()
         .find(|c| c.name == col_name) {
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
            let error = format!("COLUMN {col_name} NOT FOUND");
            return Err(Errors::ColumnNotFound(error));
         }
         self.query.append_find_by_id(col_name, id);
      return Ok(res);
   }

   fn delete_column(&mut self, col_name: String) -> Result<String, Errors> {
      if self.table_name.is_empty() {
         let error = format!("ERROR: COLUMN NOT FOUND");
         return Err(Errors::ColumnNotFound(error));
      }
      
      let index = match self.cols.iter()
         .enumerate().find(|(_, c)| *c.name == col_name){
            Some((index, _)) => index,
            None => { return Err(Errors::TableNotFound(format!("TABLE NOT FOUND")))}
         };

      self.cols.remove(index);
      self.query.append_delete_column(self.table_name.clone(), col_name.clone());
      Ok(format!("{} REMOVED", col_name))

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
         
         self.query.append_delete_column_data(col_name.clone(), data.clone());

         Ok(format!("{:?} REMOVED FROM {} COLUMN", data, col_name))
      } else {
         let expected_type = &column.values[0].get_type_name().unwrap();
         let received_type = &data.get_type_name().unwrap();
         Err(Errors::TypeNotEqual(format!(
               "ERROR: COLUMN {} ACCEPTS {}, TRIED TO PUSH {}",
               col_name, expected_type, received_type
         )))
      }
   }

   fn contains_column(&mut self, col_name: String) -> Result<Column, Errors>{
      let option_col = &self.cols.iter()
                        .find(|c| *c.name == col_name).to_owned();
      
      match option_col {
         Some(col) => {return Ok(col.to_owned().to_owned())}
         None => {return Err(Errors::ColumnNotFound(format!("COLUMN {} NOT FOUND", col_name)));}
      }
   }

   fn join_table<T: BluePrint>(&mut self, cols_name: &[String], mut table: T) -> Result<Table, Errors> {
      let [col_a_name, col_b_name] = cols_name else {
         return Err(Errors::InvalidJoinOperation);
      };
      
      let mut col_a = self.contains_column(col_a_name.to_owned())?;
      col_a.name = format!("{}.{}", col_a_name, self.get_table_name()?);

      let mut col_b = table.contains_column(col_b_name.to_owned())?;
      col_b.name = format!("{}.{}", col_b_name, table.get_table_name()?);

      let ret_table = Table { 
            table_name: format!("{}_{}", &self.table_name, table.get_table_name()?),
            cols: vec![col_a, col_b],
            query: Query::initialize_query()
      };
      self.query.append_join_table(
         col_a_name.clone(), col_b_name.clone(), 
         self.table_name.clone(), table.get_table_name()?
      );
      Ok(ret_table)
   }
}