use crate::parser::Expr;

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(i) => *i,
        Expr::Plus(i, j) => eval(i) + eval(j),
        Expr::Minus(i, j) => eval(i) - eval(j),
        Expr::Multiply(i, j) => eval(i) * eval(j),
        Expr::Divide(i, j) => eval(i) / eval(j),
        Expr::Negate(i) => eval(i) * -1,
    }
}
