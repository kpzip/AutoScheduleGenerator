import numpy as np
import Schedule
from Constants import DAYS_IN_WEEK, NUMBER_OF_PERIODS

class StudentSchedule(Schedule):

  def __init__(self, student):
    Schedule.__init__(self, NUMBER_OF_PERIODS + 1 if student.getTakesZeroPeriod() else NUMBER_OF_PERIODS)
    self.student = student

  def getStudent(self):
    return self.student