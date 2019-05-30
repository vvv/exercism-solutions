extern crate num_integer;
use num_integer::Integer;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        const MINUTES_IN_DAY: i32 = 24 * 60;

        let mut mins = (hours * 60 + minutes) % MINUTES_IN_DAY;
        if mins < 0 {
            mins += MINUTES_IN_DAY;
        }
        let (hours, minutes) = mins.div_rem(&60);
        Clock {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
