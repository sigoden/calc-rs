#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Num(i64),
    LParen,
    RParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Eof,
}

pub fn lex(line: &str) -> Vec<Tok> {
    let mut toks = Vec::new();
    let mut iter = line.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '(' => toks.push(Tok::LParen),
            ')' => toks.push(Tok::RParen),
            '+' => toks.push(Tok::Plus),
            '-' => toks.push(Tok::Minus),
            '*' => toks.push(Tok::Multiply),
            '/' => toks.push(Tok::Divide),
            '0'...'9' => {
                let mut num = String::new();
                num.push(c);
                while let Some(&c) = iter.peek() {
                    if c.is_digit(10) {
                        num.push(c);
                        iter.next();
                    }
                    break;
                }
                toks.push(Tok::Num(num.parse::<i64>().unwrap()));
            }
            _ => {}
        };
    }
    toks.push(Tok::Eof);

    toks
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lex() {
        assert_eq!(
            lex("24 / (4 - 4)"),
            Ok(vec![
                Tok::Num(24),
                Tok::Divide,
                Tok::LParen,
                Tok::Num(4),
                Tok::Minus,
                Tok::Num(4),
                Tok::RParen,
                Tok::Eof
            ])
        )
    }
}
