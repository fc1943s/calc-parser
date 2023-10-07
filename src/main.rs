#[derive(Debug, PartialEq)]
pub enum EvalError {
    DivisionByZero,
    InvalidCharacter,
    InvalidBlock,
    InvalidInput,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn evaluate(expr: &str) -> Result<f64, EvalError> {
    fn operate(op: Operator, a: f64, b: f64) -> Result<f64, EvalError> {
        match op {
            Operator::Add => Ok(a + b),
            Operator::Subtract => Ok(a - b),
            Operator::Multiply => Ok(a * b),
            Operator::Divide if b != 0. => Ok(a / b),
            Operator::Divide => Err(EvalError::DivisionByZero),
        }
    }

    fn parse_number(iter: &mut std::iter::Peekable<std::str::Chars>, number: f64) -> f64 {
        match iter.peek() {
            Some('0'..='9') => {
                let new_number = number * 10. + iter.next().unwrap().to_digit(10).unwrap() as f64;
                parse_number(iter, new_number)
            }
            _ => number,
        }
    }

    fn parse_group(
        iter: &mut std::iter::Peekable<std::str::Chars>,
        count: i32,
        inner: String,
    ) -> Result<String, EvalError> {
        match iter.next().ok_or(EvalError::InvalidBlock)? {
            'e' => parse_group(iter, count + 1, inner),
            'f' if count == 1 => Ok(inner),
            'f' => parse_group(iter, count - 1, inner),
            c => parse_group(iter, count, inner + &c.to_string()),
        }
    }

    fn parse(
        iter: &mut std::iter::Peekable<std::str::Chars>,
        operands: (Option<f64>, Option<f64>),
        operator: Option<Operator>,
    ) -> Result<f64, EvalError> {
        match iter.peek() {
            Some(c) => match c {
                '0'..='9' => {
                    let number = parse_number(iter, 0.);
                    let new_operands = match (operator.clone(), operands) {
                        (Some(op), (Some(a), Some(b))) => (Some(operate(op, a, b)?), Some(number)),
                        (_, (Some(a), None)) => (Some(a), Some(number)),
                        _ => (Some(number), None),
                    };
                    parse(iter, new_operands, operator)
                }
                'a'..='d' => {
                    let next_op = match iter.next().unwrap() {
                        'a' => Some(Operator::Add),
                        'b' => Some(Operator::Subtract),
                        'c' => Some(Operator::Multiply),
                        'd' => Some(Operator::Divide),
                        _ => None,
                    };
                    let new_operands = match (operator, operands) {
                        (Some(op), (Some(a), Some(b))) => (Some(operate(op, a, b)?), None),
                        _ => operands,
                    };
                    parse(iter, new_operands, next_op)
                }
                'e' => {
                    iter.next().unwrap();
                    let inner = parse_group(iter, 1, String::new())?;
                    let new_operand = evaluate(&inner)?;
                    let new_operands = match operands {
                        (Some(a), _) => (Some(a), Some(new_operand)),
                        _ => (Some(new_operand), None),
                    };
                    parse(iter, new_operands, operator)
                }
                _ => Err(EvalError::InvalidCharacter),
            },
            None => Ok(
                if let (Some(op), (Some(a), Some(b))) = (operator, operands) {
                    operate(op, a, b)?
                } else {
                    operands.0.ok_or(EvalError::InvalidInput)?
                },
            ),
        }
    }

    Ok(parse(&mut expr.chars().peekable(), (None, None), None)?)
}

#[test]
fn tests() {
    assert_eq!(evaluate("3a2c4").unwrap(), 20.);
    assert_eq!(evaluate("32a2d2").unwrap(), 17.);
    assert_eq!(evaluate("500a10b66c32").unwrap(), 14208.);
    assert_eq!(evaluate("3ae4c66fb32").unwrap(), 235.);
    assert_eq!(evaluate("3c4d2aee2a4c41fc4f").unwrap(), 990.);

    assert_eq!(evaluate("3").unwrap(), 3.);
    assert_eq!(evaluate("2a2a2").unwrap(), 6.);
    assert_eq!(evaluate("2b3c4").unwrap(), -4.);
    assert_eq!(evaluate("4c3b2d4").unwrap(), 2.5);
    assert!(evaluate("4d0").is_err_and(|x| x == EvalError::DivisionByZero));
    assert!(evaluate("3ae4d0fb2").is_err_and(|x| x == EvalError::DivisionByZero));
    assert!(evaluate("3a2z4").is_err_and(|x| x == EvalError::InvalidCharacter));
    assert!(evaluate("32a2d2g").is_err_and(|x| x == EvalError::InvalidCharacter));
    assert!(evaluate("1ae1").is_err_and(|x| x == EvalError::InvalidBlock));
    assert!(evaluate("a").is_err_and(|x| x == EvalError::InvalidInput));
    assert!(evaluate("").is_err_and(|x| x == EvalError::InvalidInput));
}

pub fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let expr = &args[1];
    println!("Evaluating {expr}");
    let result = evaluate(expr);
    match result {
        Ok(v) => println!("Result: {v}"),
        Err(e) => println!("Error: {e:?}"),
    }
    Ok(())
}
