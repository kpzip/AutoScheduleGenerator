use super::Course::*;
use super::Preference::*;
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

pub struct Teacher<'a> {
    name: &'a str,
    id: &'a str,
    ranked_courses: Vec<Course<'a>>,
    periods_available: Vec<i32>, // students_ranked :
}

impl Preference for Teacher<'_> {
    fn evaluate(&self) -> f64 {
        let mut score: f64 = 0f64;

        for course in self.ranked_courses {
            // need teacher schedule to be done
        }

        score
    }
}
