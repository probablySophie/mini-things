#![allow(dead_code)]

/*
  Work  ▏ 25:00 ▕  ☺
 Break  ▏  5:00 ▕
Repeat  ▏  × 3  ▕
 Break  ▏ 20:00 ▕
*/


// Times in seconds
pub const TIME_WORK:        u16 = 1500;
pub const TIME_SHORT_BREAK: u16 = 300;
pub const TIME_LONG_BREAK:  u16 = 1200;

const DEFAULT_REPEAT_NUM:   u16 = 3;


pub enum PomodoroState
{
	Work,
	ShortBreak,
	LongBreak,
}

pub struct Time
{
	work: u16,
	short_break: u16,
	long_break: u16,
}
impl Default for Time
{
	fn default() -> Self
	{
		Time
		{
			work: TIME_WORK,
			short_break: TIME_SHORT_BREAK,
			long_break: TIME_LONG_BREAK,
		}
	}
}

pub struct Pomodoro
{
	state: PomodoroState,
	timer: u16,
	times: Time,
	repeats: u16,
	base_repeats: u16,
	
}
impl Default for Pomodoro
{
	fn default() -> Self
	{
		Pomodoro
		{
			state: PomodoroState::Work,
			timer: TIME_WORK,
			times: Time::default(),
			repeats: DEFAULT_REPEAT_NUM,
			base_repeats: DEFAULT_REPEAT_NUM,
		}
	}
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
