'''
Constructor Syntax:

id
string, the room number or name

room_type
string, describes the type of room
room types: normal, chemstry lab, physics lab, biology lab, band, outside, multipurpose
'''
import numpy as np

class Room:
    
  def __init__(self, id, room_type):
    self.id = id
    self.room_type = room_type
    
  def getId(self):
    return self.id

  def setId(self):
    self.id = id

  def getRoomType(self):
    return self.room_type

  def setRoomType(self, room_type):
    self.type = room_type