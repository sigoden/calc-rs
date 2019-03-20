use crate::lex::Tok;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Num(i64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(i) => write!(f, "{}", i),
            Expr::Plus(ref i, ref j) => write!(f, "{} + {}", i, j),
            Expr::Minus(ref i, ref j) => write!(f, "{} - {}", i, j),
            Expr::Multiply(ref i, ref j) => write!(f, "{} * {}", i, j),
            Expr::Divide(ref i, ref j) => write!(f, "{} / {}", i, j),
            Expr::Negate(ref i) => write!(f, "-{}", i),
        }
    }
}

pub struct Parser {
    toks: Vec<Tok>,
    pos: usize,
}

impl Parser {
    pub fn new(toks: Vec<Tok>) -> Self {
        Parser { toks: toks, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        if self.toks.len() == 0 {
            return Err("no tokens.");
        }
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, &'static str> {
        let lhs = self.parse_primary()?;
        self.parse_binary_ops(0, lhs)
    }

    fn parse_primary(&mut self) -> Result<Expr, &'static str> {
        let tok = self.get_token(self.pos)?;
        match tok {
            Tok::Minus => self.parse_negate(),
            Tok::Num(_) => self.parse_num(),
            Tok::LParen => self.parse_paren(),
            _ => Err("unexpect tok."),
        }
    }

    fn parse_negate(&mut self) -> Result<Expr, &'static str> {
        self.eat_token();

        let tok = self.get_token(self.pos)?;
        match tok {
            Tok::Num(i) => {
                self.eat_token();
                Ok(Expr::Negate(Box::new(Expr::Num(i))))
            }
            _ => Err("unexpect negate expr."),
        }
    }

    fn parse_num(&mut self) -> Result<Expr, &'static str> {
        let tok = self.get_token(self.pos)?;
        match tok {
            Tok::Num(i) => {
                self.eat_token();
                Ok(Expr::Num(i))
            }
            _ => Err("unexpect num expr."),
        }
    }

    fn parse_paren(&mut self) -> Result<Expr, &'static str> {
        self.eat_token();

        let expr = self.parse_expr()?;
        let tok = self.get_token(self.pos)?;
        match tok {
            Tok::RParen => {
                self.eat_token();
                Ok(expr)
            }
            _ => Err("mismatch parenthesis."),
        }
    }

    fn parse_binary_ops(
        &mut self,
        expr_pred: i8,
        mut left_expr: Expr,
    ) -> Result<Expr, &'static str> {
        loop {
            let tok = self.get_token(self.pos)?;
            let tok_pred = Self::get_op_pred(&tok);
            if tok_pred < expr_pred {
                return Ok(left_expr);
            }
            self.eat_token();
            let mut right_expr = self.parse_primary()?;
            let next_op = self.get_token(self.pos)?;
            let next_pred = Self::get_op_pred(&next_op);

            if tok_pred < next_pred {
                right_expr = self.parse_binary_ops(expr_pred + 1, right_expr)?;
            }

            left_expr = Self::binary_op_expr(&tok, left_expr, right_expr);
        }
    }

    fn binary_op_expr(op: &Tok, lhs: Expr, rhs: Expr) -> Expr {
        match *op {
            Tok::Plus => Expr::Plus(Box::new(lhs), Box::new(rhs)),
            Tok::Minus => Expr::Minus(Box::new(lhs), Box::new(rhs)),
            Tok::Multiply => Expr::Multiply(Box::new(lhs), Box::new(rhs)),
            Tok::Divide => Expr::Divide(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        }
    }

    fn get_token(&self, pos: usize) -> Result<Tok, &'static str> {
        if pos < self.toks.len() {
            Ok(self.toks[pos].clone())
        } else {
            Err("no more token.")
        }
    }

    fn eat_token(&mut self) {
        self.pos += 1;
    }

    fn get_op_pred(tok: &Tok) -> i8 {
        match *tok {
            Tok::Num(_) => -1,
            Tok::Plus => 10,
            Tok::Minus => 10,
            Tok::Multiply => 20,
            Tok::Divide => 20,
            Tok::LParen => -1,
            Tok::RParen => -1,
            Tok::Eof => -1,
        }
    }
}
