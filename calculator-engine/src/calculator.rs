use crate::token::{CalculateError, Token, TokenType};
use crate::{CalculationTraceDetails, CalculationTracer};

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

    (Token::new(digits, TokenType::Number), last_idx)
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
            let token = Token::new("+".to_string(), TokenType::Plus);
            token_list.push(token);
            idx += 1;
        } else if ch == '-' {
            let token = Token::new("-".to_string(), TokenType::Minus);
            token_list.push(token);
            idx += 1;
        } else if ch == '*' {
            let token = Token::new("*".to_string(), TokenType::Multiply);
            token_list.push(token);
            idx += 1;
        } else if ch == '/' {
            let token = Token::new("/".to_string(), TokenType::Divide);
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

fn convert_infix_postfix(
    infix: Vec<Token>,
    mut tracer: Option<CalculationTracer>,
) -> (Vec<Token>, Option<CalculationTracer>) {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();

    if let Some(ref mut t) = tracer {
        t.set_infix(&infix);
    }

    for token in infix {
        if token.is_operand() {
            postfix.push(token);
            if let Some(ref mut t) = tracer {
                t.add_postfix_trace(&stack, &postfix);
            }
        } else if token.token_type == TokenType::OpenParam {
            stack.push(token);
            if let Some(ref mut t) = tracer {
                t.add_postfix_trace(&stack, &postfix);
            }
        } else if token.token_type == TokenType::CloseParam {
            while let Some(last) = stack.last() {
                if last.token_type != TokenType::OpenParam {
                    stack.pop().map(|t| postfix.push(t));
                    if let Some(ref mut t) = tracer {
                        t.add_postfix_trace(&stack, &postfix);
                    }
                } else {
                    stack.pop();
                    if let Some(ref mut t) = tracer {
                        t.add_postfix_trace(&stack, &postfix);
                    }
                    break;
                }
            }
        } else {
            while let Some(last) = stack.last() {
                if last.precedence() >= token.precedence() {
                    stack.pop().map(|t| postfix.push(t));
                    if let Some(ref mut t) = tracer {
                        t.add_postfix_trace(&stack, &postfix);
                    }
                } else {
                    break;
                }
            }
            stack.push(token);
            if let Some(ref mut t) = tracer {
                t.add_postfix_trace(&stack, &postfix);
            }
        }
    }
    while stack.is_empty() == false {
        stack.pop().map(|t| postfix.push(t));
        if let Some(ref mut t) = tracer {
            t.add_postfix_trace(&stack, &postfix);
        }
    }

    (postfix, tracer)
}

fn print_token_list(token_list: &Vec<Token>) {
    for token in token_list {
        print!("{}", token.token)
    }
    println!()
}

fn calculate_token(
    operator: &Token,
    value1: &Token,
    value2: &Token,
) -> Result<f64, CalculateError> {
    match &operator.token_type {
        TokenType::Plus => Ok(value1.float()? + value2.float()?),
        TokenType::Minus => Ok(value1.float()? - value2.float()?),
        TokenType::Multiply => Ok(value1.float()? * value2.float()?),
        TokenType::Divide => Ok(value1.float()? / value2.float()?),
        _ => Err(CalculateError::FailedCalculate(format!(
            "{} {} {}",
            value1.token.to_string(),
            operator.token.to_string(),
            value2.token.to_string(),
        ))),
    }
}

fn calculate(
    postfix: Vec<Token>,
    mut tracer: Option<CalculationTracer>,
) -> Result<(f64, Option<CalculationTracer>), CalculateError> {
    let mut stack = Vec::new();
    let mut current: f64 = 0.;

    if let Some(ref mut t) = tracer {
        t.set_postfix(&postfix);
    }

    for token in postfix {
        if token.is_operand() {
            let clone_token = token.token.clone();
            stack.push(token);
            if let Some(ref mut t) = tracer {
                t.add_calculation_trace(&stack, clone_token, current);
            }
        } else {
            let value1 = stack.pop().ok_or(CalculateError::StackEmptyCalculation)?;
            let value2 = stack.pop().ok_or(CalculateError::StackEmptyCalculation)?;
            current = calculate_token(&token, &value2, &value1)?;
            let created_token = Token::new(current.to_string(), TokenType::Number);
            stack.push(created_token);
            if let Some(ref mut t) = tracer {
                t.add_calculation_trace(&stack, token.token.clone(), current);
            }
        }
    }

    let res = stack
        .pop()
        .map(|token| token.float())
        .ok_or(CalculateError::StackEmptyCalculation)?;

    res.map(|ans| (ans, tracer))
}

pub fn calculate_str(
    input: &str,
    enable_trace: bool,
) -> Result<(f64, Option<CalculationTraceDetails>), CalculateError> {
    let infix = tokenizer(input);
    print_token_list(&infix);

    let tracer = if enable_trace {
        Some(CalculationTracer::new())
    } else {
        None
    };

    let (postfix, tracer) = convert_infix_postfix(infix, tracer);
    print_token_list(&postfix);
    let (ans, tracer) = calculate(postfix, tracer)?;
    if let Some(ref tracer) = tracer {
        let details = tracer.trace_details();
        let json =
            serde_json::to_string(&details).map_err(|_| CalculateError::CalculationTraceError)?;
        println!("{}", json);
        return Ok((ans, Some(details)));
    }

    Ok((ans, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_zero() {
        let input = "1/0";
        let (result, _) = calculate_str(input, true).unwrap();
        assert_eq!(result, f64::INFINITY);
    }

    #[test]
    fn it_works() {
        let input = "1 + 2 * (3 + 4) / 2"; // expected 1234+*2/+
        let (result, _trace_details) = calculate_str(input, true).unwrap();
        assert_eq!(result, 8.);
    }
}
