use std::fmt;
use std::io::ErrorKind;
use std::{collections::HashMap, io::Error};

use crate::types::{BaseNode, NodeKind, Operation, PrintTokenType, VarType};

#[derive(Clone)]
enum Value {
    Integer(i16),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

pub fn evaluate(ast_root: BaseNode, debug_print: bool) -> Result<(), Error> {
    if debug_print {
        println!("\n________EVALUATOR RESULTS________\n");
    }
    let mut vars: HashMap<String, (Option<Value>, VarType)> = HashMap::new();
    'outer: for node in ast_root.children {
        if let NodeKind::Dim { var_name, var_type } = &node.kind {
            vars.insert(var_name.clone(), (None, var_type.clone()));
        }
        if let NodeKind::Assignment { var_name } = &node.kind {
            let (_, var_type) = vars.get(var_name).unwrap();
            for child in node.children {
                if let NodeKind::StringValue { value: v } = &child.kind {
                    if !matches!(var_type, VarType::String) {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("{} should be a String, found {}", var_name, var_type),
                        ));
                    }
                    vars.insert(
                        var_name.clone(),
                        (Some(Value::String(v.clone())), var_type.clone()),
                    );
                    continue 'outer;
                }
                if let NodeKind::MathOp { op } = &child.kind {
                    if matches!(op, Operation::Addition) {
                        let mut result: i16 = 0;
                        for child in &child.children {
                            if let NodeKind::MathValue { value } = &child.kind {
                                result += value;
                            }
                        }
                        vars.insert(
                            var_name.clone(),
                            (Some(Value::Integer(result)), var_type.clone()),
                        );
                        continue 'outer;
                    }
                }
            }
        }
        if let NodeKind::Print { print_tokens } = &node.kind {
            let mut print_output: String = String::new();
            for (token_value, token_type) in print_tokens {
                if matches!(token_type, PrintTokenType::String) {
                    print_output.push_str(token_value);
                } else {
                    let (saved_value, _) = vars.get(token_value).unwrap();
                    print_output.push_str(saved_value.clone().unwrap().to_string().as_str())
                }
            }
            println!("{}", print_output);
        }
    }
    Ok(())
}
