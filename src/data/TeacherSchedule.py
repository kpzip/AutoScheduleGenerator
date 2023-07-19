import numpy as np
import Schedule
from Constants import DAYS_IN_WEEK, NUMBER_OF_PERIODS

class TeacherSchedule(Schedule):

  def __init__(self, teacher):
    Schedule.__init__(self, NUMBER_OF_PERIODS + 1)
    self.teacher = teacher

  def getTeacher(self):
    return self.teacher