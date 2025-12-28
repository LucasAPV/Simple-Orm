#[derive(Debug, Clone, PartialEq)]
pub struct Query{
   query: String
}

impl Query {
   pub fn initialize_query() -> Self{
      Self { query: String::new() }
   }

   pub fn append(&mut self, q: String){
      self.query.push_str(&q);
   }

   pub fn show_query(&self) -> &String{
      &self.query
   }
}