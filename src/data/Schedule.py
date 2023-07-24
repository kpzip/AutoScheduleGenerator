import numpy as np
from abc import ABC
import src.data.Constants as Constants
from src.data.Section import Section


class Schedule(ABC):
    
    def __init__(self, numberOfPeriods):
        self.sections = np.empty((Constants.DAYS_IN_WEEK, numberOfPeriods), dtype=Section)
    
    def getScheduledSection(self, day_of_week, period):
        return self.sections[day_of_week][period]
