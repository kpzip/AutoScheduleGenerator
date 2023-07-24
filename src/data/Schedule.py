from abc import ABC

from src.data.Section import Section
import numpy as np
import src.data.Constants as Constants


class Schedule(ABC):
    
    def __init__(self, numberOfPeriods):
        self.sections = np.empty((Constants.DAYS_IN_WEEK, numberOfPeriods), dtype=Section)
    
    def getScheduledSection(self, day_of_week, period):
        return self.sections[day_of_week][period]
