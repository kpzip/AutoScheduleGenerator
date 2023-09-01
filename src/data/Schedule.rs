use crate::data::Section::*;
use std::default::Default;

pub const NUM_PERIODS_IN_DAY: usize = 8;

trait Schedule {
    fn get_section(&self, period: usize) -> &Section;
}

pub struct StudentSchedule<'a> {
    has_zero_period: bool,
    sections: [&'a Section; NUM_PERIODS_IN_DAY + 1],
}

impl Schedule for StudentSchedule<'_> {
    fn get_section(&self, period: usize) -> &Section {
        self.sections[period]
    }
}

impl Default for StudentSchedule<'_> {
    fn default() -> StudentSchedule {
        StudentSchedule {
            has_zero_period: true,
            sections: [&EMPTY_SECTION; NUM_PERIODS_IN_DAY + 1],
        }
    }
}
