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
        prefered_required_room: (0, r),
        num_periods_per_week: 7i32,
    };

    let s = Student::Student {
        name: "unlazy world",
        id: "1230097438",
        gradenum: 120i32,
        element: "Ne",
        mandatory_courses: vec![c.clone()],
        ranked_mandatory_courses: vec![vec![c.clone()]],
        ranked_electives: vec![c.clone()],
    };

    println!("{}", s.name);
}
