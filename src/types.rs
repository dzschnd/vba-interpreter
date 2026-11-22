use core::fmt::{self, Display};
use std::{collections::HashMap, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Print,
    Dim,
    As,
    String,
    Integer,
    Quote,
    Ampersand,
    Eq,
    Less,
    More,
    Exponentiation,
    Negation,
    Multiplication,
    Division,
    IntDivision,
    Modulus,
    Addition,
    Subtraction,
    LeftParenthesis,
    RightParenthesis,
    EndLine,
    Unknown,
}

pub static TOKENS_MAP: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("Debug.Print", TokenType::Print),
        ("Dim", TokenType::Dim),
        ("As", TokenType::As),
        ("String", TokenType::String),
        ("Integer", TokenType::Integer),
        ("\"", TokenType::Quote),
        ("&", TokenType::Ampersand),
        ("=", TokenType::Eq),
        ("<", TokenType::Less),
        (">", TokenType::More),
        ("^", TokenType::Exponentiation),
        ("-", TokenType::Negation),
        ("*", TokenType::Multiplication),
        ("/", TokenType::Division),
        ("\\", TokenType::IntDivision),
        ("Mod", TokenType::Modulus),
        ("+", TokenType::Addition),
        ("-", TokenType::Subtraction),
        ("(", TokenType::LeftParenthesis),
        (")", TokenType::RightParenthesis),
        ("\n", TokenType::EndLine),
    ])
});

pub enum Operation {
    Addition,
}

pub enum NodeKind {
    Program,
    Dim {
        var_name: String,
        var_type: VarType,
    },
    Assignment {
        var_name: String,
    },
    StringValue {
        value: String,
    },
    MathValue {
        value: i16,
    },
    MathOp {
        op: Operation,
    },
    Print {
        print_tokens: Vec<(String, PrintTokenType)>,
    },
    Unknown,
}

#[derive(Clone)]
pub enum VarType {
    String,
    Integer,
}

impl Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarType::String => write!(f, "String"),
            VarType::Integer => write!(f, "Integer"),
        }
    }
}

pub enum PrintTokenType {
    String,
    Var,
}

impl Display for PrintTokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrintTokenType::String => write!(f, "String"),
            PrintTokenType::Var => write!(f, "Var"),
        }
    }
}

pub struct BaseNode {
    pub kind: NodeKind,
    pub children: Vec<BaseNode>,
}
