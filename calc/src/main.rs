enum Expr {
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Num(isize),
}

impl Expr {
    fn eval(&self) -> isize {
        match &self {
            Self::Mul(x, y) => x.eval() * y.eval(),
            Self::Add(x, y) => x.eval() + y.eval(),
            Self::Sub(x, y) => x.eval() - y.eval(),
            Self::Num(x) => *x,
        }
    }
}

fn fib(n: usize) -> Box<Expr> {
    if n >= 2 {
        Box::new(Expr::Add(fib(n-1), fib(n-2)))
    } else {
        Box::new(Expr::Num(1))
    }
}

fn main() {
    use Expr::*;
    let x = Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)));
    println!("x evals to {}", x.eval());
    println!("fib(11) evals to {}", fib(100).eval()); // wooooooooooooooooooooooooooooooooooooooooooooo
}
