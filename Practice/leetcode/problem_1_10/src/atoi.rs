#[derive(PartialEq)]
enum State {
    S0,
    S1,
    S2,
    SE,
}

struct StateMachine {
    state: State,
    result: i32,
    sign: i8,
}

impl StateMachine {
    fn to_state_s1(&mut self, ch: char) {
        self.sign = if ch == '-' { -1 } else { 1 };
        self.state = State::S1;
    }

    fn to_state_s2(&mut self, digit: usize) {
        self.state = State::S2;
        self.append_digit(digit)
    }

    fn to_state_se(&mut self) {
        self.state = State::SE;
    }

    fn append_digit(&mut self, digit: usize) {
        if self.result > i32::MAX / 10
            || (self.result == i32::MAX / 10 && digit > (i32::MAX % 10) as usize)
        {
            if self.sign == 1 {
                self.result = i32::MAX;
            } else {
                self.result = i32::MIN;
                self.sign = 1
            }
            self.to_state_se();
        } else {
            self.result = self.result * 10 + digit as i32;
        }
    }

    pub fn transition(&mut self, ch: char) {
        if self.state == State::S0 {
            if ch == ' ' {
                return;
            } else if ch == '-' || ch == '+' {
                self.to_state_s1(ch);
            } else if ch.is_digit(10) {
                self.to_state_s2(ch.to_digit(10).unwrap() as usize);
            } else {
                self.to_state_se();
            }
        } else if self.state == State::S1 || self.state == State::S2 {
            if ch.is_digit(10) {
                self.to_state_s2(ch.to_digit(10).unwrap() as usize)
            } else {
                self.to_state_se();
            }
        }
    }

    pub fn get_final_number(self) -> i32 {
        return self.sign as i32 * self.result;
    }
}

pub fn my_atoi(s: String) -> i32 {
    let mut state_machine = StateMachine {
        state: State::S0,
        result: 0,
        sign: 1,
    };

    for ch in s.chars() {
        state_machine.transition(ch);
        if state_machine.state == State::SE {
            break;
        }
    }

    return state_machine.get_final_number();
}
