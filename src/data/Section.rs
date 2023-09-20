use super::Course::*;
use super::Room::*;
use super::Student::*;

pub const EMPTY_SECTION: Section = Section { is_empty: true };

pub struct Section {
    pub is_empty: bool,
    pub day: i32,
    pub period: i32,
    pub room: Room,
    pub course: Course,
    pub student_list: Vec<Student>,
}
