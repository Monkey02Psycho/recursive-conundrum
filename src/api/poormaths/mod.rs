use std::{
    convert::TryFrom,
    fmt::{self, Display},
    ops::{Add, Div, Mul, Sub},
};

use rocket::serde::{json::Json, Deserialize, Serialize};

/// Represents a mathmatical Oparation. In the future it will deseralize from
/// +-*/ intead of the literal name
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Op {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Sub,
    #[serde(rename = "*")]
    Mul,
    #[serde(rename = "/")]
    Div,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Equation {
    left: f64,
    op: Op,
    right: f64,
}

#[cfg(test)]
mod tests {
    use super::Equation;

    #[test]
    fn solve_equation() {
        let eq: Equation = serde_json::from_str(
            "{
            \"left\": 3.0,
            \"op\": \"*\",
            \"right\": 10.0}",
        )
        .unwrap();
        assert_eq!(eq.solve(), 30.0f64);
    }

    #[test]
    fn solve_with_error() {
        let eq: Equation = serde_json::from_str(
            "{
            \"left\": 40,
            \"op\": \"*\",
            \"right\": 10}",
        )
        .unwrap();
        let result = eq.solve_with_error();
        let lower = 400.0 - (400.0 * 0.05);
        let upper = 400.0 + (400.0 * 0.05);
        if (result >= lower) && (result <= upper) {
            // println!("{}", result);
            assert!(true);
        } else {
            // println!("{}", result);
            panic!("Result is not within the bounds. result is: {:?}", result);
        }
    }

    #[test]
    fn solve_with_error_trials() {
        for _ in 0..255 {
            solve_with_error()
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &Op::Add => '+',
                Op::Sub => '-',
                Op::Mul => '*',
                Op::Div => '/',
            }
        )
    }
}

pub struct InvalidOP;

impl TryFrom<String> for Op {
    type Error = InvalidOP;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(InvalidOP),
        }
    }
}

impl Equation {
    fn solve(&self) -> f64 {
        match self.op {
            Op::Add => self.left.add(self.right),
            Op::Sub => self.left.sub(self.right),
            Op::Mul => self.left.mul(self.right),
            Op::Div => self.left.div(self.right),
        }
    }

    fn solve_with_error(&self) -> f64 {
        let arm = self.solve();
        let mut result = self.solve();
        // println!("{}", result);
        // a 5% error
        let error = arm * 0.05;
        // should return an error within 5% of the original
        match rand::random::<bool>(){
            true => result += rand::random::<f64>() * error,
            false => result -= rand::random::<f64>() * error,
        }
        // println!("{}", result);
        result
    }
}

// https://rocket.rs/v0.5-rc/guide/requests/#json

#[post("/solve", format = "json", data = "<equation>")]
pub fn solve_eq(equation: Json<Equation>) -> String {
    equation.into_inner().solve().to_string()
    // "test".into()
}

#[post("/solveerror", format = "json", data = "<equation>")]
pub fn solve_eq_error(equation: Json<Equation>) -> String {
    equation.into_inner().solve_with_error().to_string()
    // "test".into()
}
