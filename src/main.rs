//use tch::{kind, Tensor};

/*
fn grad_example() {
    let mut x = Tensor::from(2.0).set_requires_grad(true);
    let y = &x * &x + &x + 36;
    println!("{}", y.double_value(&[]));
    x.zero_grad();
    y.backward();
    let dy_over_dx = x.grad();
    println!("{}", dy_over_dx.double_value(&[]));
}

fn main() {
    tch::maybe_init_cuda();
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    t.print();
    let t = Tensor::randn(&[5, 4], kind::FLOAT_CPU);
    t.print();
    (&t + 1.5).print();
    (&t + 2.5).print();
    let mut t = Tensor::of_slice(&[1.1f32, 2.1, 3.1]);
    t += 42;
    t.print();
    println!("{:?} {}", t.size(), t.double_value(&[1]));
    grad_example();
    println!("Cuda available: {}", tch::Cuda::is_available());
    println!("Cudnn available: {}", tch::Cuda::cudnn_is_available());
}
*/


mod expression;

use fancy_regex::Regex;

use crate::expression::{
    varnum::Variable,
    Context,
    shunter, operators::{Operator, Op},
};

fn main() {
    
    let expr = "sin(X)*max(X,Y)+cos(sin(X)*cos(Y))+cos(Y)*cos(Y)";
    let mut context: Context = Context::default();
    context.add_variable(Variable::new("X"));
	context.add_variable(Variable::new("Y"));
    let rpn = shunter::shunt(expr, &context);
    println!("full rpn notation: {:?}", rpn);
	if let Ok(rpn) = rpn {
		let rpnstr = shunter::stringify_rpn(&rpn);
		println!("rpn notation: {}", rpnstr);

		let mut pairs: Vec<(u32, String)> = vec![];

		for i in 0..rpn.len() {
			if rpn[i].stringify().eq("Y") {
				if let Some(next) = rpn.iter().nth(i+1) {
					match next {
						expression::Token::Operator(op) => {
							match op {
								Operator::UnaryOperator(unop) => {
									pairs.push((i.try_into().unwrap(), unop.get_token().to_owned() + "Y"));
								}
								_ => {},
							}
						},
						expression::Token::Function(func) => {
							if func.get_n_inputs().eq(&1) {
								pairs.push((i.try_into().unwrap(), func.get_token().to_owned() + "Y"));
							}
						}
						_ => {},
					}
				}
			}
		}

		println!("Y Leaf pairs: {:?}", pairs);

		let mut pairs: Vec<(u32, String)> = vec![];

		for i in 0..rpn.len() {
			if rpn[i].stringify().eq("X") {
				if let Some(next) = rpn.iter().nth(i+1) {
					match next {
						expression::Token::Operator(op) => {
							match op {
								Operator::UnaryOperator(unop) => {
									pairs.push((i.try_into().unwrap(), unop.get_token().to_owned() + "X"));
								}
								_ => {},
							}
						},
						expression::Token::Function(func) => {
							if func.get_n_inputs().eq(&1) {
								pairs.push((i.try_into().unwrap(), func.get_token().to_owned() + "X"));
							}
						}
						_ => {},
					}
				}
			}
		}

		println!("Y Leaf pairs: {:?}", pairs);


	}
    
	


    /*
    let reg = fancy_regex::Regex::new(
        r"^(?=[iI.\d+-])([+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?(?![iI.\d]))?([+-]?(?:(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?)?[iI])?");
    let reg = reg.unwrap();
    
    let expr = "-3.0e1+2e-5i)*2";
    let result = reg.find(expr);
    assert!(result.is_ok());
    let match_option = result.unwrap();
    if match_option.is_some() {
        let m = match_option.unwrap();
        println!("match: {}", m.as_str());
    }
    */

}