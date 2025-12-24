#[derive(Debug)]
pub enum Errors {
   #[allow(dead_code)]
   IndexNotFound   (String),

   #[allow(dead_code)]
   TableNotFound   (String),

   #[allow(dead_code)]
   TypeNotEqual    (String),

   #[allow(dead_code)]
   ColumnNotFound  (String),

   #[allow(dead_code)]
   IndexOutOfBounds(String)
}