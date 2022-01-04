use serde::{Deserialize, Serialize};

use crate::Token;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostTraceItem {
    stack: Vec<String>,
    current: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostFixConversionTrace {
    trace: Vec<PostTraceItem>,
}

impl PostFixConversionTrace {
    pub fn new() -> Self {
        Self { trace: Vec::new() }
    }

    pub(crate) fn add_trace(&mut self, stack: &Vec<Token>, current: &Vec<Token>) {
        let item = PostTraceItem {
            stack: to_string_vec_rev(stack),
            current: to_string_vec(current),
        };
        self.trace.push(item);
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

fn to_string_vec(vec: &Vec<Token>) -> Vec<String> {
    let mut list = Vec::new();
    for token in vec {
        list.push(token.token.clone())
    }
    list
}

fn to_string_vec_rev(vec: &Vec<Token>) -> Vec<String> {
    let mut list = Vec::new();
    for token in vec.iter().rev() {
        list.push(token.token.clone())
    }
    list
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculationTraceItem {
    stack: Vec<String>,
    token: String,
    answer: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculationTrace {
    trace: Vec<CalculationTraceItem>,
}

impl CalculationTrace {
    pub fn new() -> Self {
        Self { trace: Vec::new() }
    }

    pub(crate) fn add_trace(&mut self, stack: &Vec<Token>, token: String, answer: f64) {
        let item = CalculationTraceItem {
            stack: to_string_vec_rev(stack),
            token,
            answer,
        };

        self.trace.push(item);
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculationTraceDetails {
    infix: Vec<String>,
    postfix: Vec<String>,
    postfix_trace: PostFixConversionTrace,
    calculation_trace: CalculationTrace,
}

impl CalculationTraceDetails {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

pub struct CalculationTracer {
    infix: Vec<String>,
    postfix: Vec<String>,
    postfix_trace: PostFixConversionTrace,
    calculation_trace: CalculationTrace,
}

impl CalculationTracer {
    pub fn new() -> Self {
        Self {
            infix: Vec::new(),
            postfix: Vec::new(),
            postfix_trace: PostFixConversionTrace::new(),
            calculation_trace: CalculationTrace::new(),
        }
    }

    pub(crate) fn set_postfix(&mut self, postfix: &Vec<Token>) {
        self.postfix = to_string_vec(postfix);
    }

    pub(crate) fn set_infix(&mut self, infix: &Vec<Token>) {
        self.infix = to_string_vec(infix);
    }

    pub(crate) fn add_postfix_trace(&mut self, stack: &Vec<Token>, current: &Vec<Token>) {
        self.postfix_trace.add_trace(stack, current);
    }

    pub(crate) fn add_calculation_trace(&mut self, stack: &Vec<Token>, token: String, answer: f64) {
        self.calculation_trace.add_trace(stack, token, answer);
    }

    pub fn trace_details(&self) -> CalculationTraceDetails {
        CalculationTraceDetails {
            infix: self.infix.clone(),
            postfix: self.postfix.clone(),
            postfix_trace: self.postfix_trace.clone(),
            calculation_trace: self.calculation_trace.clone(),
        }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        let details = self.trace_details();
        serde_json::to_string(&details)
    }
}
