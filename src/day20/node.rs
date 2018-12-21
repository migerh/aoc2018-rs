#[derive(Debug)]
pub enum Directions {
  Content(String),
  Options(Vec<Directions>),
  Concat(Vec<Directions>),
}

#[derive(Debug)]
pub struct Node {
  pub buffer: String,
  pub children: Vec<Box<Node>>,
}

impl Node {
  pub fn new() -> Node {
    let buffer = "".to_string();
    let children = vec![];

    Node { buffer, children }
  }
}