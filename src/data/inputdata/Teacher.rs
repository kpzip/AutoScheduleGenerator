use crate::data::inputdata::Course;

pub mod Course;
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

trait preference {
    fn evaluate(&self) -> i32
}

pub struct Teacher<'a> {
    name: &'a str,
    id: &'a str,
    ranked_courses: Vec<Course::Course>,
    periods_available: Vec<i32>, // students_ranked :
}

impl preference for Teacher {
    fn evaluate (&self) -> i32 {
        let score : mut i32 = 0;

        for course in self.ranked_courses {
            // need teacher schedule to be done
        }
    }
}

