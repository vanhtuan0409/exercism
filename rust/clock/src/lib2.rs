use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        if hours.abs() >= 24 {
            return Clock::new(hours % 24, minutes);
        }
        if minutes.abs() >= 60 {
            return Clock::new(hours + minutes / 60, minutes % 60);
        }
        if hours < 0 {
            return Clock::new(24 + hours, minutes);
        }
        if minutes < 0 {
            return Clock::new(hours - 1, 60 + minutes);
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
