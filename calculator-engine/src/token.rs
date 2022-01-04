use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CalculateError {
    #[error("error on converting from a string to a float")]
    ParseError(std::num::ParseFloatError),
    #[error("failed to calculate the operator")]
    FailedCalculate(String),
    #[error("stack is empty for calculation")]
    StackEmptyCalculation,
    #[error("token is not a number")]
    NotNumber,
    #[error("failure to generate a trace for postfix conversion")]
    PostFixTraceError,
    #[error("failure to generate a calculation trace")]
    CalculationTraceError,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum TokenType {
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParam,
    CloseParam,
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) token: String,
    pub(crate) token_type: TokenType,
}

impl Token {
    pub(crate) fn new(token: String, token_type: TokenType) -> Self {
        Self { token, token_type }
    }
    pub(crate) fn is_operand(&self) -> bool {
        self.token_type == TokenType::Number
    }

    pub(crate) fn float(&self) -> Result<f64, CalculateError> {
        return if self.token_type == TokenType::Number {
            self.token
                .parse()
                .map_err(|e| CalculateError::ParseError(e))
        } else {
            Err(CalculateError::NotNumber)
        };
    }

    pub(crate) fn is_operator(&self) -> bool {
        match self.token_type {
            TokenType::Plus | TokenType::Minus | TokenType::Multiply | TokenType::Divide => true,
            _ => false,
        }
    }

    pub(crate) fn precedence(&self) -> usize {
        match self.token_type {
            TokenType::Plus | TokenType::Minus => 1,
            TokenType::Multiply | TokenType::Divide => 2,
            _ => 0,
        }
    }
}
