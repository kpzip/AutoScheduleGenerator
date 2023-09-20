use super::Course::*;
use super::Room::*;
use super::Student::*;

pub const EMPTY_SECTION: Section = Section {
    is_empty: true,
    day: -1,
    period: -1,
    room: &Room::Default::default(),
    course: &Course::Default::default(),
    student_list: Vec::new(),
};

pub struct Section<'a> {
    pub is_empty: bool,
    pub day: i32,
    pub period: i32,
    pub room: &'a Room<'a>,
    pub course: &'a Course<'a>,
    pub student_list: Vec<&'a Student<'a>>,
}
