mod data;
use crate::data::Schedule;

fn main() {
    let c = Course::Course {
        name: "math",
        req_one_per_element: false,
        prefered_required_room: (0, None),
        num_periods_per_week: 7i32,
    };

    let s = Student::Student {
        name: "unlazy world",
        id: "1230097438",
        gradenum: 120i32,
        element: "Ne",
        mandatory_courses: vec![c],
        ranked_mandatory_courses: vec![vec![c; 1]; 1],
        ranked_electives: vec![c],
    };

    println!("{}", s.name);
}
