use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Literal(i32),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    LParen,
    RParen,
    End,
}

struct Parser {
    tokens: VecDeque<Token>,
    current_token: Option<Token>,
}

impl Parser {
    fn new(program: &str) -> Self {
        let token_pat = Regex::new(r"\s*(?:(\d+)|(.))").unwrap();
        let mut tokens = VecDeque::new();

        for caps in token_pat.captures_iter(program) {
            if let Some(number) = caps.get(1) {
                tokens.push_back(Token::Literal(number.as_str().parse().unwrap()));
            } else if let Some(operator) = caps.get(2) {
                let op = match operator.as_str() {
                    "+" => Token::Add,
                    "-" => Token::Sub,
                    "*" => Token::Mul,
                    "/" => Token::Div,
                    "^" => Token::Pow,
                    "(" => Token::LParen,
                    ")" => Token::RParen,
                    _ => panic!("unknown operator: {}", operator.as_str()),
                };
                tokens.push_back(op);
            }
        }
        tokens.push_back(Token::End);

        Parser {
            tokens,
            current_token: None,
        }
    }

    fn next(&mut self) {
        self.current_token = self.tokens.pop_front();
    }

    fn match_token(&mut self, expected: Token) {
        if self.current_token != Some(expected.clone()) {
            panic!("Expected {:?}", expected);
        }
        self.next();
    }

    fn parse(&mut self) -> i32 {
        self.next();
        self.expression(0)
    }

    fn expression(&mut self, rbp: i32) -> i32 {
        let mut left = self.nud();
        while rbp < self.lbp() {
            left = self.led(left);
        }
        left
    }

    fn nud(&mut self) -> i32 {
        match self.current_token.take() {
            Some(Token::Literal(value)) => {
                self.next();
                value
            }
            Some(Token::Add) => {
                self.next();
                -self.expression(100)
            }
            Some(Token::LParen) => {
                self.next();
                let expr = self.expression(0);
                self.match_token(Token::RParen);
                expr
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn led(&mut self, left: i32) -> i32 {
        match self.current_token.take() {
            Some(Token::Add) => {
                self.next();
                left + self.expression(10)
            }
            Some(Token::Sub) => {
                self.next();
                left - self.expression(10)
            }
            Some(Token::Mul) => {
                self.next();
                left * self.expression(20)
            }
            Some(Token::Div) => {
                self.next();
                left / self.expression(20)
            }
            Some(Token::Pow) => {
                self.next();
                left.pow(self.expression(30 - 1) as u32)
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn lbp(&self) -> i32 {
        match self.current_token {
            Some(Token::Add) | Some(Token::Sub) => 10,
            Some(Token::Mul) | Some(Token::Div) => 20,
            Some(Token::Pow) => 30,
            _ => 0,
        }
    }
}

fn main() {
    // 计算方法执行时间
    let start = std::time::Instant::now();
    let mut parser = Parser::new("3 * (2 + (3 - 4)) ^ 4");
    let result = parser.parse();
    println!("******************* {}", result);
    println!("******************* {:?}", start.elapsed());
}
