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
    shunter,
};

fn main() {
    
    let expr = "sin(3*X)*2";
    let mut context: Context = Context::default();
    context.add_variable(Variable::new("X"));
    let rpn = shunter::shunt(expr, &context);
    println!("rpn notation: {:?}", rpn);
    

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