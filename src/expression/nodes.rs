
use std::{rc::{Rc, Weak}};

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


trait NodeId {
	fn get_token(&self) -> &Token;

	fn get_name(&self) -> &str;

	fn get_full_name(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct LeafNode<T> {
	parent: 	Weak<Node<T>>,
	token:		Token,
	value: 		Rc<T>,
	unity: 		bool,
	zero:		bool,
	children: 	Vec<Rc<Node<T>>>,
}

impl<T> LeafNode<T> where T: LeafMake<T>
{

	fn from_number(num: &varnum::Number) -> Self {
		Self {
			parent: Weak::new(),
			token: Token::Number(num.clone()),
			value: {
				Rc::new(T::from_str(num.get_token()))
			},
			unity: false,
			zero: false,
			children: vec![],
		}
	}
	
	fn from_variable(var: &varnum::Variable, expr: &Expression<T>) -> Self where T: ExprType<T> {
		Self {
			parent: Weak::new(),
			token: Token::Variable(var.clone()),
			value: match expr.get_variable(var.get_token()) {
				Some(var) => var,
				None => panic!("Expression was 
					not supplied with variable matching token name {}", var.get_token()),
			},
			unity: false,
			zero: false,
			children: vec![],
				
		}
	}

	fn from_zero() -> Self {
		Self {
			parent: Weak::new(),
			token: Token::Zero,
			value: Rc::new(T::from_zero()),
			unity: false,
			zero: true,
			children: vec![],
				
		}
	}

	fn from_unity() -> Self {
		Self {
			parent: Weak::new(),
			token: Token::Unity,
			value: Rc::new(T::from_unity()),
			unity: true,
			zero: false,
			children: vec![],
				
		}
	}
}

impl<T> NodeId for LeafNode<T> {
	fn get_token(&self) -> &Token {
		&self.token
	}

	fn get_name(&self) -> &str {
		self.token.stringify()
	}

	fn get_full_name(&self) -> String {
		self.get_name().to_string()
	}
}

// Required arithmatic traits
use std::ops::{Add, Sub, Mul, Div, Neg};
pub trait Pow<Rhs = Self> {
	type Output;
	fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait ExprType<T>: Add<T> + Sub<T> + Mul<T> + Div<T> + Pow<T> + Neg
	+ LeafMake<T> + Debug {}




#[derive(Debug, Clone)]
pub struct UnaryOperatorNode<T> {
	parent: 	Weak<Node<T>>,
	token: 		Token,
	child: 		Rc<Node<T>>,
}

impl<T> NodeId for UnaryOperatorNode<T> {
	fn get_token(&self) -> &Token {
		&self.token
	}

	fn get_name(&self) -> &str {
		self.token.stringify()
	}

	fn get_full_name(&self) -> String {
		self.child.get_full_name() + "," + self.get_name()
	}
}

#[derive(Debug, Clone)]
pub struct BinaryOperatorNode<T> {
	parent: 	Weak<Node<T>>,
	token:		Token,
	lhchild: 	Rc<Node<T>>,
	rhchild: 	Rc<Node<T>>,
}

impl<T> NodeId for BinaryOperatorNode<T> {
	fn get_token(&self) -> &Token {
		&self.token
	}

	fn get_name(&self) -> &str {
		self.token.stringify()
	}

	fn get_full_name(&self) -> String {
		self.lhchild.get_full_name() + "," + &self.rhchild.get_full_name() + "," + self.get_name()
	}
}


#[derive(Debug, Clone)]
pub enum OperatorNode<T>{
	UnaryOperator(UnaryOperatorNode<T>),
	BinaryOperator(BinaryOperatorNode<T>),
}

impl<T> NodeId for OperatorNode<T> {
	fn get_token(&self) -> &Token {
		match self {
			OperatorNode::UnaryOperator(unop) => return unop.get_token(),
			OperatorNode::BinaryOperator(biop) => return biop.get_token(),
		}
	}

	fn get_name(&self) -> &str {
		match self {
			OperatorNode::UnaryOperator(unop) => return unop.get_name(),
			OperatorNode::BinaryOperator(biop) => return biop.get_name(),
		}
	}

	fn get_full_name(&self) -> String {
		match self {
			OperatorNode::UnaryOperator(unop) => return unop.get_full_name(),
			OperatorNode::BinaryOperator(biop) => return biop.get_full_name(),
		}
	}
}



#[derive(Debug, Clone)]
pub struct FunctionNode<T> {
	parent: Weak<Node<T>>,
	token: Token,
	children: Vec<Rc<Node<T>>>,
}

impl<T> NodeId for FunctionNode<T> {
	fn get_token(&self) -> &Token {
		&self.token
	}

	fn get_name(&self) -> &str {
		self.token.stringify()
	}


	fn get_full_name(&self) -> String {
		let mut full_name = String::from("");

		for child in &self.children {
			full_name += &(child.get_full_name() + ",");
		}
		full_name += self.get_name();

		return full_name;
	}
}


#[derive(Debug, Clone)]
pub enum Node<T> {
	LeafNode(LeafNode<T>),
	Operator(OperatorNode<T>),
	Function(FunctionNode<T>),
}

impl<T> NodeId for Node<T> {
	fn get_token(&self) -> &Token {
		match self {
			Node::LeafNode(leaf) => {
				return leaf.get_token();
			},
			Node::Operator(op) => {
				return op.get_token();
			}
			Node::Function(func) => {
				return func.get_token();
			}
		}
	}

	fn get_name(&self) -> &str {
		match self {
			Node::LeafNode(leaf) => {
				return leaf.get_name();
			},
			Node::Operator(op) => {
				return op.get_name();
			}
			Node::Function(func) => {
				return func.get_name();
			}
		}	
	}

	fn get_full_name(&self) -> String {
		match self {
			Node::LeafNode(leaf) => {
				return leaf.get_full_name();
			},
			Node::Operator(op) => {
				return op.get_full_name();
			}
			Node::Function(func) => {
				return func.get_full_name();
			}
		}
	}
}


pub struct FuncFactory<T> where T: ExprType<T> {
	eval_mapper: fn(&functions::Function) -> Option<fn(&Vec<Rc<Node<T>>>) -> T>,
	diff_mapper: fn(&functions::Function) -> Option<fn(&Vec<Rc<Node<T>>>) -> Node<T>>,
}

pub struct UnaryOpFactory<T> where T: ExprType<T> {
	eval_mapper: fn(&operators::UnaryOperator) -> Option<fn(&Rc<Node<T>>) -> T>,
	diff_mapper: fn(&operators::UnaryOperator) -> Option<fn(&Rc<Node<T>>) -> Node<T>>,
}

pub struct BinaryOpFactory<T> where T: ExprType<T> {
	eval_mapper: fn(&operators::BinaryOperator) -> Option<fn(&Rc<Node<T>>, &Rc<Node<T>>) -> T>,
	diff_mapper: fn(&operators::BinaryOperator) -> Option<fn(&Rc<Node<T>>, &Rc<Node<T>>) -> Node<T>>,
}

pub struct Expression<T> where T: ExprType<T> {
	variables: Vec<(String, Rc<T>)>,
	func_factory: FuncFactory<T>,
	unop_factory: UnaryOpFactory<T>,
	biop_factory: BinaryOpFactory<T>,
	nodes: Vec<Node<T>>,
	leafs: Vec<Node<T>>,
}

impl<T> Expression<T> where T: ExprType<T> {

	pub fn from_tokens(tokens: &Vec<Token>,
		variables: &Vec<(String, Rc<T>)>,
		func_factory: FuncFactory<T>,
		unop_factory: UnaryOpFactory<T>,
		biop_factory: BinaryOpFactory<T>) -> anyhow::Result<Self> 
	{
		// Create Expression object
		let expr = Self {
			variables: variables.clone(),
			func_factory: func_factory,
			unop_factory: unop_factory,
			biop_factory: biop_factory,
			nodes: vec![],
			leafs: vec![],
		};

		// Create vector of nodes
		let mut nodes: Vec<Node<T>> = Vec::with_capacity(tokens.len());
		for tok in tokens {
			let node = expr.tok_to_node(tok);
			 if let Err(node) = node {
				return Err(node);
			 }
			 nodes.push(node.unwrap());
		}

		// Identify Leafs

		// Simplyfy
		expr.simplify();


		return Ok(expr);

	}

	fn tok_to_node(&self, token: &Token) -> anyhow::Result<Node<T>> 
		where T: ExprType<T>
	{
		match token {
			Token::Number(num) => {
				return Ok(Node::LeafNode(LeafNode::<T>::from_number(num)));
			},
			Token::Variable(var) => {
				return Ok(Node::LeafNode(LeafNode::<T>::from_variable(var, self)))
			},
			Token::Zero => 
				return Ok(Node::LeafNode(LeafNode::<T>::from_zero())),
			Token::Unity =>
				return Ok(Node::LeafNode(LeafNode::<T>::from_unity())),
			Token::Function(func) => 
				return self.func_to_node(token, func),
			Token::Operator(op) => 
				return self.op_to_node(token, op),
			_ => return Err(anyhow::anyhow!("Found non convertible token: {}", token.stringify())),
		}
	}

	fn func_to_node(&self, token: &Token, func: &functions::Function) -> anyhow::Result<Node<T>> {
		// Extended functions
		if let Some(eval_func) = (self.func_factory.eval_mapper)(func) {
			if let Some(diff_func) = (self.func_factory.diff_mapper)(func) {
				return Ok(Node::Function(FunctionNode::<T> {
					parent: Weak::new(),
					token: token.clone(),
					children: Vec::with_capacity(func.get_n_inputs().into()),
				}));
			}
			return Err(anyhow::anyhow!("diff_mapper for func_factory hadn't implemented the function token: {}", token.stringify()));
		}
		return Err(anyhow::anyhow!("eval_mapper for func_factory hadn't implemented the function token: {}", token.stringify()));
	}

	fn op_to_node(&self, token: &Token, op: &operators::Operator)-> anyhow::Result<Node<T>> {
		match op {
			Operator::UnaryOperator(unop) => {
				// Extended unary operators
				if let Some(eval_func) = (self.unop_factory.eval_mapper)(unop) {
					if let Some(diff_func) = (self.unop_factory.diff_mapper)(unop) {
						return Ok(Node::Function(UnaryOperatorNode::<T> {
							parent: Weak::new(),
							token: token.clone(),
							child: 
						}));
					}
					return Err(anyhow::anyhow!("diff_mapper for func_factory hadn't implemented the function token: {}", token.stringify()));
				}
				return Err(anyhow::anyhow!("eval_mapper for func_factory hadn't implemented the function token: {}", token.stringify()));
			}
			Operator::BinaryOperator(biop) => {
				// Extended unary operators
				if let Some(eval_func) = (self.unop_factory.eval_mapper)(unop) {
					if let Some(diff_func) = (self.unop_factory.diff_mapper)(unop) {
						return Ok(Node::Function(UnaryOperatorNode::<T> {
							parent: Weak::new(),
							token: token.clone(),
							child: Rc::
						}));
					}
					return Err(anyhow::anyhow!("diff_mapper for func_factory hadn't implemented the function token: {}", token.stringify()));
				}
				return Err(anyhow::anyhow!("eval_mapper for func_factory hadn't implemented the function token: {}", token.stringify()));
			}
		}
	}

	
	fn simplify(&self) {
		
	}
	

	fn get_variable(&self, name: &str) -> Option<Rc<T>> {
		for var in &self.variables {
			if var.0.eq(name) {
				return Some(var.1.clone());
			}
		}
		return None;
	}

}