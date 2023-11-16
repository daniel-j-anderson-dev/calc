use std::{io::{self, Write, stdin}, str::FromStr, fmt::Display};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // greeting 
    println!("Simple Terminal Calculator\nSupported operations: + - * / ^");

    // use the calculator
    loop {
        // get input
        let input = get_input("> ")?;
        
        // check if user wants to quit
        if input.to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        // if the user didn't want to quit parse the input into an `Expression`
        let expression: Expression = match input.parse() { 
            Ok(parsed_expression) => parsed_expression,
            Err(error) => {
                eprintln!("Invalid input: {}\nTry again", error);
                continue;
            },
        }; 

        // evaluate the input `Expression`
        match expression.evaluate() { 
            Ok(result) => println!("{} = {}", expression, result),
            Err(error) => {
                eprintln!("Error evaluating expression: {}\nTry again", error);
                continue;
            }, 
        }
    }

    Ok(())
}

struct Expression {
    lhs: f64,
    rhs: f64,
    operation: Operation,
}
impl Expression {
    pub fn evaluate(&self) -> Result<f64, Box<dyn std::error::Error>> {
        match self.operation {
            Operation::Add         => Ok(self.lhs + self.rhs),
            Operation::Subtract    => Ok(self.lhs - self.rhs),
            Operation::Multiply    => Ok(self.lhs * self.rhs),
            Operation::Exponential => Ok(self.lhs.powf(self.rhs)),
            Operation::Divide 
                if self.rhs != 0.0 => Ok(self.lhs / self.rhs),
            Operation::Divide      => Err("Divide by zero error".into()),
        }
    }
}
impl FromStr for Expression { // Trait that allows .parse to work

    type Err = Box<dyn std::error::Error>; // parse error type

    /// Parse an `Expression` from `s`.<br>
    /// `s` must start with a number
    /// # Parameters
    ///  - `s`: The string slice to be parsed
    /// # Returns
    ///  - `Ok(expression)`: When `s` is one of the supported operation characters,
    ///  - `Err(from_str_error)`: When `s` is not one of the supported operation characters,
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string = String::new();
        for char in s.chars() {
            if !char.is_whitespace() {
                string.push(char);
            }
        } 

        let mut lhs = String::new();
        let mut current_index = 0;
        for (i, char) in string.chars().enumerate() {
            if char.is_digit(10) || char == '.' {
                lhs.push(char);
            }
            else {
                current_index = i; // save index of first non-digit non. char
                break;
            }
        }
        let lhs: f64 = match lhs.parse() {
            Ok(parsed_lhs) => parsed_lhs,
            Err(error) => return Err(format!("Failed to parse left hand side: {}", error).into()),
        };

        let operation = match string.chars().nth(current_index) {
            Some(operation) => match operation.to_string().parse() {
                Ok(parsed_operation) => parsed_operation,
                Err(error) => return Err(format!("Failed to parse operation: {}", error).into()),
            },
            None => return Err("Missing operator".into()),
        };
        current_index += 1;

        println!("{}", &string[current_index..]);
        let rhs: f64 = match string[current_index..].parse() { // parse the remainder of `string` ignore
            Ok(parsed_rhs) => parsed_rhs,
            Err(error) => return Err(format!("Failed to parse right hand side: {}", error).into()),
        };

        Ok(Expression { lhs, rhs, operation })
    }
}
impl Display for Expression { // allows for `println!()` and `.to_string()`

    /// writes a the expression to the formatter `f`
    /// # Parameters
    ///  - `f`: the `Formatter` that we will write the expression to. (can be a string or stdout) 
    /// # Returns
    ///  - `Ok(())`: if `write!` succeeds
    ///  - `Err(format_error)`: if `write!` fails
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.operation, self.rhs)
    }
}

/// An enumeration representing each supported operation
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponential,
}
impl FromStr for Operation { // Trait that allows `.parse()` to work

    type Err = Box<dyn std::error::Error>; // parse error type

    /// Creates a new instance of Operation if the `s` is a supported operation.<br>
    /// supported operation characters: `+` `-` `*` `/` `^`
    /// # Parameters
    ///  - `s`: The string slice to be parsed
    /// # Returns
    ///  - `Ok(operation)`: When `s` is one of the supported operation characters,
    ///  - `Err(from_str_error)`: When `s` is not one of the supported operation characters,
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            "^" => Ok(Operation::Exponential),
            _ => Err("Invalid operator. Supported operators: + - * / ^".into())
        }
    }
}

impl Display for Operation { // allows for `println!()` and `.to_string()`

    /// writes a character corresponding to self's variant
    /// # Parameters
    ///  - `f`: the `Formatter` that we will write the operation character to. (can be a string or stdout) 
    /// # Returns
    ///  - `Ok(())`: if `write!` succeeds
    ///  - `Err(format_error)`: if `write!` fails
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `write!` the character corresponding to `self`'s variant to `f`
        write!(f, "{}", match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
            Operation::Exponential => "^",
        })
    }
}

// get user input
fn get_input(prompt: &str) -> Result<String, io::Error> {
    io::stdout().write(prompt.as_bytes())?;
    io::stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = input.trim().to_owned();

    Ok(input)
}