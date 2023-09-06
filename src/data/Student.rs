mod Course;
mod Preference;
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

pub struct Student<'a> {
    name: &'a str,
    id: &'a str,
    gradenum: i32,
    element: &'a str,
    mandatory_courses: Vec<Course::Course>,
    ranked_mandatory_courses: Vec<Vec<Course::Course>>,
    ranked_electives: Vec<Course::Course>,
}

impl Preference::Preference for Student<'_> {
    fn evaluate(&self) -> f64 {
        let mut score: f64 = 0f64;

        for i in 0..self.ranked_electives.len() {
            // i need student schedule to be done
            /*
            chosen_elective = elective chosen in student schedule
            if chosen_elective.name == self.ranked_elective[i] {
                score += self.ranked_electives.len() - i
            }
            */
        }

        for c in self.ranked_mandatory_courses {
            for course in 0..c.len() {
                // i need student schedule to be done
                /*

                */
            }
        }

        score
    }
}
