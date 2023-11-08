use super::Course::*;
use super::Student::Student;
use super::Teacher::{Teacher, NOT_A_TEACHER};

pub const EMPTY_SECTION: Section = Section {
    is_empty: true,
    day: 0,
    period: 0,
    course: &NO_COURSE,
    students: Vec::new(),
    teacher: &NOT_A_TEACHER,
};

#[derive(Debug, Clone)]
pub struct Section<'a> {
    is_empty: bool,
    day: u8,
    period: u8,
    course: &'a Course<'a>,
    students: Vec<&'a Student<'a>>,
    teacher: &'a Teacher<'a>,
}
