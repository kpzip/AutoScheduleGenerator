from Schedule import Schedule
import Constants as Constants


class StudentSchedule(Schedule):
    
    def __init__(self, student):
        super.__init__(self, Constants.NUMBER_OF_PERIODS + 1 if student.getTakesZeroPeriod() else Constants.NUMBER_OF_PERIODS)
        self.student = student
    
    def getStudent(self):
        return self.student
