/*
Variables:

name
string, teacher's full name first, middle, last

id
string, teacher id, unique for each teacher

ranked_courses
vector of strings, includes all the courses the teacher can teach in order of which courses they prefer to teach

periods_available
vector of ints, includes all periods the teacher is available to teach

students_ranked
map, student : weight, weight ranges from -1 -> +1, where negative means the teacher doesn't want the student in their class and +1 means they do
*/
use super::{Course::*, Schedule::NUM_PERIODS_IN_DAY};
use std::default::Default;

pub const NOT_A_TEACHER: Teacher = Default::default();

#[derive(Debug, Default)]
pub struct Teacher<'a> {
    pub name: String,
    pub id: u64,
    pub ranked_courses: Vec<Course<'a>>,
    pub periods_available: [bool; NUM_PERIODS_IN_DAY + 1], // students_ranked :
}
