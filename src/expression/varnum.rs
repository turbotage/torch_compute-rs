
use crate::expression::{
    Context,
    Token,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    token: String,
}

impl Variable {

    pub fn new(token: &str) -> Self {
        Self {token: token.to_string()}
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    token: String,
}

impl Number {

    pub fn new(token: &str) -> Self {
        Self {token: token.to_string()}
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

}

pub (super) fn begins_with_variable<'a>(expr: &str, _tokens: &Vec<Token>, context: &'a Context) -> Option<&'a Variable> {
    for var in context.variables.iter() {
        if expr.starts_with(&var.token) {
            return Some(var);
        }
    }
    return None;
}


extern crate lazy_static;

lazy_static::lazy_static! {
	pub (super) static ref GLOBAL_REGEX: fancy_regex::Regex = {
		let reg = fancy_regex::Regex::new(
            r"^(?=[iI.\d+-])([+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?(?![iI.\d]))?([+-]?(?:(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?)?[iI])?");
		return reg.unwrap()
	};
}

pub (super) fn begins_with_number(expr: &str, _tokens: &Vec<Token>, _context: &Context) -> Option<Number> {
    let result = GLOBAL_REGEX.find(expr);
    assert!(result.is_ok());
    let match_option = result.unwrap();
    if match_option.is_some() {
        let m = match_option.unwrap();
        //println!("match: {}", m.as_str());
        return Some(Number{token: m.as_str().to_string() });
    }
    return None;
}