use aoc::aoc_input::get_input;
use itertools::iproduct;
use num_traits::AsPrimitive;
use std::{iter::Sum, ops::Add, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LBracket,
    RBracket,
    Comma,
    Num(u8),
    Unexpected,
    Overflow,
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

    fn peek_digit(&mut self) -> bool {
        match self.peek_ch() {
            None => false,
            Some(c) => c.is_ascii_digit(),
        }
    }

    fn next_digit<T>(&mut self) -> T
    where
        T: 'static + Copy,
        u32: AsPrimitive<T>,
    {
        self.next_ch().unwrap().to_digit(10).unwrap().as_()
    }

    fn number(&mut self) -> Token {
        let mut n: u8 = self.next_digit();

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
        let c = *self.peek_ch()?;
        let t = match c {
            '0'..='9' => self.number(),
            '[' => {
                self.next_ch();
                Token::LBracket
            }
            ']' => {
                self.next_ch();
                Token::RBracket
            }
            ',' => {
                self.next_ch();
                Token::Comma
            }
            _ => {
                self.next_ch();
                Token::Unexpected
            }
        };
        Some(t)
    }
}

struct Parser<'a> {
    tokens: std::iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            tokens: lexer.peekable(),
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), &'static str> {
        let t = self.tokens.next().ok_or("Unexpected EOF")?;
        if t != token {
            Err("Unexpected token")
        } else {
            Ok(())
        }
    }

    fn parse_pair_rparen(&mut self) -> Result<Node, &'static str> {
        let left = Box::new(self.parse_node()?);
        self.expect(Token::Comma)?;
        let right = Box::new(self.parse_node()?);
        self.expect(Token::RBracket)?;
        Ok(Node::Pair { left, right })
    }

    fn parse_node(&mut self) -> Result<Node, &'static str> {
        match self.tokens.next().ok_or("EOF node")? {
            Token::LBracket => self.parse_pair_rparen(),
            Token::Num(n) => Ok(Node::Num { value: n }),
            _ => Err("Unexpected token for node start"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CursorDir {
    L,
    R,
}

type Cursor = Vec<CursorDir>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DfsResult {
    NotFound,
    Found,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Identity,
    Num { value: u8 },
    Pair { left: Box<Node>, right: Box<Node> },
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(Lexer::new(s));
        parser.parse_node()
    }
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        if self == Node::Identity {
            rhs
        } else if rhs == Node::Identity {
            self
        } else {
            let left = Box::new(self);
            let right = Box::new(rhs);
            let mut res = Node::Pair { left, right };
            res.reduce();
            res
        }
    }
}

impl Sum<Node> for Node {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::Identity, |a, b| a + b)
    }
}

impl Node {
    fn get(&self, cursor: &Cursor) -> &Self {
        let mut cur = self;
        for d in cursor {
            match cur {
                Node::Pair { left, right } => match d {
                    CursorDir::L => cur = left.as_ref(),
                    CursorDir::R => cur = right.as_ref(),
                },
                _ => panic!("Invalid cursor"),
            }
        }
        cur
    }

    fn get_mut(&mut self, cursor: &Cursor) -> &mut Self {
        let mut cur = self;
        for d in cursor {
            match cur {
                Node::Pair { left, right } => match d {
                    CursorDir::L => cur = left.as_mut(),
                    CursorDir::R => cur = right.as_mut(),
                },
                _ => panic!("Invalid cursor"),
            }
        }
        cur
    }

    fn get_num(&self) -> Option<&u8> {
        match self {
            Node::Num { value } => Some(value),
            _ => None,
        }
    }

    fn get_num_mut(&mut self) -> Option<&mut u8> {
        match self {
            Node::Num { value } => Some(value),
            _ => None,
        }
    }

    fn dfs_inorder_recurse(
        &self,
        cursor: &mut Cursor,
        stop: impl Fn(&Node, &Cursor) -> bool + Copy,
    ) -> DfsResult {
        if stop(self, cursor) {
            return DfsResult::Found;
        }

        if let Node::Pair { left, right } = self {
            for (n, d) in [(left, CursorDir::L), (right, CursorDir::R)] {
                cursor.push(d);
                if n.dfs_inorder_recurse(cursor, stop) == DfsResult::Found {
                    return DfsResult::Found;
                }
                cursor.pop();
            }
        }

        DfsResult::NotFound
    }

    fn dfs_inorder(&self, stop: impl Fn(&Node, &Cursor) -> bool + Copy) -> Option<Cursor> {
        let mut cursor = Cursor::new();
        match self.dfs_inorder_recurse(&mut cursor, stop) {
            DfsResult::Found => Some(cursor),
            DfsResult::NotFound => None,
        }
    }

    fn find_leftmost_explode(&self) -> Option<Cursor> {
        self.dfs_inorder(|n, c| {
            if c.len() < 4 {
                return false;
            }

            match n {
                Node::Pair { left: _, right: _ } => true,
                _ => false,
            }
        })
    }

    fn find_leftmost_split(&self) -> Option<Cursor> {
        self.dfs_inorder(|n, _| match n {
            Node::Num { value } => *value >= 10,
            _ => false,
        })
    }

    fn next_left(&self, cursor: &Cursor) -> Option<Cursor> {
        let (i, _) = cursor
            .iter()
            .enumerate()
            .rfind(|(_, d)| **d == CursorDir::R)?;

        let mut res = cursor[0..i].to_owned();
        res.push(CursorDir::L);

        let mut cur = self.get(&res);
        while let Node::Pair { left: _, right } = cur {
            res.push(CursorDir::R);
            cur = right.as_ref();
        }
        Some(res)
    }

    fn next_right(&self, cursor: &Cursor) -> Option<Cursor> {
        let (i, _) = cursor
            .iter()
            .enumerate()
            .rfind(|(_, d)| **d == CursorDir::L)?;

        let mut res = cursor[0..i].to_owned();
        res.push(CursorDir::R);

        let mut cur = self.get(&res);
        while let Node::Pair { left, right: _ } = cur {
            res.push(CursorDir::L);
            cur = left.as_ref();
        }
        Some(res)
    }

    fn explode(&mut self, cursor: Cursor) {
        let cur = self.get_mut(&cursor);
        let (lnum, rnum) = match cur {
            Node::Pair { left, right } => {
                let res = (*left.get_num().unwrap(), *right.get_num().unwrap());
                *cur = Node::Num { value: 0 };
                res
            }
            _ => panic!("Invalid explosion cursor"),
        };
        drop(cur);

        if let Some(left) = self.next_left(&cursor) {
            *self.get_mut(&left).get_num_mut().unwrap() += lnum;
        }

        if let Some(right) = self.next_right(&cursor) {
            *self.get_mut(&right).get_num_mut().unwrap() += rnum;
        }
    }

    fn split(&mut self, cursor: Cursor) {
        let cur = self.get_mut(&cursor);
        let num = *cur.get_num().unwrap();
        let lnum = num / 2;
        let rnum = (num + 1) / 2;

        *cur = Node::Pair {
            left: Box::new(Node::Num { value: lnum }),
            right: Box::new(Node::Num { value: rnum }),
        }
    }

    fn try_explode(&mut self) -> bool {
        match self.find_leftmost_explode() {
            Some(cursor) => {
                self.explode(cursor);
                true
            }
            None => false,
        }
    }

    fn try_split(&mut self) -> bool {
        match self.find_leftmost_split() {
            Some(cursor) => {
                self.split(cursor);
                true
            }
            None => false,
        }
    }

    fn reduce(&mut self) {
        while self.try_explode() || self.try_split() {}
    }

    fn magnitude(&self) -> u32 {
        match self {
            Node::Identity => 0,
            Node::Num { value } => *value as u32,
            Node::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

fn main() {
    let input = get_input(2021, 18);
    let nodes: Vec<Node> = input.trim().lines().map(|s| s.parse().unwrap()).collect();

    let sum: Node = nodes.iter().cloned().sum();
    println!("Magnitude: {}", sum.magnitude());

    let max = iproduct!(0..nodes.len(), 0..nodes.len())
        .filter_map(|(i, j)| {
            if i == j {
                None
            } else {
                Some((nodes[i].clone() + nodes[j].clone()).magnitude())
            }
        })
        .max()
        .unwrap();
    println!("Maximum magnitude of sum of two numbers: {}", max);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    fn check_find_leftmost_explode(n_str: &str, expected: &[CursorDir]) {
        let n: Node = n_str.parse().unwrap();
        let c = n.find_leftmost_explode().unwrap();
        assert_eq!(c, expected);
    }

    #[test]
    fn test_find_leftmost_explode() {
        check_find_leftmost_explode("[[[[[9,8],1],2],3],4]", &[CursorDir::L; 4]);
        check_find_leftmost_explode("[7,[6,[5,[4,[3,2]]]]]", &[CursorDir::R; 4]);
        check_find_leftmost_explode(
            "[[6,[5,[4,[3,2]]]],1]",
            &[CursorDir::L, CursorDir::R, CursorDir::R, CursorDir::R],
        );
    }

    fn check_find_leftmost_split(n_str: &str, expected: &[CursorDir]) {
        let n: Node = n_str.parse().unwrap();
        let c = n.find_leftmost_split().unwrap();
        assert_eq!(c, expected);
    }

    #[test]
    fn test_find_leftmost_split() {
        check_find_leftmost_split(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            &[CursorDir::L, CursorDir::R, CursorDir::L],
        );
        check_find_leftmost_split(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            &[CursorDir::L, CursorDir::R, CursorDir::R, CursorDir::R],
        );
    }

    #[test]
    fn test_explode() {
        let expected1: Node = "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
        let expected2: Node = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap();

        let mut actual: Node = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
        assert!(actual.try_explode());
        assert_eq!(actual, expected1);

        assert!(actual.try_explode());
        assert_eq!(actual, expected2);
    }

    #[test]
    fn test_split() {
        let expected1: Node = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".parse().unwrap();
        let expected2: Node = "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".parse().unwrap();

        let mut actual: Node = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap();
        assert!(actual.try_split());
        assert_eq!(actual, expected1);

        assert!(actual.try_split());
        assert_eq!(actual, expected2);
    }
}
