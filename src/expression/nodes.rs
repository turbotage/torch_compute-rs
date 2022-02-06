
use std::rc::{Rc, Weak};

use tch::nn::Variables;

use super::{Token, varnum, functions, operators};

trait StrMake<T> {
	fn from_str(str: &str) -> T;
}

trait ZeroMake<T> {
	fn from_zero() -> T;
}

trait UnityMake<T> {
	fn from_unity() -> T;
}

struct LeafNode<T> {
	parent: 	Weak<Node<T>>,
	token:		Token,
	value: 		Rc<T>,
	unity: 		bool,
	zero:		bool,
	children: 	Vec<Rc<Node<T>>>,
}

impl<T> LeafNode<T> where
	T: StrMake<T> + ZeroMake<T> + UnityMake<T>
{

	fn from_number(num: &varnum::Number) -> Self {
		Self {
			parent: Weak::new(),
			token: Token::Number(*num),
			value: {
				Rc::new(T::from_str(num.get_token()))
			},
			unity: false,
			zero: false,
			children: vec![],
		}
	}
	
	fn from_variable(var: &varnum::Variable, expr: &Expression<T>) -> Self {
		Self {
			parent: Weak::new(),
			token: Token::Variable(*var),
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

struct UnaryOperatorNode<T> {
	parent: 	Rc<Node<T>>,
	child: 		Rc<Node<T>>,
}

struct BinaryOperatorNode<T> {
	parent: 	Rc<Node<T>>,
	lhchild: 	Rc<Node<T>>,
	rhchild: 	
	Rc<Node<T>>,
}

enum OperatorNode<T> {
	UnaryOperator(UnaryOperatorNode<T>),
	BinaryOperator(BinaryOperatorNode<T>),
}

struct FunctionNode<T> {
	parent: Rc<Node<T>>,
	children: Vec<Rc<Node<T>>>,
}

enum Node<T> {
	LeafNode(LeafNode<T>),
	Operator(OperatorNode<T>),
	Function(FunctionNode<T>),
}

struct Expression<T> {
	variables: Vec<(String, Rc<T>)>,
	nodes: Vec<Node<T>>,
	leafs: Vec<Node<T>>,
}

pub fn tok_to_node<T>(token: &Token, expr: &Expression<T>, 
	func_factory: fn(&functions::Function) -> anyhow::Result<Node<T>>,
	op_factory: fn(&operators::Operator) -> anyhow::Result<Node<T>>,
) -> anyhow::Result<Node<T>> 
	where T: StrMake<T> + ZeroMake<T> + UnityMake<T>
{
	match token {
		Token::Number(num) => {
			return Ok(Node::LeafNode(LeafNode::<T>::from_number(num)));
		},
		Token::Variable(var) => {
			return Ok(Node::LeafNode(LeafNode::<T>::from_variable(var, expr)))
		},
		Token::Zero => 
			return Ok(Node::LeafNode(LeafNode::<T>::from_zero())),
		Token::Unity =>
			return Ok(Node::LeafNode(LeafNode::<T>::from_unity())),
		Token::Function(func) => 
			return func_factory(func),
		Token::Operator(op) =>
			return op_factory(op),
		_ => return Err(anyhow::anyhow!("Found token that can't 
			be converted to node, token: {}", token.stringify())),
	}
}

impl<T> Expression<T> {

	fn new(variables: &Vec<(String, Rc<T>)>) -> Self {
		
	}

	fn init_expr(tokens: &Vec<Token>,
		func_factory: fn(&functions::Function) -> anyhow::Result<Node<T>>,
		op_factory: fn(&operators::Operator) -> anyhow::Result<Node<T>>) -> Self 
		where T: StrMake<T> + ZeroMake<T> + UnityMake<T>
		{
		let nodes: Vec<Node<T>> = Vec::with_capacity(tokens.len());
		for tok in tokens {
			let tok = tok_to_node(tok, expr, func_factory, op_factory);
			 if let Ok(node) =  {
				 nodes.push(node);
			 }
			 else {
				panic!("Invalid token")
			 }
		}
	}

	fn get_variable(&self, name: &str) -> Option<Rc<T>> {
		for var in self.variables {
			if var.0.eq(name) {
				return Some(var.1);
			}
		}
		None
	}

}