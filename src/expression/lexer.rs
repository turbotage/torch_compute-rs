

use crate::expression::{
	Token,
	Context,
	operators,
	functions,
	varnum,
};

	/*
	pub (super) struct Lexer {
		pub (super) tokens: Vec<Token>,
	}
	*/


pub fn lex<'a>(expr: &str, context: &Context) -> anyhow::Result<Vec<Token>> {
	let mut tokens: Vec<Token> = vec![Token::NoToken];

	let mut reststr: &str = &expr;
	loop {
		let (next_token, retstr) = lex_token(reststr, &tokens, context);
		reststr = retstr;
		//println!("reststr: {}", reststr);
		//println!("reststr len: {}", reststr.len());

		if next_token.eq(&Token::NoToken) {
			return Err(anyhow::anyhow!("lex_token() signaled bad expression"));
		}

		tokens.push(next_token);

		if reststr.len() == 0 {
			return Ok(tokens);
		}
	}
}

fn lex_token<'a>(expr: &'a str, tokens: &Vec<Token>, context: &Context) -> (Token, &'a str) {

	use crate::expression::operators::Op;
	use crate::expression::operators::Operator;

	if let Some(c) = expr.chars().nth(0) {
		match c {
			'(' => return (Token::LeftParen, &expr[1..]),
			')' => return (Token::RightParen, &expr[1..]),
			',' => return (Token::Comma, &expr[1..]),
			_ => {},
		}
	} 

	// Unary operator
	let uop = operators::begins_with_unary_operator(expr, tokens, context);
	if uop.is_some() {
		let uop = uop.unwrap();
		return (Token::Operator(Operator::UnaryOperator(uop.clone())), &expr[uop.get_token().len()..])
	}

	// Binary operator
	let bop = operators::begins_with_binary_operator(expr, tokens, context);
	if bop.is_some() {
		let bop = bop.unwrap();
		return (Token::Operator(Operator::BinaryOperator(bop.clone())), &expr[bop.get_token().len()..]);
	}

	// Function
	let func = functions::begins_with_function(expr, tokens, context);
	if func.is_ok() {
		let func = func.unwrap();
		if func.is_some() {
			let func = func.unwrap();
			return (Token::Function(func.clone()), &expr[func.get_token().len()..]);
		}
	}

	// Variable
	let var = varnum::begins_with_variable(expr, tokens, context);
	if var.is_some() {
		let var = var.unwrap();
		return (Token::Variable(var.clone()), &expr[var.get_token().len()..]);
	}

	// Number
	let num = varnum::begins_with_number(expr, tokens, context);
	if num.is_some() {
		let num = num.unwrap();
		return (Token::Number(num.clone()), &expr[num.get_token().len()..]);
	}
	

	(Token::default(), expr)
}
