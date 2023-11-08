use super::Course::*;

pub const EMPTY_SECTION: Section = Section {
    is_empty: true,
    day: 0,
    period: 0,
    course: &NO_COURSE,
};

pub struct Section<'a> {
    is_empty: bool,
    day: u8,
    period: u8,
    course: &'a Course<'a>,
}
