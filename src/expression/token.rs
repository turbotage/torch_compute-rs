

use crate::expression::{
	functions::Function,
	operators::Operator,
	operators::UnaryOperator,
	operators::BinaryOperator,
	varnum::Variable,
	varnum::Number,
};

use self::operators::Op;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	NoToken,
	Number(Number),
	Zero,
	Unity,
	Variable(Variable),
	Function(Function),
	Operator(Operator),
	LeftParen,
	RightParen,
	Comma,
}

impl Token {

	pub fn stringify(&self) -> &str {
		match self {
			Token::NoToken => return "",
			Token::Number(num) => return num.get_token(),
			Token::Zero => return "Zero",
			Token::Unity => return "Unity",
			Token::Variable(var) => return var.get_token(),
			Token::Function(func) => return func.get_token(),
			Token::Operator(op) => return op.get_token(),
			Token::LeftParen => return "(",
			Token::RightParen => return ")",
			Token::Comma => return ",",
		}
	}

	pub fn len(&self) -> usize {
		match self {
			Token::NoToken => return 0,
			Token::Number(num) => return num.get_token().chars().count(),
			Token::Zero => return 4,
			Token::Unity => return 5,
			Token::Variable(var) => return var.get_token().chars().count(),
			Token::Function(func) => return func.get_token().chars().count(),
			Token::Operator(op) => return op.get_token().chars().count(),
			Token::LeftParen => return 1,
			Token::RightParen => return 1,
			Token::Comma => return 1,
		}
	}

}

impl Default for Token {
	fn default() -> Self {
		Self::NoToken
	}
}


pub struct Context {
	unary_operators: Vec<UnaryOperator>,
	binary_operators: Vec<BinaryOperator>,
	functions: Vec<Function>,
	variables: Vec<Variable>,
}

impl Context {

	fn new() -> Self {
		Self {..Default::default()}
	}

	pub fn add_variable(&mut self, var: Variable) {
		self.variables.push(var);
	}

}

impl Default for Context {

	fn default() -> Self {
		Context {
			unary_operators: operators::default_unary_operators(),
			binary_operators: operators::default_binary_operators(),
			functions: functions::default_functions(),
			variables: vec![],
		}
	}

}