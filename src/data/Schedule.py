import numpy as np
import inputdata.Course
from abc import ABC, abstractmethod
from Constants import DAYS_IN_WEEK, NUMBER_OF_PERIODS
import Section

class Schedule(ABC):

  def __init__(self, numberOfPeriods):
    self.sections = np.empty((DAYS_IN_WEEK, numberOfPeriods), dtype = Section)

  def getScheduledSection(self, day_of_week, period):
    return self.sections[day_of_week][period]