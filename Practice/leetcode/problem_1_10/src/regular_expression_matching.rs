use std::collections::VecDeque;

#[derive(PartialEq, Copy, Clone)]
enum State {
  CHAR,
  DOT,
  EOF,
  STAR,
  UNKNOWN,
}

struct Token {
  state: State,
  pattern: char,
}

fn lexer(regex_str: String) -> Vec<Token> {
  let mut tokens = Vec::new();
  for ch in regex_str.chars() {
    if 'a' <= ch && ch <= 'z' {
      tokens.push(Token { state: State::CHAR, pattern: ch })
    } else if ch == '.' {
      tokens.push(Token { state: State::DOT, pattern: '.' })
    } else if ch == '*' {
      tokens.push(Token { state: State::STAR, pattern: '*' })
    } else {
      tokens.push(Token { state: State::UNKNOWN, pattern: ch })
    }
  }

  return tokens;
}

fn parse(tokens: &Vec<Token>) -> Vec<Token> {
  let mut parsed = Vec::new();
  let mut index = 0usize;
  while index < tokens.len() {
    let eof = index + 1 == tokens.len();
    let lookahead = tokens.get(index).unwrap();
    if !eof && tokens.get(index + 1).unwrap().state == State::STAR {
      if lookahead.state == State::STAR {
        panic!("Found ** in input regex");
      }
      parsed.push(Token { state: State::STAR, pattern: lookahead.pattern });
      index += 1;
    } else {
      if lookahead.state == State::STAR {
        panic!("Found ^* in input regex");
      }
      parsed.push(Token { state: lookahead.state, pattern: lookahead.pattern });
    }
    index += 1;
  }
  parsed.push(Token { state: State::EOF, pattern: '$' });

  return parsed;
}

fn do_elementary_match(token: &Token, ch: char) -> bool {
  if token.state == State::CHAR {
    return token.pattern == ch;
  } else if token.state == State::DOT {
    return 'a' <= ch && ch <= 'z';
  } else if token.state == State::EOF {
    return ch == '$';
  } else if token.state == State::STAR {
    return false;
  }
  return false;
}

fn do_char_match(left: char, right: char) -> bool {
  if 'a' <= left && left <= 'z' {
    return left == right;
  } else if left == '.' {
    return 'a' <= right && right <= 'z';
  } else if left == '$' {
    return right == '$';
  } else if left == '*' {
    return false;
  }
  return false;
}

fn do_match(parsed: &Vec<Token>, str_to_match: String) -> bool {
  let input = str_to_match + &*'$'.to_string();
  let input_chars: Vec<char> = input.chars().collect();
  let mut input_index = 0usize;
  let mut token_index = 0usize;
  let mut backtrack;
  let mut queue = VecDeque::new();

  loop {
    backtrack = true;
    let end_of_input = input_index == input.len();
    let end_of_pattern = token_index == parsed.len();
    if end_of_input && end_of_pattern {
      return true;
    } else if end_of_input || end_of_pattern {
      backtrack = true;
    } else {
      let token = parsed.get(token_index).unwrap();
      let ch = input_chars.get(input_index).unwrap();
      if do_elementary_match(token, *ch) {
        input_index += 1;
        token_index += 1;
        backtrack = false;
      } else if token.state == State::STAR {
        backtrack = false;
        if do_char_match(token.pattern, *ch) {
          queue.push_back((input_index, token_index + 1));
          input_index += 1;
        } else {
          token_index += 1;
        }
      }
    }
    if backtrack {
      if queue.len() > 0 {
        let queue_element = queue.pop_back();
        input_index = queue_element.unwrap().0;
        token_index = queue_element.unwrap().1;
      } else {
        return false;
      }
    }
  }
}

pub fn is_match(regex_str: String, str_to_match: String) -> bool {
  if regex_str.len() == 0 {
    return str_to_match.len() == 0;
  }
  let tokens = lexer(regex_str);
  let parsed = parse(&tokens);

  return do_match(&parsed, str_to_match);
}
