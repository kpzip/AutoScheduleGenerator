'''
Constructor Syntax:

id
string, the room number or name

room_type
string, describes the type of room
room types: normal, chemstry lab, physics lab, biology lab, band, outside, multipurpose
'''


class Room:
    
    def __init__(self, name, room_type):
        self.name = name
        self.room_type = room_type
        
    def getId(self):
        return self.name

    def setId(self, name):
        self.name = name

    def getRoomType(self):
        return self.room_type

    def setRoomType(self, room_type):
        self.type = room_type
