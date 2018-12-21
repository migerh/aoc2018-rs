#[derive(Debug)]
pub enum Directions {
  Content(String),
  Options(Vec<Directions>),
  Concat(Vec<Directions>),
}
