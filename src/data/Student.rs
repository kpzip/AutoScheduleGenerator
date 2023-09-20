/*
Variables:

name
string, student's full name first, middle, last

id
string, student's id, should be unique

gradenum
integer, student's grade level

element
string, name of student's element

mandatory_courses
array of courses, courses the student must take

ranked_mandatory_courses
array of array of courses, ranked choices for courses where one is mandatory, but students may not get their prefered choice

ranked_electives
list of Courses, the electives a student wishes to take
*/

use super::Course::*;
use std::clone::Clone;
use std::default::Default;

#[derive(Clone)]
pub struct Student<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub gradenum: i32,
    pub element: &'a str,
    pub mandatory_courses: Vec<Course<'a>>,
    pub ranked_mandatory_courses: Vec<Vec<Course<'a>>>,
    pub ranked_electives: Vec<Course<'a>>,
}

impl<'a> Default for Student<'a> {
    fn default() -> Student<'a> {
        Student {
            name: "",
            id: "",
            gradenum: -1,
            element: "",
            mandatory_courses: Vec::new(),
            ranked_mandatory_courses: Vec::new(),
            ranked_electives: Vec::new(),
        }
    }
}
