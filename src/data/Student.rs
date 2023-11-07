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
use std::thread;

static mut ID_COUNTER: u64 = 0;

#[derive(Clone)]
pub struct Student<'a> {
    name: String,
    id: u64,
    gradenum: u8,
    element: &'a str,
    mandatory_courses: Vec<&'a Course<'a>>,
    ranked_mandatory_courses: Vec<Vec<&'a Course<'a>>>,
    ranked_electives: Vec<&'a Course<'a>>,
}

impl Student<'_> {
    //only call on the main thread!!!!
    pub fn new(name: String, grade: u8, element: &'static str) -> Student {
        //really bad but it works I guess
        assert_eq!(thread::current().name().unwrap(), "main");
        let current_id;
        unsafe {
            current_id = ID_COUNTER;
            ID_COUNTER = ID_COUNTER + 1;
        }
        Student {
            name,
            id: current_id,
            gradenum: grade,
            element,
            ..Default::default()
        }
    }
}

impl<'a> Default for Student<'a> {
    fn default() -> Student<'a> {
        Student {
            name: String::from("John Doe"),
            id: u64::MAX,
            gradenum: 0, //imagine being in grade 0... idiot
            element: "",
            mandatory_courses: Vec::new(),
            ranked_mandatory_courses: Vec::new(),
            ranked_electives: Vec::new(),
        }
    }
}
