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

    fn float(&self) -> Option<f64> {
        return if self.token_type == TokenType::DIGITS {
            self.token.parse().ok()
        } else {
            None
        }
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

// algorithm from https://www.tutorialspoint.com/Convert-Infix-to-Postfix-Expression
// Begin
//    initially push some special character say # into the stack
//    for each character ch from infix expression, do
//       if ch is alphanumeric character, then
//          add ch to postfix expression
//       else if ch = opening parenthesis (, then
//          push ( into stack
//       else if ch = ^, then            //exponential operator of higher precedence
//          push ^ into the stack
//       else if ch = closing parenthesis ), then
//          while stack is not empty and stack top ≠ (,
//             do pop and add item from stack to postfix expression
//          done
//
//          pop ( also from the stack
//       else
//          while stack is not empty AND precedence of ch <= precedence of stack top element, do
//             pop and add into postfix expression
//          done
//
//          push the newly coming character.
//    done
//
//    while the stack contains some remaining characters, do
//       pop and add to the postfix expression
//    done
//    return postfix
// End
// int precedence(char ch) {
//     if(ch == '+' || ch == '-') {
//        return 1;              //Precedence of + or - is 1
//     }else if(ch == '*' || ch == '/') {
//        return 2;            //Precedence of * or / is 2
//     }else if(ch == '^') {
//        return 3;            //Precedence of ^ is 3
//     }else {
//        return 0;
//     }
// }
// verification: https://raj457036.github.io/Simple-Tools/prefixAndPostfixConvertor.html

fn convert_infix_postfix(infix: Vec<Token>) -> Vec<Token> {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();
    for token in infix {
        if token.is_operand() {
            postfix.push(token);
        } else if token.token_type == TokenType::OpenParam {
            stack.push(token);
        } else if token.token_type == TokenType::CloseParam {
            while stack.is_empty() == false {
                let last = &stack[stack.len() - 1];
                if last.token_type != TokenType::OpenParam {
                    stack.pop().map(|t| postfix.push(t));
                } else {
                    stack.pop();
                    break;
                }
            }

        } else {
            while stack.is_empty() == false {
                let last = &stack[stack.len() - 1];
                if last.precedence() >= token.precedence() {
                    stack.pop().map(|t| postfix.push(t));
                } else {
                    break;
                }
            }
            stack.push(token);
        }
    }
    while stack.is_empty() == false {
        stack.pop().map(|t| postfix.push(t));
    }
    postfix
}

fn print_token_list(token_list: &Vec<Token>) {
    for token in token_list {
        print!("{}", token.token)
    }
    println!()
}

fn calculate(postfix: &Vec<Token>) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "1 + 2 * (3 + 4) / 5"; // expected 1234+*5/+
        let infix = tokenizer(input);
        print_token_list(&infix);
        let postfix = convert_infix_postfix(infix);
        print_token_list(&postfix);
        let result = calculate(&postfix);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}