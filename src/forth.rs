use std::ops::{Add, Div, Mul, Sub};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type Stack = Vec<Value>;

pub struct Forth {
    stack: Stack,
    definitions: Vec<Variable>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
}

pub enum InputValue<'a> {
    Number(Value),
    Operator(Operator),
    Definition,
    Variable(&'a Variable),
    Void,
}

#[derive(Clone, Debug)]
pub struct Variable {
    word: String,
    definition: Vec<String>,
    definitions_index: usize,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Vec::new(),
            definitions: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack[..]
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input: Vec<&str> = input.split_whitespace().collect();
        let mut stack: Stack = Vec::with_capacity(input.len());
        let mut i = 0;
        while i < input.len() {
            let definitions = &self.definitions.clone();
            self.evaluate_character(&mut i, &input, &mut stack, &definitions)?;
            i += 1;
        }
        self.stack = stack;
        return Ok(());
    }

    fn evaluate_character(
        &mut self,
        i: &mut usize,
        input: &Vec<&str>,
        stack: &mut Stack,
        definitions: &Vec<Variable>,
    ) -> Result {
        // println!("i {}", i);
        let input_value = self.evaluate_input(input.get(*i).unwrap(), definitions);
        match input_value {
            InputValue::Number(num) => stack.push(num),
            InputValue::Operator(op) => {
                op.operate(stack)?;
            }
            InputValue::Definition => {
                let inp = input.iter();
                let definition: Vec<String> = inp
                    .skip(*i + 2)
                    .map(|x| String::from(*x))
                    .take_while(|x| x != &";".to_string())
                    .collect();
                if None == input.iter().position(|x| x == &";") || definition.len() < 1 {
                    return Err(Error::InvalidWord);
                };
                let definition_name = input.get(*i + 1).unwrap();
                if let Some(c) = definition_name.chars().nth(0) {
                    if c.is_digit(10) {
                        return Err(Error::InvalidWord);
                    }
                }
                *i += definition.len() + 2;

                self.definitions.push(Variable {
                    word: String::from(definition_name.to_uppercase()),
                    definition: definition.into(),
                    definitions_index: self.definitions.len(),
                });
            }
            InputValue::Variable(variable) => {
                let mut j = 0;
                let definition = &variable.definition;
                let len = definition.len();
                let definition = definition.clone();
                let definition = definition.iter().map(|x| x.as_ref()).collect();
                while j < len {
                    self.evaluate_character(
                        &mut j,
                        &definition,
                        stack,
                        &self
                            .definitions
                            .clone()
                            .split_at(variable.definitions_index)
                            .0
                            .into(),
                    )?;
                    j += 1;
                }
            }
            InputValue::Void => return Err(Error::UnknownWord),
        }
        Ok(())
    }
    fn evaluate_input<'a>(&self, val: &str, definitions: &'a Vec<Variable>) -> InputValue<'a> {
        // println!("{}", val);
        if val == ":" {
            return InputValue::Definition;
        };
        // println!("{:?}", definitions);
        if let Some(variable) = definitions
            .iter()
            .rev()
            .find(|x| x.word == val.to_uppercase())
        {
            return InputValue::Variable(variable);
        }
        let operator = match &val.to_uppercase()[..] {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            "DUP" => Some(Operator::Dup),
            "DROP" => Some(Operator::Drop),
            "SWAP" => Some(Operator::Swap),
            "OVER" => Some(Operator::Over),
            _ => None,
        };
        if let Some(op) = operator {
            return InputValue::Operator(op);
        };

        if let Ok(number) = val.parse::<Value>() {
            return InputValue::Number(number);
        };

        return InputValue::Void;
    }
}

impl Operator {
    fn operate(&self, stack: &mut Stack) -> Result {
        match self {
            Self::Add => {
                if stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                };
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a.add(b));
                Ok(())
            }
            Self::Sub => {
                if stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                };
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b.sub(a));
                Ok(())
            }
            Self::Mul => {
                if stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                };
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b.mul(a));
                Ok(())
            }
            Self::Div => {
                if stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                };
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                if a == 0 {
                    return Err(Error::DivisionByZero);
                };
                stack.push(b.div(a));
                Ok(())
            }
            Self::Dup => {
                if stack.len() < 1 {
                    return Err(Error::StackUnderflow);
                };
                let a = stack.pop().unwrap();
                stack.push(a);
                stack.push(a);
                Ok(())
            }
            Self::Drop => {
                if stack.len() < 1 {
                    return Err(Error::StackUnderflow);
                };
                stack.pop().unwrap();
                Ok(())
            }
            Self::Swap => {
                if stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                };
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a);
                stack.push(b);
                Ok(())
            }
            Self::Over => {
                if stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                };
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
                stack.push(b);

                Ok(())
            }
        }
    }
}
