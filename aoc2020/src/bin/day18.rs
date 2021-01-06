use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    Number(u64),
    Add,
    Multiply,
    LParen,
}

fn to_infix(s: &str, prec_map: &HashMap<Token, usize>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for tok in s.chars() {
        match tok {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { output.push(Token::Number(tok.to_digit(10).unwrap() as u64)); },
            '+' | '*' =>  {
                let tok = match tok {
                    '+' => Token::Add,
                    '*' => Token::Multiply,
                    _ => unreachable!(),
                };
                let precedence = prec_map.get(&tok).unwrap();
                
                while !operators.is_empty()
                && (prec_map.get(operators.last().unwrap()).unwrap() >= precedence)
                && (*operators.last().unwrap() != Token::LParen) {
                    let op = operators.pop().unwrap();
                    output.push(op);
                }
                operators.push(tok);
            }
            '(' => operators.push(Token::LParen),
            ')' => {
                while *operators.last().unwrap() != Token::LParen {
                    let op = operators.pop().unwrap();
                    output.push(op);
                }
                operators.pop();
            }
            ' ' => {},
            _ => unreachable!(),
        }
    }

    while !operators.is_empty() {
        let op = operators.pop().unwrap();
        output.push(op);
    }

    output
}

fn eval(operations: &Vec<Token>) -> u64 {
    let mut stack = Vec::new();

    for o in operations.iter() {
        match o {
            Token::Number(n) => { stack.push(*n); },
            Token::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Multiply => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            Token::LParen => unreachable!(),        
        }
    }
    *stack.last().unwrap()
}

fn main() {
    let file = File::open("input/day18.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let part1_precedence_map = [(Token::Add, 1), (Token::Multiply, 1), (Token::LParen, 0)].iter().cloned().collect();
    let part2_precedence_map = [(Token::Add, 2), (Token::Multiply, 1), (Token::LParen, 0)].iter().cloned().collect();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in file.lines() {
        let line = line.unwrap();

        let infix = to_infix(&line, &part1_precedence_map);
        let n = eval(&infix);
        part1 += n;

        let infix = to_infix(&line, &part2_precedence_map);
        let n = eval(&infix);
        part2 += n;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}