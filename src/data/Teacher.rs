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
use super::Course::*;
use std::clone::Clone;
use std::default::{self, Default};

pub const NOT_A_TEACHER: Teacher = Default::default();

#[derive(Clone)]
pub struct Teacher<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub ranked_courses: Vec<Course<'a>>,
    pub periods_available: Vec<i32>, // students_ranked :
}

impl<'a> Default for Teacher<'a> {
    fn default() -> Teacher<'a> {
        Teacher {
            name: "",
            id: "",
            ranked_courses: Vec::new(),
            periods_available: Vec::new(),
        }
    }
}
