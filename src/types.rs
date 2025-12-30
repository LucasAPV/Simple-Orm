use std::usize;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]

pub enum Types {
   Int(i32),
   BigInt(i64),
   Usize(usize),
   Real(f64),
   Bool(bool),
   Text(String),
   Null,
}

impl Types{
   pub fn get_type_name(&self) -> Option<String> {
      match self {
         Types::BigInt(_) => Some("BIGINT".to_string()),
         Types::Bool(_)   => Some("BOOL".to_string()),
         Types::Real(_)   => Some("REAL".to_string()),
         Types::Text(_)   => Some("TEXT".to_string()),
         Types::Usize(_)  => Some("USIZE".to_string()),
         Types::Int(_)    => Some("INT".to_string()),
         _                => None,
      }
   }

   pub fn get_content(&self) -> Option<String>{
      match self {
         Types::BigInt(d) => Some(d.to_string()),
         Types::Bool(d)   => Some(d.to_string()),
         Types::Real(d)   => Some(d.to_string()),
         Types::Text(d)   => Some(d.to_string()),
         Types::Usize(d)  => Some(d.to_string()),
         Types::Int(d)    => Some(d.to_string()),
         _                => None,
      }
   }
}
