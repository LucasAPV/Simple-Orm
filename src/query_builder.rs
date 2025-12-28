use crate::types::Types;

#[derive(Debug, Clone, PartialEq)]
pub struct Query{
   query: String
}

impl Query {
   pub fn initialize_query() -> Self{
      Self { query: String::new() }
   }
   
   pub fn show_query(&self) -> &String{
      &self.query
   }

   pub fn append_select_table_name(&mut self, table_name: String){
      let res =format!("SELECT {} FROM INFORMATION_SCHEMA.TABLES;\n", table_name);
      self.query.push_str(&res);
   }

   pub fn append_select_table_columns(&mut self, table_name: String){
      let res = format!("SELECT * FROM {};\n", table_name);
      self.query.push_str(&res);
   }

   pub fn append_add_column(&mut self, table_name: String, col_name: String){
      let res = format!("ALTER TABLE {} ADD {};\n", table_name, col_name);
      self.query.push_str(&res);
   }

   pub fn append_add_data_to_column(&mut self, table_name: String, col_name: String, data: Types){
      let res = format!(
            "INSERT INTO {} ({}) VALUES ({:?});\n", 
            table_name, col_name, data
         );
      self.query.push_str(&res);
   }

   pub fn append_select(&mut self, data: Option<Types> ,table_name: String){
      let res = match data {
         Some(d) => {
            format!(
               "SELECT {:?} FROM {};\n", 
               d, table_name
            )
         }
         None => {
            format!(
               "SELECT * FROM {};\n", 
               table_name
            )
         }
      };
      self.query.push_str(&res);
   }

   pub fn append_find_by_id(&mut self, col_name: String, id: usize){
      let res = format!(
            "SELECT * FROM {} WHERE {}.id = {};\n", 
            col_name, col_name, id
         );
      
      self.query.push_str(&res);
   }

   pub fn append_delete_column(&mut self, table_name: String, col_name: String){
      let res = format!(
         "ALTER TABLE {} DROP COLUMN {};\n", 
         table_name, col_name
      );

      self.query.push_str(&res);
   }

   pub fn append_delete_column_data(&mut self, col_name: String, data: Types){
      let res = format!(
            "DELETE FROM {} WHERE {:?} == {:?};\n", 
            col_name, data, data
         );
      self.query.push_str(&res);
   }

   pub fn append_join_table(
      &mut self, col_a_name: String, col_b_name: String, 
      table_a_name: String, table_b_name: String,
   ){
      let res = format!(
         "SELECT {}, {} FROM {} INNER JOIN {};\n", 
         col_a_name, col_b_name, table_a_name, table_b_name
      );

      self.query.push_str(&res);
   }
}