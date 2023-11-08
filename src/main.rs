mod data;
use crate::data::*;

fn main() {
    let r = Room::Room {
        id: "13242397884",
        room_type: "physics lab",
    };

    let c = Course::Course {
        name: "math",
        req_one_per_element: false,
        prefered_required_room: None,
        num_periods_per_week: 7,
    };

    let s = Student::Student {
        name: String::from("unlazy world"),
        id: 1230097438,
        gradenum: 120,
        element: "Ne",
        mandatory_courses: vec![&c],
        ranked_mandatory_courses: vec![vec![&c]],
        ranked_electives: vec![&c],
    };

    println!("{}", s.element);
}
