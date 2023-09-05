use crate::data::inputdata::Course;

pub mod Course;
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

trait preference {
    fn evaluate(&self) -> i32
}

pub struct Student<'a> {
    name: &'a str,
    id: &'a str,
    gradenum: i32,
    element: &'a str,
    mandatory_courses: Vec<Course::Course>,
    ranked_mandatory_courses: Vec<Vec<Course::Course>>,
    ranked_electives: Vec<Course::Course>,
}

impl preference for Student {
    fn evaluate (&self) -> i32 {
        let score : mut i32 = 0;

        for elective in self.ranked_electives {
            // i need student schedule to be done
        }

        for c in self.ranked_mandatory_courses {
            for course in c {
                // i need student schedule to be done
            }
        }
    }
}
