use aoc2020::aoc_input::get_input;

#[derive(Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Asterisk,
    Num(u64),
    Overflow,
    Unexpected,
}

struct Lexer<'a> {
    stream: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer {
            stream: input.chars().peekable(),
        }
    }

    fn peek_ch(&mut self) -> Option<&char> {
        self.stream.peek()
    }

    fn next_ch(&mut self) -> Option<char> {
        self.stream.next()
    }

    fn skip_spaces(&mut self) {
        while let Some(&c) = self.peek_ch() {
            if c.is_whitespace() {
                self.next_ch();
            } else {
                break;
            }
        }
    }

    fn peek_digit(&mut self) -> bool {
        match self.peek_ch() {
            None => false,
            Some(c) => c.is_ascii_digit(),
        }
    }

    fn next_digit(&mut self) -> u64 {
        self.next_ch().unwrap().to_digit(10).unwrap() as u64
    }

    fn number(&mut self) -> Token {
        let mut n = self.next_digit();

        while self.peek_digit() {
            let d = self.next_digit();
            n = match n.checked_mul(10) {
                None => return Token::Overflow,
                Some(k) => k,
            };
            n = match n.checked_add(d) {
                None => return Token::Overflow,
                Some(k) => k,
            };
        }
        Token::Num(n)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_spaces();

        let t = match *self.peek_ch()? {
            '0'..='9' => self.number(),
            '(' => {
                self.next_ch();
                Token::LParen
            }
            ')' => {
                self.next_ch();
                Token::RParen
            },
            '+' => {
                self.next_ch();
                Token::Plus
            },
            '*' => {
                self.next_ch();
                Token::Asterisk
            },
            _ => {
                self.next_ch();
                Token::Unexpected
            },
        };
        Some(t)
    }
}

#[derive(Debug, Clone)]
enum AstNode {
    Add(Box<AstNode>, Box<AstNode>),
    Multiply(Box<AstNode>, Box<AstNode>),
    Number(u64),
}

struct Parser<'a> {
    tokens: std::iter::Peekable<Lexer<'a>>
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { tokens: lexer.peekable() }
    }

    fn parse_expr_rparen(&mut self) -> AstNode {
        let expr = self.parse_expr();
        match self.tokens.next().unwrap() {
            Token::RParen => (),
            _ => panic!("Syntax error"),
        };
        expr
    }

    fn parse_term(&mut self) -> AstNode {
        match self.tokens.next().unwrap() {
            Token::LParen => self.parse_expr_rparen(),
            Token::Num(n) => AstNode::Number(n),
            Token::Unexpected | Token::Overflow => panic!("Tokenization failed"),
            _ => panic!("Syntax error"),
        }
    }

    fn peek_operator(&mut self) -> bool {
        match self.tokens.peek() {
            Some(&Token::Plus) | Some(&Token::Asterisk) => true,
            _ => false,
        }
    }

    fn parse_expr(&mut self) -> AstNode {
        let mut node = self.parse_term();
        while self.peek_operator() {
            let operator = self.tokens.next().unwrap();
            let rhs = Box::new(self.parse_term());
            let lhs = Box::new(node);
            node = match operator {
                Token::Plus => AstNode::Add(lhs, rhs),
                Token::Asterisk => AstNode::Multiply(lhs, rhs),
                _ => unreachable!(),
            };
        }

        node
    }
}

fn eval(node: &AstNode) -> u64 {
    match node {
        AstNode::Number(n) => *n,
        AstNode::Add(lhs, rhs) => eval(lhs.as_ref()) + eval(rhs.as_ref()),
        AstNode::Multiply(lhs, rhs) => eval(lhs.as_ref()) * eval(rhs.as_ref()),
    }
}

fn main() {
    let input = get_input(18);
    let mut total = 0u64;
    for line in input.lines() {
        let lexer = Lexer::new(line);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_expr();
        total += eval(&expr);
    }
    dbg!(total);
}
