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
use super::Room::*;
use std::clone::Clone;
use std::default::Default;

pub const NO_COURSE: Course = Course::default();

#[derive(Clone)]
pub struct Course<'a> {
    pub name: &'a str,
    pub req_one_per_element: bool,
    pub prefered_required_room: Option<&'a Room<'a>>,
    pub num_periods_per_week: u8,
}

impl<'a> Default for Course<'a> {
    fn default() -> Course<'a> {
        Course {
            name: "Unknown Course",
            req_one_per_element: false,
            prefered_required_room: None,
            num_periods_per_week: 5,
        }
    }
}
