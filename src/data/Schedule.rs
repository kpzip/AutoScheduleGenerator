use super::Section::*;
use std::default::Default;

//not including zero period
pub const NUM_PERIODS_IN_DAY: usize = 8;

pub enum ScheduleType {
    Student,
    Teacher,
    Room,
}

trait Schedule<'a> {
    fn get_section(&self, period: usize) -> &Section;

    fn set_section(&mut self, period: usize, section: &'a Section);

    fn get_type() -> ScheduleType;
}

trait DynamicSchedule<'a>: Schedule<'a> {
    fn add_section(&mut self, section: &'a Section);

    fn get_first_period(&self) -> u8;

    fn get_last_period(&self) -> u8;
}

pub struct StudentSchedule<'a> {
    has_zero_period: bool,
    sections: [&'a Section<'a>; NUM_PERIODS_IN_DAY + 1],
}

impl<'a> Schedule<'a> for StudentSchedule<'a> {
    fn get_section(&self, period: usize) -> &Section {
        assert!(self.has_zero_period || period > 0);
        self.sections[period]
    }

    fn set_section(&mut self, period: usize, section: &'a Section) {
        assert!(self.has_zero_period || period > 0);
        self.sections[period] = section;
    }

    fn get_type() -> ScheduleType {
        ScheduleType::Student
    }
}

impl<'a> Default for StudentSchedule<'a> {
    fn default() -> StudentSchedule<'a> {
        StudentSchedule {
            has_zero_period: true,
            sections: [&EMPTY_SECTION; NUM_PERIODS_IN_DAY + 1],
        }
    }
}

pub struct TeacherSchedule<'a> {
    first_teaching_period: u8,
    sections: Vec<&'a Section<'a>>,
}

impl TeacherSchedule<'_> {
    fn get_index(&self, period: usize) -> usize {
        period + self.first_teaching_period as usize
    }

    fn expand(&mut self, amount: usize, end: bool) {
        if end {
            self.sections.append(&mut vec![&EMPTY_SECTION; amount])
        } else {
            assert!(self.first_teaching_period - amount as u8 >= 0);
            self.sections.reverse();
            self.expand(amount, true);
            self.sections.reverse();
            self.first_teaching_period -= amount as u8;
        }
    }
}

impl<'a> Schedule<'a> for TeacherSchedule<'a> {
    fn get_section(&self, period: usize) -> &Section {
        match self.sections.get(self.get_index(period)) {
            Some(sec) => sec,
            None => &EMPTY_SECTION,
        }
    }

    fn set_section(&mut self, period: usize, section: &'a Section) {
        assert!(period <= NUM_PERIODS_IN_DAY);
        if period as u8 > self.get_last_period() {
            self.expand(period - self.get_last_period() as usize, true);
        } else if (period as u8) < self.get_first_period() {
            self.expand(self.first_teaching_period as usize - period, false);
        }
        let index = self.get_index(period);
        self.sections.remove(index);
        self.sections.insert(index, section);
    }

    fn get_type() -> ScheduleType {
        ScheduleType::Teacher
    }
}

impl<'a> DynamicSchedule<'a> for TeacherSchedule<'a> {
    fn add_section(&mut self, section: &'a Section) {
        assert!(self.get_last_period() < NUM_PERIODS_IN_DAY as u8);
        self.sections.push(section);
    }

    fn get_last_period(&self) -> u8 {
        self.first_teaching_period + self.sections.len() as u8
    }

    fn get_first_period(&self) -> u8 {
        self.first_teaching_period
    }
}

impl<'a> Default for TeacherSchedule<'a> {
    fn default() -> TeacherSchedule<'a> {
        TeacherSchedule {
            sections: vec![],
            first_teaching_period: 1,
        }
    }
}

struct RoomSchedule<'a> {
    sections: [&'a Section<'a>; NUM_PERIODS_IN_DAY + 1],
}

impl<'a> Schedule<'a> for RoomSchedule<'a> {
    fn get_section(&self, period: usize) -> &Section {
        self.sections[period]
    }

    fn set_section(&mut self, period: usize, section: &'a Section) {
        self.sections[period] = section;
    }

    fn get_type() -> ScheduleType {
        ScheduleType::Room
    }
}

impl<'a> Default for RoomSchedule<'a> {
    fn default() -> RoomSchedule<'a> {
        RoomSchedule {
            sections: [&EMPTY_SECTION; NUM_PERIODS_IN_DAY + 1],
        }
    }
}

#[derive(Default)]
struct MasterSchedule<'a, T: Schedule<'a>> {
    schedules: Vec<&'a T>,
}
