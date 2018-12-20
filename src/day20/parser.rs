use std::borrow::BorrowMut;
use super::node::Node;

fn preprocess_input(s: &str) -> String {
  s.chars().rev().collect()
}

fn recursive_parse(mut s: &mut String) -> Vec<Box<Node>> {
  let mut nodes = vec![];
  let mut node: Box<Node> = Box::new(Node::new());
  while let Some(c) = s.pop() {
    match c {
      '$' | '^' => {},
      '(' => {
        let b: &mut Node = node.borrow_mut();
        let mut children = recursive_parse(&mut s);
        b.children.append(&mut children);
        nodes.push(node);
        node = Box::new(Node::new());
      },
      ')' => {
        nodes.push(node);
        return nodes;
      },
      '|' => {
        nodes.push(node);
        node = Box::new(Node::new());
      },
      _ => {
        let b: &mut Node = node.borrow_mut();
        b.buffer.push(c);
      }
    }
  }

  nodes.push(node);
  nodes
}

pub fn parse(s: &str) -> Vec<Box<Node>> {
  let mut input = preprocess_input(s);
  recursive_parse(&mut input)
}