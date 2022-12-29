#[derive(Debug)]
pub enum CalcError {
    UnknownCharacter,
}

#[derive(Debug)]
enum BracketType {
    Open,
    Close,
}

#[derive(Debug)]
enum Operation {
    Value(i8),
    Dot,
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Power,
    Remainder,
    Bracket(BracketType),
}

fn parse_operations(pattern: &str) -> Result<Vec<Operation>, CalcError> {
    let mut operations: Vec<Operation> = vec![];

    for char in pattern.chars() {
        let v = match char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Operation::Value(char.to_digit(10).unwrap() as i8)
            }
            '.' => Operation::Dot,
            '+' => Operation::Addition,
            '-' => Operation::Subtraction,
            '/' => Operation::Division,
            '*' => Operation::Multiplication,
            '^' => Operation::Power,
            '%' => Operation::Remainder,
            '(' => Operation::Bracket(BracketType::Open),
            ')' => Operation::Bracket(BracketType::Close),
            _ => Err(CalcError::UnknownCharacter)?,
        };

        operations.push(v);
    }

    Ok(operations)
}

fn get_priority(operation: Operation) -> u8 {
    match operation {
        Operation::Value(_) => 1,
        Operation::Dot => 1,
        Operation::Addition => 2,
        Operation::Subtraction => 2,
        Operation::Division => 3,
        Operation::Multiplication => 3,
        Operation::Power => 4,
        Operation::Remainder => 3,
        Operation::Bracket(_) => 5,
    }
}

pub fn calculate(pattern: &str) -> Result<f64, CalcError> {
    let pattern = pattern.replace(" ", "");
    let operations = parse_operations(&pattern)?;

    for operation in operations {
        let priority = get_priority(operation);
    }

    Ok(0.0)
}

#[cfg(test)]
mod tests {
    use super::{calculate, CalcError};

    #[test]
    fn it_works() -> Result<(), CalcError> {
        let cases = vec![
            ("2 + 2", 4.0f64),
            ("2 - 2", 0f64),
            ("2 * 3", 6f64),
            ("6 / 3", 2f64),
            ("6 / 2", 3f64),
            ("3 ^ 3", 27f64),
            ("(2 + 3) * 2", 10f64),
            ("2 + 3 * 2", 8f64),
            ("-2 -2", -4f64),
        ];

        for (value, result) in cases.into_iter() {
            let v = calculate(value);
            assert_eq!(v?, result);
        }

        Ok(())
    }
}
