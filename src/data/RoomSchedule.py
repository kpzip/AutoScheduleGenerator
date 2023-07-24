from src.data.Schedule import Schedule
import src.data.Constants as Constants


class RoomSchedule(Schedule):
    
    def __init__(self, room):
        super.__init__(self, Constants.NUMBER_OF_PERIODS + 1)
        self.room = room
    
    def getRoom(self):
        return self.room
