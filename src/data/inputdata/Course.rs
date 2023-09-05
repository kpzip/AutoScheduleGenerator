use crate::data::inputdata::Room;

pub mod Room;
/*
Variables:

name
string, course's name

req_one_per_element
bool, whether or not a section of this course should have one section for every element that takes it

prefered_required_room

tuple,
first: 0, 1, 2; (0 means no preference, 1 means prefered, 2 means required)
second: room that is required or prefered

num_periods_per_week
int, the number of periods this course has per week
*/

trait preference {
    fn evaluate(&self) -> i32
}

pub struct Course<'a> {
    name: &'a str,
    req_one_per_element: bool,
    prefered_required_room: (i32, Room::Room),
    num_periods_per_week: i32,
}

impl preference for Course {
    fn evaluate (&self) -> i32 {
        let score : mut i32 = 0;

        if self.prefered_required_room.0 = 1 {
            // need room schedule to be done
        }
    }
}