
use std::{rc::{Rc, Weak}, cell::RefCell, borrow::BorrowMut, process::Command};

use std::collections::HashMap;

use core::fmt::Debug;

use super::{Token, varnum, functions, operators::{self, Operator}};

pub trait StrMake<T> {
	fn from_str(str: &str) -> T;
}
pub trait ZeroMake<T> {
	fn from_zero() -> T;
}
pub trait UnityMake<T> {
	fn from_unity() -> T;
}

pub trait LeafMake<T>: StrMake<T> + ZeroMake<T> + UnityMake<T> {}

// Required arithmatic traits
use std::ops::{Add, Sub, Mul, Div, Neg};
pub trait Pow<Rhs = Self> {
	type Output;
	fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait ExprType<T>: Add<T> + Sub<T> + Mul<T> + Div<T> + Pow<T> + Neg
	+ LeafMake<T> + Debug {}



pub enum NodeType {
	LeafNode,
	UnaryOperator,
	BinaryOperator,
	Function,
}

pub enum CommutativityType {
	Commutative,
	AntiCommutative,
}

#[derive(Debug, Clone)]
pub struct Node<'a, T> {
	expr: &'a Expression<T>,
	node_type: NodeType,
	token: Token,
	commutativity: Option<CommutativityType>,
	order_of_inputs: Option<Vec<(CommutativityType,Vec<usize>)>>,
	parents: Vec<&'a str>,
	children: Vec<&'a str>,
}

impl<T> Node<'_, T> {
	
	fn get_full_name(&self) -> anyhow::Result<&str> {
		let child_full_names: Vec<&str>;
		for childstr in self.children {
			child_full_names.push(self.expr.get_node(childstr)?.get_full_name());
		}

		// Sort
		for input_group in self.order_of_inputs.unwrap_or().iter() {
			let group_strs = Vec::with_capacity((input_group.1).count());
			for input in input_group.1 {
				group_strs.push(child_full_names[input]);
			}

			if group_strs.is_sorted() { 
				continue; 
			}

			group_strs.sort_unstable();

			for (i, input) in input_group.1.iter().enumerate() {
				child_full_names[input] = group_strs[i];
			}
		}

		let tokstr = self.token.stringify();
		let capacity = 0;
		for child_full_name in child_full_names {
			capacity += child_full_name.len() + 1;
		}
		capacity += tokstr.len();
		let full_name = String::with_capacity(capacity);
		for child_full_name in child_full_names {
			full_name += child_full_name;
		}

		return full_name;
	}

}

pub struct Expression<T> {
	map: HashMap<&'a str, Node<T>>,
}

impl<T> Expression<T> {

	fn from_tokens(tokens: &Vec<Token>) -> anyhow::Result<Self> {

		let expr = Self {
			map: HashMap::new(),
		};
 
		expr.build_graph(tokens);

	}

	fn build_graph(&self, tokens: &Vec<Token>) {
		let nodestack: Vec<&Node<T>>;
		for tok in tokens {
			let nodetype = Expression::get_node_type(tok);
			let tokstr = tok.stringify();

			match nodetype {
				NodeType::LeafNode => {
					let matchnode = self.map.get(tokstr);
					if let Some(matchnode) = matchnode {
						nodestack.push(matchnode);
					}
					else {
						let ret = self.map.try_insert(tokstr, Node 
						{ 
							expr: self, 
							node_type: NodeType::LeafNode,
							commutativity: None,
							order_of_input: None,
							token: tok, 
							parents: vec![], 
							children: vec![],
						});
						nodestack.push(ret.unwrap()?);
					}
				},
				NodeType::UnaryOperator => {
					let prevnode = nodestack.pop()?;
					let nodename = prevnode.get
				},
				NodeType::BinaryOperator => {

				},
				NodeType::Function => {

				}
			}

			let tokname = tok.stringify();
			if let Some(node) = self.map.get(tokname) {
				
			}
			
		}
	}


	fn get_node_type(token: Token) -> Option<NodeType> {
		match token {
			Token::Number(_) 	=> return Some(NodeType::LeafNode),
			Token::Zero 		=> return Some(NodeType::LeafNode),
			Token::Unity 		=> return Some(NodeType::LeafNode),
			Token::Variable(_) 	=> return Some(NodeType::LeafNode),
			Token::Function(_) 	=> return Some(NodeType::LeafNode),
			Token::Operator(op) => {
				match op {
					Operator::UnaryOperator(_) => return Some(NodeType::UnaryOperator),
					Operator::BinaryOperator(_) => return Some(NodeType::BinaryOperator),
				}
			},
			_ => return None,
		}
	}

	fn get_node(&self, nodestr: &str) -> Option<&Node<T>> {
		self.map.get(nodestr)
	}

}


