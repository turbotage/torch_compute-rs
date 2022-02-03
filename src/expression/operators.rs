
use crate::expression::{
	Token,
	Context,
};

pub trait Op {
	
	fn get_token(&self) -> &str;

	fn get_precedence(&self) -> u8;

	fn get_is_left_associative(&self) -> bool;

}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperator {
	token: String,
	precedence: u8,
	is_left_associative: bool,
	allowed_left_tokens: Vec<Token>
}

impl UnaryOperator {
	pub fn new(token: String, precedence: u8, is_left_associative: bool, allowed_left_tokens: Vec<Token>) -> Self {
		Self {token, precedence, is_left_associative, allowed_left_tokens}
	}

	pub fn get_allowed_left_tokens(&self) -> &[Token] {
		&self.allowed_left_tokens
	}
}

impl Op for UnaryOperator {
    fn get_token(&self) -> &str {
        &self.token
    }

    fn get_precedence(&self) -> u8 {
        self.precedence
    }

    fn get_is_left_associative(&self) -> bool {
        self.is_left_associative
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperator {
	token: String,
	precedence: u8,
	is_left_associative: bool,
}

impl BinaryOperator {
	pub fn new(token: String, precedence: u8, is_left_associative: bool) -> Self {
		Self {token: token, precedence: precedence, is_left_associative: is_left_associative}
	}
}

impl Op for BinaryOperator {
    fn get_token(&self) -> &str {
        &self.token
    }

    fn get_precedence(&self) -> u8 {
        self.precedence
    }

    fn get_is_left_associative(&self) -> bool {
        self.is_left_associative
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	UnaryOperator(UnaryOperator),
	BinaryOperator(BinaryOperator),
}

impl Op for Operator {

	fn get_token(&self) -> &str {
		match self {
			Operator::UnaryOperator(op) => return &op.token,
			Operator::BinaryOperator(op) => return &op.token,
		}
	}

	fn get_precedence(&self) -> u8 {
		match self {
			Operator::UnaryOperator(op) => return op.precedence,
			Operator::BinaryOperator(op) => return op.precedence,
		}
	}

	fn get_is_left_associative(&self) -> bool {
		match self {
			Operator::UnaryOperator(op) => return op.is_left_associative,
			Operator::BinaryOperator(op) => return op.is_left_associative,
		}
	}

}


pub (super) fn begins_with_unary_operator<'a>(expr: &str, tokens: &Vec<Token>, context: &'a Context) -> Option<&'a UnaryOperator> {
	for uop in context.unary_operators.iter() {
		if expr.starts_with(&uop.token) {
			for allowed_left_tok in uop.get_allowed_left_tokens().iter() {
				if tokens.last().unwrap().eq(allowed_left_tok) {
					return Some(uop);
				}
			}
		}
	}
	None
}

pub (super) fn begins_with_binary_operator<'a>(expr: &str, _tokens: &Vec<Token>, context: &'a Context) -> Option<&'a BinaryOperator> {
	for bop in context.binary_operators.iter() {
		if expr.starts_with(&bop.token) {
			return Some(bop);
		}
	}
	None
}

pub enum DefaultOperetor {
	Neg,
	Pow,
	Mul,
	Div,
	Add,
	Sub,
}

pub fn default_unary_operators() -> Vec<UnaryOperator> {
	return vec![
		UnaryOperator { 
			token: String::from("-"), 
			precedence: default_precedence(DefaultOperetor::Neg), 
			is_left_associative: false,
			allowed_left_tokens: vec![Token::NoToken, Token::LeftParen],
		},
	];
}

pub fn default_binary_operators() -> Vec<BinaryOperator> {
	return vec![
		BinaryOperator { 
			token: String::from("^"),
			precedence: default_precedence(DefaultOperetor::Pow), 
			is_left_associative: false 
		},
		BinaryOperator { 
			token: String::from("*"),
			precedence: default_precedence(DefaultOperetor::Mul), 
			is_left_associative: false 
		},
		BinaryOperator { 
			token: String::from("/"),
			precedence: default_precedence(DefaultOperetor::Div), 
			is_left_associative: false 
		},
		BinaryOperator { 
			token: String::from("+"),
			precedence: default_precedence(DefaultOperetor::Add), 
			is_left_associative: false 
		},
		BinaryOperator { 
			token: String::from("-"),
			precedence: default_precedence(DefaultOperetor::Sub), 
			is_left_associative: false 
		},
	];
}

pub (super) fn default_precedence(op: DefaultOperetor) -> u8 {
	match op {
		DefaultOperetor::Neg => 10,
		DefaultOperetor::Pow => 10,
		DefaultOperetor::Mul => 5,
		DefaultOperetor::Div => 5,
		DefaultOperetor::Add => 3,
		DefaultOperetor::Sub => 3,
		//_ => panic!("Unimplemented DefaultOperator was supplied"),
	}
}
