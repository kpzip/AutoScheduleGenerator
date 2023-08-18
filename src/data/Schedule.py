from abc import ABC

from src.data.Section import Section
import numpy as np
import src.data.Constants as Constants

class Schedule(ABC):

    def __init__(self, numberOfPeriods):
        self.sections = np.empty((Constants.DAYS_IN_WEEK, numberOfPeriods), dtype=Section)

    def getScheduledSection(self, day_of_week, period):
        return self.sections[day_of_week][period]
  
    def getEmptySectionTimes(self):
        empty_list = np.array();
        for i in range(Constants.DAYS_IN_WEEK):
            for j in range(Constants.NUMBER_OF_PERIODS):
                if self.sections[i][j] == None:
                    empty_list.append((i + 1, j + 1))
        return empty_list
  
    def getOccupiedSectionTimes(self):
        occupied_list = np.array();
        for i in range(Constants.DAYS_IN_WEEK):
            for j in range(Constants.NUMBER_OF_PERIODS):
                if self.sections[i][j] != None:
                    occupied_list.append((i + 1, j + 1))
        return occupied_list
