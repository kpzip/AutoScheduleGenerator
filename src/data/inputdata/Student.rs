use extern crate::data::inputdata::Course;
/*
Variables:

name
string, student's full name girst, middle, last

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

pub struct Student {
    name: &str,
    id: &str,
    gradenum: i32,
    element: &str,
    mandatory_courses: Vec<Course::Course>,
    ranked_mandatory_courses: Vec<Vec<Course::Course>>,
    ranked_electives: Vec<Course::Course>,
}
