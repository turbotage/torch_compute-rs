

use crate::expression::{
	Token,
	Context,
	lexer,
	operators::Operator,
};

/*
pub struct Shunter {
	output: Vec<Token>,
	operators: Vec<Token>,
	pub (super) last_token: Token,
}
*/

pub fn shunt(expr: &str, context: &Context) -> anyhow::Result<Vec<Token>> {
	
	let tokens = lexer::lex(expr, context);
	if tokens.is_err() {
		return Err(tokens.unwrap_err());
	}
	let tokens = tokens.unwrap();
	//println!("Lexed tokens: {:?}", tokens);

	let mut operator_stack: Vec<Token> = vec![];
	let mut output: Vec<Token> = vec![];

	for ref token in tokens {
		match token { // check this, unecessary clone
			Token::NoToken => {},
			Token::Number(_) | Token::Unity | Token::Zero => output.push(token.clone()),
			Token::Variable(_) => output.push(token.clone()),
			Token::Function(_) => operator_stack.push(token.clone()),
			Token::Operator(op) => {
				if let Err(res) = handle_operator(&mut operator_stack, &mut output, op) {
					return anyhow::private::Err(res);
				}
				operator_stack.push(token.clone());
			},
			Token::LeftParen => operator_stack.push(token.clone()),
			Token::RightParen => {
				if let Err(res) = handle_rparen(&mut operator_stack, &mut output) {
					return anyhow::private::Err(res);
				}
			},
			Token::Comma => {},
		}
	}

	if shift_until(&mut operator_stack, &mut output, &Token::LeftParen) {
		return Err(anyhow::anyhow!("missmatched parenthesis"));
	}

	assert!(operator_stack.is_empty());

	return Ok(output);
}

pub fn stringify_rpn(postfix: &Vec<Token>) -> String {
	let mut capacity = 0;
	for tok in postfix {
		capacity += tok.len();
	}
	let mut ret = String::with_capacity(capacity);
	for tok in postfix {
		ret += tok.stringify();
		ret += ",";
	}
	return ret;
}

fn handle_operator(operator_stack: &mut Vec<Token>, output: &mut Vec<Token>, operator: &Operator) -> anyhow::Result<()> {
	use crate::expression::operators::Op;

	while let Some(top) = operator_stack.last() {
		match top {
			Token::LeftParen => break,
			Token::Operator(top_operator) => {
				let p = top_operator.get_precedence();
				let q = operator.get_precedence();
				if (p > q) ||(p == q && operator.get_is_left_associative()) {
					output.push(operator_stack.pop().unwrap());
				} else {
					break;
				}
			},
			_ => return Err(anyhow::anyhow!("{:?} must not be on operator stack", operator.get_token())),
		}
	}
	return Ok(());
}

fn handle_rparen(operator_stack: &mut Vec<Token>, output: &mut Vec<Token>) -> anyhow::Result<()> {
	// Move from operator_stack to output untill we meet a (
	if !shift_until(operator_stack, output, &Token::LeftParen) {
		return Err(anyhow::anyhow!("missmatched parenthesis"));
	}

	if let Some(top) = operator_stack.last() {
		if let Token::Function(_) = top {
			output.push(top.clone());
			operator_stack.pop();
		}
	}

	return Ok(());
}

fn shift_until(operator_stack: &mut Vec<Token>, output: &mut Vec<Token>, stop: &Token) -> bool {
	while let Some(token) = operator_stack.pop() {
		if token.eq(stop) {
			return true;
		}
		output.push(token);
	}
	return false;
}


