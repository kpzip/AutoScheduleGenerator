'''
Constructor Syntax
name
string, teacher's full name first, middle, last

id
string, teacher id, unique for each teacher

ranked_courses
nparray of strings, includes all the courses the teacher can teach in order of which courses they prefer to teach

periods_available
nparray of ints, includes all periods the teacher is available to teach

students_ranked
dictionary, student : weight, weight ranges from -1 -> +1, where negative means the teacher doesn't want the student in their class and +1 means they do
'''

import numpy as np

class Teacher:
  def __init__(self, name, id, ranked_courses, periods_available, students_ranked):
    self.name = name
    self.id = id
    self.ranked_courses = ranked_courses
    self.periods_available = periods_available
    self.students_ranked = students_ranked

  def getName(self): 
    return self.name

  def setName(self, name): 
    self.name = name

  def getId(self):
    return self.id

  def setId(self, id):
    self.id = id

  def getRankedCourses(self):
    return self.ranked_courses

  def setRankedCourses(self, ranked_courses):
    self.ranked_courses = ranked_courses

  def getPeriodsAvailable(self):
    return self.periods_available

  def setPeriodsAvailable(self, periods_available):
    self.periods_available = periods_available

  def getStudentsRanked(self):
    return self.students_ranked

  def setStudentsRanked(self, students_ranked):
    self.students_ranked = students_ranked