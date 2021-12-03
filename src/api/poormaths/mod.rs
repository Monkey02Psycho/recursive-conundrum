use std::{
    convert::TryFrom,
    fmt::{self, Display},
    ops::{Add, Div, Mul, Sub},
};

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Equation {
    left: f64,
    op: Op,
    right: f64,
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
        match &self.op {
            Op::Add => self.left.add(self.right),
            Op::Sub => self.left.sub(self.right),
            Op::Mul => self.left.mul(self.right),
            Op::Div => self.left.div(self.right),
        }
    }
}

// https://rocket.rs/v0.5-rc/guide/requests/#json
