#[derive(Debug)]
#[allow(dead_code)]
pub enum Errors {
   IndexNotFound   (String),
   TableNotFound   (String),
   TypeNotEqual    (String),
   ColumnNotFound  (String),
   IndexOutOfBounds(String),
   InvalidJoinOperation
}
