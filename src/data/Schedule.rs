mod data::Section;
use std::default::Default;

pub static NUM_PERIODS_IN_DAY: u32 = 8;

trait Schedule {
    fn get_section(&self, period: i32) -> Section::Section;
}

pub struct StudentSchedule {
    has_zero_period: bool,
    sections: [&Section::Section, &NUM_PERIODS_IN_DAY + has_zero_period as i32],
}

impl Schedule for StudentSchedule {
    fn get_section(&self, period: i32) -> Section::Section {
        self.sections[period]
    }
}

impl Default for StudentSchedule {
    fn default() -> StudentSchedule {
        StudentSchedule {
            has_zero_period: true,
            sections: [&Section::EMPTY_SECTION; &NUM_PERIODS_IN_DAY + 1],
        }
    }
}
