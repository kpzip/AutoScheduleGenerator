from src.data.RoomSchedule import RoomSchedule
import numpy as np


class MasterSchedule:
    
    def __init__(self):
        self.roomSchedules = np.empty(0, dtype=RoomSchedule)
    
    def getRoomSchedule(self, identifier):
        for s in self.roomSchedules:
            if s.getRoom().getId() == identifier:
                return s
        return None
    
    def addRoomSchedule(self, roomSchedule):
        self.roomSchedules.append(roomSchedule)
    
    def getNumberOfRooms(self):
        return self.roomSchedules.size()
