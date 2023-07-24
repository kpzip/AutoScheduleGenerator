import numpy as np
from RoomSchedule import RoomSchedule

class MasterSchedule:

  def __init__(self):
    self.roomSchedules = np.empty(0, dtype = RoomSchedule)

  def getRoomSchedule(self, id):
    for s in self.roomSchedules:
      if s.getRoom().getId() == id:
        return s
    return None

  def addRoomSchedule(self, roomSchedule):
    self.roomSchedules.append(roomSchedule)

  def getNumberOfRooms(self):
    return self.roomSchedules.size()

  