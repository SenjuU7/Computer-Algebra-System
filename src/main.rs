use num_complex::Complex64;
use std::collections::HashMap;
use crate::Expr::Var;

type Env = HashMap<String, Complex64>;

#[derive(Clone,Debug)]
enum Expr{
    Var(String),
    NumCX(Complex64),

    Add(Vec<Expr>),
    Mul(Vec<Expr>),

    Pow(Box<Expr>, Box<Expr>),

    Neg(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
}

impl Expr {

    fn eval(&self, env: &Env) -> Complex64 {
        match self {
            Expr::NumCX(n) => *n,

            Expr::Var(name) =>
                *env.get(name).expect("variable not found"),

            Expr::Add(xs) =>
                xs.iter().map(|e| e.eval(env)).sum(),

            Expr::Mul(xs) =>
                xs.iter().map(|e| e.eval(env)).product(),

            Expr::Neg(e) =>
                -e.eval(env),

            Expr::Sin(e) =>
                e.eval(env).sin(),

            Expr::Cos(e) =>
                e.eval(env).cos(),

            Expr::Pow(a, b) =>
                a.eval(env).powc(b.eval(env)),
        }
    }
}

fn main(){
    let i = Expr::NumCX(Complex64::new(0.0, 1.0));

    // function (ac - bd)(ad+bc)i
    let expr = Expr::Add(vec![
        Expr::Mul(vec![
            Var("a".into()),
            Var("c".into())
        ]),Expr::Neg(Box::new(
            Expr::Mul(vec![
                Var("b".into()),
                Var("d".into()),
            ]))),
        Expr::Mul(vec![
            Expr::Add(vec![
                Expr::Mul(vec![Var("a".into()), Var("d".into())]),
                Expr::Mul(vec![Var("b".into()), Var("c".into())]),
            ]),
            i,
        ]),
    ]);

    let mut env = Env::new();
    env.insert("a".into(), Complex64::new(6.0, 0.0));
    env.insert("c".into(), Complex64::new(7.0, 0.0));

    env.insert("b".into(), Complex64::new(5.0, 0.0));
    env.insert("d".into(), Complex64::new(4.0, 0.0));

    let result = expr.eval(&env);
    println!("result: {:#?}", result);

}