/*
Variables:

id
string, the room number or name

room_type
string, describes the type of room
room types: normal, chemstry lab, physics lab, biology lab, band, outside, multipurpose
*/
use std::clone::Clone;
use std::default::Default;

pub const NOT_A_ROOM: Room = Default::default();

#[derive(Clone)]
pub struct Room<'a> {
    pub id: &'a str,
    pub room_type: &'a str,
}

impl<'a> Default for Room<'a> {
    fn default() -> Room<'a> {
        Room {
            id: "",
            room_type: "",
        }
    }
}
