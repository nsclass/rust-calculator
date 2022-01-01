use crate::TokenType::{CloseParam, OpenParam};

#[derive(PartialEq, Debug)]
enum TokenType {
    DIGITS,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    OpenParam,
    CloseParam,
}

#[derive(Debug)]
struct Token {
    token: String,
    token_type: TokenType,
}

impl Token {
    fn new(token: String, token_type: TokenType) -> Self {
       Self {
           token,
           token_type
       }
    }
    fn is_operand(&self) -> bool {
        self.token_type == TokenType::DIGITS
    }

    fn is_operator(&self) -> bool {
       match self.token_type {
           TokenType::PLUS | TokenType::MINUS | TokenType::MULTIPLY | TokenType::DIVIDE => true,
           _ => false
       }
    }

    fn precedence(&self) -> usize {
        match self.token_type {
            TokenType::PLUS | TokenType::MINUS => 1,
            TokenType::MULTIPLY | TokenType::DIVIDE => 2,
            _ => 0
        }
    }
}

fn detect_digits(input: &Vec<char>, start: usize) -> (Token, usize) {
    let mut last_idx = start;
    let mut digits = String::new();
    while last_idx < input.len() {
        if input[last_idx].is_digit(10) {
            digits.push(input[last_idx]);
        } else {
            break;
        }
        last_idx += 1;
    }

    (Token::new(digits, TokenType::DIGITS), last_idx)
}

fn tokenizer(input: &str) -> Vec<Token> {
    let mut token_list = Vec::new();
    let char_vec: Vec<char> = input.chars().collect();
    let mut idx = 0;

    while idx < char_vec.len() {
        let ch = char_vec[idx];
        if ch.is_digit(10) {
            let (token, last_idx) = detect_digits(&char_vec, idx);
            idx = last_idx;
            token_list.push(token);
        } else if ch == '+' {
            let token = Token::new("+".to_string(), TokenType::PLUS);
            token_list.push(token);
            idx += 1;
        } else if ch == '-' {
            let token = Token::new("-".to_string(), TokenType::MINUS);
            token_list.push(token);
            idx += 1;
        } else if ch == '*' {
            let token = Token::new("*".to_string(), TokenType::MULTIPLY);
            token_list.push(token);
            idx += 1;
        } else if ch == '/' {
            let token = Token::new("/".to_string(), TokenType::DIVIDE);
            token_list.push(token);
            idx += 1;
        } else if ch == '(' {
            let token = Token::new("(".to_string(), TokenType::OpenParam);
            token_list.push(token);
            idx += 1;
        } else if ch == ')' {
            let token = Token::new(")".to_string(), TokenType::CloseParam);
            token_list.push(token);
            idx += 1;
        } else {
            idx += 1;
        }
    }

    token_list
}

fn can_push_stack(token: &Token, stack: &Vec<&Token>) -> bool {
    if stack.is_empty() {
        return true;
    }

    stack.last().map_or(false, |last| -> bool {
        last.token_type == TokenType::OpenParam || token.precedence() > last.precedence()
    })
}

fn convert_infix_postfix(infix: &Vec<Token>) -> Vec<&Token> {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();
    for token in infix {
        if token.is_operand() {
            postfix.push(token);
        } else if token.token_type == OpenParam {
            stack.push(token);
        } else if token.token_type == CloseParam {
            while stack.is_empty() == false {
                let last = stack[stack.len() - 1];
                if last.token_type != TokenType::OpenParam {
                    postfix.push(last);
                    stack.pop();
                } else {
                    stack.pop();
                    break;
                }
            }

        } else {
            while stack.is_empty() == false {
                let last = stack[stack.len() - 1];
                if last.precedence() >= token.precedence() {
                    postfix.push(last);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(token);
        }
    }
    while stack.is_empty() == false {
        let last = stack[stack.len() - 1];
        postfix.push(last);
        stack.pop();
    }
    postfix
}

fn print_token_ref_list(token_list: &Vec<&Token>) {
    for token in token_list {
        print!("{}", token.token)
    }
    println!()
}

fn print_token_list(token_list: &Vec<Token>) {
    for token in token_list {
        print!("{}", token.token)
    }
    println!()
}

fn main() {
    let input = "1 + 2 * (3 + 4) / 5";
    let infix = tokenizer(input);
    print_token_list(&infix);
    let postfix = convert_infix_postfix(&infix);
    print_token_ref_list(&postfix);
}
