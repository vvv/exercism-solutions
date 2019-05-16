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
        let mut mins = (hours * 60 + minutes) % (24 * 60);
        if mins < 0 {
            mins += 24 * 60;
        }
        Clock {
            hours: mins / 60,
            minutes: mins % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
