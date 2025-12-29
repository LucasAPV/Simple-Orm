
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Env{
   pub data_base_type    : String,
   pub data_base_port    : String,
   pub data_base_name    : String,
   pub data_base_password: String,
   pub us_name           : String
}

impl Env{
   pub fn create(
      data_base_type    : String,
      data_base_port    : String,
      data_base_name    : String,
      data_base_password: String,
      us_name: String
   ) -> Self{

      Self { 
         data_base_type, 
         data_base_port, 
         data_base_name, 
         data_base_password, 
         us_name
      }

   }
}