from RoomSchedule import RoomSchedule
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
        self.roomSchedules = np.append(self.roomSchedules, roomSchedule)
    
    def getNumberOfRooms(self):
        return self.roomSchedules.size()
    
    def getIndexesOfRoomsWithPeriodUnoccuppied(self, period):
        rooms_with_period_unoccupied = np.array([])
        for i in range(self.roomSchedules.size):
            if self.roomSchedules[i].sections[0][period] == None:
                rooms_with_period_unoccupied = np.append(rooms_with_period_unoccupied, i)
        return rooms_with_period_unoccupied