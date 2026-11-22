use crate::types::{BaseNode, NodeKind, Operation, PrintTokenType, TokenType, VarType};
use std::io::{Error, ErrorKind};

pub fn parse(tokens: &[(String, TokenType)], debug_print: bool) -> Result<BaseNode, Error> {
    if debug_print {
        println!("\n________PARSER RESULTS________\n");
    }
    let mut nodes: Vec<BaseNode> = Vec::new();
    let mut cur_line: Vec<(String, TokenType)> = Vec::new();
    for (value, token) in tokens {
        if matches!(*token, TokenType::EndLine) {
            let node: BaseNode = match cur_line[0].1 {
                TokenType::Dim => handle_dim(&cur_line, debug_print)?,
                TokenType::Unknown => handle_assignment(&cur_line, debug_print)?,
                TokenType::Print => handle_print(&cur_line, debug_print),
                _ => BaseNode {
                    kind: NodeKind::Unknown,
                    children: Vec::new(),
                },
            };
            nodes.push(node);
            cur_line.clear();
        } else {
            cur_line.push((String::from(value), *token));
        }
    }
    Ok(BaseNode {
        kind: NodeKind::Program,
        children: nodes,
    })
}

fn handle_dim(cur_line: &[(String, TokenType)], debug_print: bool) -> Result<BaseNode, Error> {
    if cur_line.len() != 4 || cur_line[2].0 != "As" {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Incorrect syntax for variable initialization",
        ));
    }
    let var_name = String::from(cur_line[1].0.trim());
    let var_type_string = String::from(cur_line[3].0.trim());
    if debug_print {
        println!(
            "Variable name is \"{}\", type starts with \"{}\"",
            var_name, var_type_string
        );
    }
    let var_type = if var_type_string == "String" {
        VarType::String
    } else if var_type_string == "Integer" {
        VarType::Integer
    } else {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Unsupported type for Dim statement",
        ));
    };
    Ok(BaseNode {
        kind: NodeKind::Dim { var_name, var_type },
        children: Vec::new(),
    })
}

fn handle_assignment(
    cur_line: &[(String, TokenType)],
    debug_print: bool,
) -> Result<BaseNode, Error> {
    if cur_line.len() < 3 || cur_line[1].0 != "=" {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Incorrect syntax for variable assignment",
        ));
    }
    let var_name = String::from(cur_line[0].0.trim());
    let line_values = &cur_line[2..];
    let mut quote_count = 0;
    let mut nodes = vec![];
    for (i, (_, token)) in line_values.iter().enumerate() {
        if matches!(token, TokenType::Quote) {
            quote_count += 1;
            if quote_count > 2 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Can only have one String in an assignment",
                ));
            }
            if quote_count % 2 == 1 {
                if !(matches!(line_values.get(i + 1), Some((_, TokenType::Quote)))
                    || matches!(line_values.get(i + 2), Some((_, TokenType::Quote))))
                {
                    return Err(Error::new(ErrorKind::InvalidData, "Unclosed string"));
                } else {
                    let str_value = line_values.get(i + 1);
                    if !matches!(str_value, Some((_, TokenType::Quote))) {
                        nodes.push(BaseNode {
                            kind: NodeKind::StringValue {
                                value: str_value.unwrap().0.clone(),
                            },
                            children: Vec::new(),
                        })
                    }
                }
            }
            continue;
        } else if quote_count > 2 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Can only have one String in an assignment",
            ));
        }
        if matches!(token, TokenType::Addition) {
            if i == 0 {
                return Err(Error::new(ErrorKind::InvalidData, "No value before a '+'"));
            }
            if quote_count % 2 == 0 {
                let mut op_vals: Vec<BaseNode> = Vec::new();
                let first = line_values.get(i - 1);
                if first.is_none() {
                    return Err(Error::new(ErrorKind::InvalidData, "No value before a '+'"));
                } else {
                    let first_num: i16 = match first.unwrap().0.clone().trim().parse() {
                        Ok(v) => v,
                        Err(_e) => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Couldn't parse a number",
                            ));
                        }
                    };
                    op_vals.push(BaseNode {
                        kind: NodeKind::MathValue { value: first_num },
                        children: Vec::new(),
                    });
                }
                let second = line_values.get(i + 1);
                if second.is_none() {
                    return Err(Error::new(ErrorKind::InvalidData, "No value after a '+'"));
                } else {
                    let second_num: i16 = match second.unwrap().0.clone().trim().parse() {
                        Ok(v) => v,
                        Err(_e) => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Couldn't parse a number",
                            ));
                        }
                    };
                    op_vals.push(BaseNode {
                        kind: NodeKind::MathValue { value: second_num },
                        children: Vec::new(),
                    });
                }
                nodes.push(BaseNode {
                    kind: NodeKind::MathOp {
                        op: Operation::Addition,
                    },
                    children: op_vals,
                })
            }
        }
    }
    if debug_print {
        println!("Assigning value to \"{}\"", var_name);
    }
    Ok(BaseNode {
        kind: NodeKind::Assignment { var_name },
        children: nodes,
    })
}

fn handle_print(cur_line: &[(String, TokenType)], debug_print: bool) -> BaseNode {
    let mut print_tokens: Vec<(String, PrintTokenType)> = Vec::new();
    let mut print_output = String::new();
    let mut quote_count = 0;
    for (value, token) in &cur_line[1..] {
        if matches!(token, TokenType::Quote) {
            quote_count += 1;
        } else {
            if !matches!(token, TokenType::Ampersand) {
                let mut cur_value = value.as_str();
                let is_var = quote_count % 2 == 0 && quote_count != 0;
                if is_var {
                    print_output.push_str("<var>");
                    cur_value = cur_value.trim();
                }
                print_output.push_str(cur_value);
                print_tokens.push((
                    String::from(cur_value),
                    if is_var {
                        PrintTokenType::Var
                    } else {
                        PrintTokenType::String
                    },
                ));
                if is_var {
                    print_output.push_str("</var>");
                }
            }
        }
    }
    if debug_print {
        println!("Printing: \"{}\"", print_output);
    }
    BaseNode {
        kind: NodeKind::Print { print_tokens },
        children: Vec::new(),
    }
}
