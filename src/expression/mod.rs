
pub mod shunter;
pub mod functions;
pub mod operators;
pub mod varnum;
mod nodes;
mod lexer;

use crate::expression::{
	functions::Function,
	operators::Operator,
	operators::UnaryOperator,
	operators::BinaryOperator,
	varnum::Variable,
	varnum::Number,
};

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

impl Default for Token {
	fn default() -> Self {
		Self::NoToken
	}
}


pub struct Context {
	pub unary_operators: Vec<UnaryOperator>,
	pub binary_operators: Vec<BinaryOperator>,
	pub functions: Vec<Function>,
	pub variables: Vec<Variable>,
}

impl Context {

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




