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

#[allow(dead_code)]
pub fn get_type_name(value: &Types) -> &'static str {
   match value {
      Types::BigInt(_) => "BIGINT",
      Types::Bool(_) => "BOOL",
      Types::Real(_) => "REAL",
      Types::Text(_) => "TEXT",
      Types::Usize(_) => "USIZE",
      Types::Int(_) => "INT",
      _ => "UNKNOWN",
   }
}