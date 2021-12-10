use aoc::aoc_input::get_input;

fn expected_opener_for(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Invalid parameter"),
    }
}

fn expected_closer_for(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid parameter"),
    }
}

fn illegal_character_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid parameter"),
    }
}

fn completion_character_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid parameter"),
    }
}

#[derive(Debug, Clone)]
enum LineResult {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn parse_line(line: &str) -> Result<LineResult, &'static str> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let popped = stack.pop().ok_or("Truncated line")?;
                let expected = expected_opener_for(c);
                match popped == expected {
                    true => continue,
                    false => return Ok(LineResult::Corrupted(c)),
                }
            }
            _ => return Err("Invalid character"),
        }
    }
    Ok(LineResult::Incomplete(stack))
}

fn line_completion_score(unclosed: &Vec<char>) -> usize {
    let mut score = 0usize;
    for c in unclosed.iter().rev() {
        score = 5 * score + completion_character_score(expected_closer_for(*c));
    }
    score
}

fn main() {
    let input = get_input(2021, 10);

    let mut syntax_error_score = 0usize;
    let mut incomplete_lines = Vec::new();
    for line in input.lines() {
        match parse_line(line).unwrap() {
            LineResult::Corrupted(c) => syntax_error_score += illegal_character_score(c),
            LineResult::Incomplete(v) => incomplete_lines.push(v),
        }
    }

    println!("Total syntax error score: {}", syntax_error_score);

    let mut completion_scores: Vec<_> =
        incomplete_lines.iter().map(line_completion_score).collect();
    completion_scores.sort();
    let middle = completion_scores[completion_scores.len() / 2];
    println!("Middle completion score: {}", middle);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
}
