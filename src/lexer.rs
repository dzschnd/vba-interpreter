use crate::types::{TOKENS_MAP, TokenType};

pub fn lex(text: &str, debug_print: bool) -> Vec<(String, TokenType)> {
    let mut tokens = vec![];
    let mut i = 0;
    let mut unknown_chars: Vec<char> = Vec::new();
    'outer: while i < text.len() {
        for pattern in TOKENS_MAP.keys() {
            if matches_pattern(pattern, text, i) {
                let token_type = TOKENS_MAP.get(pattern).copied().unwrap();
                join_and_push_unknown(&mut tokens, &unknown_chars);
                tokens.push((String::from(*pattern), token_type));
                i += pattern.len();
                unknown_chars.clear();
                continue 'outer;
            }
        }
        unknown_chars.push(text.chars().nth(i).unwrap());

        i += 1
    }
    join_and_push_unknown(&mut tokens, &unknown_chars);

    if debug_print {
        println!("\n________LEXER RESULTS________\n");
        for (token, token_type) in &tokens {
            println!("<{:?}> \"{}\"", token_type, token)
        }
    }
    tokens
}

fn matches_pattern(pattern: &str, text: &str, pos: usize) -> bool {
    let mut text_chars = text.chars().skip(pos);
    let pattern_chars = pattern.chars();
    for p in pattern_chars {
        let c = text_chars.next();
        if Some(p) != c {
            return false;
        }
    }
    true
}

fn join_and_push_unknown(vec: &mut Vec<(String, TokenType)>, item: &[char]) {
    let token: String = item.iter().collect();
    let token_trimmed = token.trim();
    if !token_trimmed.is_empty() {
        vec.push((token, TokenType::Unknown));
    }
}
