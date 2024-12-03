use utils::read_all;

#[derive(Clone, Copy, Debug)]
enum State {
  Start,
  M,
  U,
  L,
  MulLeftBracket,
  XValue { x: i32 },
  Comma { x: i32 },
  YValue { x: i32, y: i32 },
  End { x: i32, y: i32 },
  D { ignore: bool },
  O { ignore: bool },
  N,
  SingleQuote,
  T,
  DoLeftBracket { ignore: bool },
  DontLeftBracket,
  Ignore,
}

impl State {
  fn next_state1(&self, c: u8) -> Self {
    match *self {
      State::Start => match c {
        b'm' => State::M,
        _ => State::Start
      },
      State::M => match c {
        b'u' => State::U,
        b'm' => State::M,
        _ => State::Start,
      },
      State::U => match c {
        b'l' => State::L,
        b'm' => State::M,
        _ => State::Start
      },
      State::L => match c {
        b'(' => State::MulLeftBracket,
        b'm' => State::M,
        _ => State::Start,
      },
      State::MulLeftBracket => match c {
        b'0'..=b'9' => State::XValue { x: (c - b'0') as i32 },
        b'm' => State::M,
        _ => State::Start,
      },
      State::XValue { x } => match c {
        b'0'..=b'9' => State::XValue { x: x * 10 + (c - b'0') as i32 },
        b',' => State::Comma { x },
        b'm' => State::M,
        _ => State::Start,
      },
      State::Comma { x } => match c {
        b'0'..=b'9' => State::YValue { x, y: (c - b'0') as i32 },
        b'm' => State::M,
        _ => State::Start,
      },
      State::YValue { x, y } => match c {
        b'0'..=b'9' => State::YValue { x, y: y * 10 + (c - b'0') as i32 },
        b')' => State::End { x, y },
        b'm' => State::M,
        _ => State::Start,
      },
      State::End { x: _, y: _ } => *self,
      _ => panic!("Invalid state"),
    }
  }

  fn next_state2(&self, c: u8) -> Self {
    match *self {
      State::Start => match c {
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start
      },
      State::M => match c {
        b'u' => State::U,
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start,
      },
      State::U => match c {
        b'l' => State::L,
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start
      },
      State::L => match c {
        b'(' => State::MulLeftBracket,
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start,
      },
      State::MulLeftBracket => match c {
        b'0'..=b'9' => State::XValue { x: (c - b'0') as i32 },
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start,
      },
      State::XValue { x } => match c {
        b'0'..=b'9' => State::XValue { x: x * 10 + (c - b'0') as i32 },
        b',' => State::Comma { x },
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start,
      },
      State::Comma { x } => match c {
        b'0'..=b'9' => State::YValue { x, y: (c - b'0') as i32 },
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start,
      },
      State::YValue { x, y } => match c {
        b'0'..=b'9' => State::YValue { x, y: y * 10 + (c - b'0') as i32 },
        b')' => State::End { x, y },
        b'm' => State::M,
        b'd' => State::D { ignore: false },
        _ => State::Start,
      },
      State::End { x: _, y: _ } => *self,
      State::D { ignore } => match c {
        b'o' => State::O { ignore },
        b'd' => State::D { ignore },
        _ if ignore => State::Ignore,
        b'm' => State::M,
        _ => State::Start,
      },
      State::O { ignore } => match c {
        b'(' => State::DoLeftBracket { ignore },
        b'd' => State::D { ignore },
        _ if ignore => State::Ignore,
        b'n' => State::N,
        b'm' => State::M,
        _ => State::Start,
      },
      State::N => match c {
        b'\'' => State::SingleQuote,
        b'd' => State::D { ignore: false },
        b'm' => State::M,
        _ => State::Start,
      },
      State::SingleQuote => match c {
        b't' => State::T,
        b'd' => State::D { ignore: false },
        b'm' => State::M,
        _ => State::Start,
      },
      State::T => match c {
        b'(' => State::DontLeftBracket,
        b'd' => State::D { ignore: false },
        b'm' => State::M,
        _ => State::Start,
      },
      State::DoLeftBracket { ignore } => match c {
        b')' => State::Start,
        b'd' => State::D { ignore },
        _ if ignore => State::Ignore,
        b'm' => State::M,
        _ => State::Start,
      }
      State::DontLeftBracket => match c {
        b')' => State::Ignore,
        b'd' => State::D { ignore: false },
        b'm' => State::M,
        _ => State::Start,
      },
      State::Ignore => match c {
        b'd' => State::D { ignore: true },
        _ => State::Ignore,
      }
    }
  }
}

fn eval(string: &str) -> i32 {
  let mut state = State::Start;
  let mut res = 0;

  for &c in string.as_bytes() {
    state = state.next_state1(c);
    
    if let State::End { x, y } = state {
      res += x * y;
      state = State::Start;
    }
  }

  return res;
}

fn eval_cond(string: &str) -> i32 {
  let mut state = State::Start;
  let mut res = 0;

  for &c in string.as_bytes() {
    state = state.next_state2(c);
    
    if let State::End { x, y } = state {
      res += x * y;
      state = State::Start;
    }
  }

  return res;
}

fn main() {
  let input = read_all();
  
  let value1 = eval(&input);
  let value2 = eval_cond(&input);

  println!("Result (part 1): {value1}");
  println!("Result (part 2): {value2}");
}
