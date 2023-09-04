/*
Variables:

id
string, the room number or name

room_type
string, describes the type of room
room types: normal, chemstry lab, physics lab, biology lab, band, outside, multipurpose
*/

pub struct Room<'a> {
    id: &'a str,
    room_type: &'a str,
}
