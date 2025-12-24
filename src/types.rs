#[derive(Debug, PartialEq, Clone)]
pub enum Types {
   Int(i32),
   #[allow(dead_code)]
   BigInt(i64),

   #[allow(dead_code)]
   Usize(usize),

   #[allow(dead_code)]
   Real(f64),

   #[allow(dead_code)]
   Bool(bool),
   Text(String),

   #[allow(dead_code)]
   Null,
}

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