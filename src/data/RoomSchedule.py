import numpy as np
import Schedule
from src.Constants import DAYS_IN_WEEK, NUMBER_OF_PERIODS

class RoomSchedule(Schedule):

  def __init__(self, room):
    Schedule.__init__(self, NUMBER_OF_PERIODS + 1)
    self.room = room

  def getRoom(self):
    return self.room