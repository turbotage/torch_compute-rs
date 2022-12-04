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

use crate::expression::{
    varnum::Variable,
    Context,
    shunter, operators::{Operator, Op},
};

use std::time::{Duration, Instant};

use cpython::{Python, PyDict, PyList, PyString, PyResult, ToPyObject, py_capsule};

fn test_shunting() {
	
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
    
}

fn main() {
	/*
	let gil = Python::acquire_gil();
    run_expr(gil.python()).unwrap();
	*/

	
	let now = Instant::now();
	test_shunting();
	println!("Time elapsed: {}", now.elapsed().as_millis());
	

}

fn run_expr(py: Python) -> PyResult<()> {
    
	let now = Instant::now();

	let sys = py.import("sys")?;

	let mut sys_path: Vec<String> = sys.get(py, "path")?.extract(py)?;
	sys_path.insert(0, "./pyth/".into());
	sys.add(py, "path", sys_path);

	let module = py.import("expr")?;

	let pyexpr = PyString::new(py, "1.0*sin(x)^2+2*cos(x)^2+1.045e-6+I+exp(I)+tan(sin(x))-exp(tan(sin(x)))+6*tanh(exp(sin(x)))");

	let symbols: PyList = vec!["x", "y"].to_py_object(py);

	let symsdict: PyDict = module.call(py, "get_syms", (symbols,), None)?.extract(py)?;

	let exprout: String = module.call(py, "get_expr", (pyexpr,symsdict,), None, )?.extract(py)?;
	
	println!("Time elapsed: {}", now.elapsed().as_millis());
	println!("Expr: {}", exprout);


	// Second time
	let now = Instant::now();

	let pyexpr = PyString::new(py, "1.0*sin(x)^2+2*cos(x)^2+1.045e-6+I+exp(I)+tan(sin(x))-exp(tan(sin(x)))+6*tanh(exp(sin(x)))");

	let symbols: PyList = vec!["x", "y"].to_py_object(py);

	let symsdict: PyDict = module.call(py, "get_syms", (symbols,), None)?.extract(py)?;

	let exprout: String = module.call(py, "get_expr", (pyexpr,symsdict,), None, )?.extract(py)?;
	
	println!("Time elapsed: {}", now.elapsed().as_millis());
	println!("Expr: {}", exprout);
	


	Ok(())
}