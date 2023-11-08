/*
Variables:

id
string, the room number or name

room_type
string, describes the type of room
room types: normal, chemstry lab, physics lab, biology lab, band, outside, multipurpose
*/
use std::default::Default;

pub const NOT_A_ROOM: Room = Default::default();

#[derive(Debug, Default)]
pub struct Room {
    pub id: String,
    pub room_type: String,
}
