'''
Constructor Syntax

name
string, course's name

req_one_per_element
bool, whether or not a section of this course should have one section for every element that takes it

prefered_required_room

tuple,
first: 0, 1, 2; (0 means no preference, 1 means prefered, 2 means required)
second: room that is required or prefered

num_periods_per_week
int, the number of periods this course has per week
'''


class Course:
    
    def __init__(self, name, req_one_per_element, prefered_required_room, num_periods_per_week):
        self.name = name
        self.req_one_per_element = req_one_per_element
        self.prefered_required_room = prefered_required_room
        self.num_periods_per_week = num_periods_per_week

    def getName(self):
        return self.name
    
    def setName(self, name):
        self.name = name

    def getReqOnePerElement(self):
        return self.req_one_per_element

    def setReqOnePerElement(self, req_one_per_element):
        self.req_one_per_element = req_one_per_element

    def getPreferedRequiredRoom(self):
        return self.prefered_required_room

    def setPreferedRequiredRoom(self, prefered_required_room):
        self.prefered_required_room = prefered_required_room

    def getNumPeriodsPerWeek(self):
        return self.num_periods_per_week

    def setNumPeriodsPerWeek(self, num_periods_per_week):
        self.num_periods_per_week = num_periods_per_week
