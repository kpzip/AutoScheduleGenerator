from Schedule import Schedule
import Constants as Constants


class TeacherSchedule(Schedule):

    def __init__(self, teacher):
        super.__init__(self, Constants.NUMBER_OF_PERIODS + 1)
        self.teacher = teacher
    
    def getTeacher(self):
        return self.teacher
