
use crate::expression::{
    Context,
    Token,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    token: String,
    n_inputs: u8,
}

impl Function {

    pub fn new(token: &str, n_inputs: u8) -> Self {
        Self {token: token.to_string(), n_inputs: n_inputs}
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_n_inputs(&self) -> u8 {
        self.n_inputs
    }

}

pub (super) fn begins_with_function<'a>(expr: &str, _tokens: &Vec<Token>, context: &'a Context) -> anyhow::Result<Option<&'a Function>> {
    for func in context.functions.iter() {
        if expr.starts_with(&func.token) {
            let flen = func.token.len();
            let lpar = expr[flen..].find("(");
            let rpar = expr[flen..].find(")");
            if lpar.is_some() && rpar.is_some() {
                let llen = lpar.unwrap();
                let rlen = rpar.unwrap();
                let argsubstr = &expr[flen+llen..flen+rlen];
                let commaocs = argsubstr.match_indices(',').count();
                if commaocs == (func.get_n_inputs() - 1) as usize {
                    return Ok(Some(func));
                }
                return Err(anyhow::anyhow!("Matched with function signature but the number of commas was inconsistent with number of arguments for function"));
            }
            return Err(anyhow::anyhow!("Matched with a function signature but opening and closing parentheses did not follow"));
        }
    }
    Ok(None)
}


pub fn default_functions() -> Vec<Function> {
    return vec![
        Function::new("sin", 1),
    ];
}