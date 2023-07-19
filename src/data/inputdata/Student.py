'''
Constructor Syntax:

name
string, student's full name girst, middle, last

id
string, student's id, should be unique

gradenum
integer, student's grade level

element
string, name of student's element

mandatory_courses
nparray of courses, courses the student must take

ranked_mandatory_courses
nparray of nparrays of courses, ranked choices for courses where one is mandatory, but students may not get their prefered choice

ranked_electives
list of Courses, the electives a student wishes to take
'''

import numpy as np

class Student:
    
  def __init__(self, name, id, gradenum, element, mandatory_courses, ranked_mandatory_courses, ranked_electives, takesZeroPeriod):
    self.name = name
    self.id = id
    self.gradenum = gradenum
    self.element = element
    self.mandatory_courses = mandatory_courses
    self.ranked_mandatory_courses = ranked_mandatory_courses
    self.ranked_electives = ranked_electives
    self.takesZeroPeriod = takesZeroPeriod

  def getName(self):
    return self.name

  def setName(self, name):
    self.name = name

  def getId(self):
    return self.id

  def setId(self, id):
    self.id = id

  def getGradeNum(self):
    return self.gradenum

  def setgradeNum(self, gradenum):
    self.gradenum = gradenum

  def getElement(self):
    return self.element

  def setElement(self, element):
    self.element = element

  def getMandatoryCourses(self):
    return self.mandatory_courses

  def setMandatoryCourses(self, mandatory_courses):
    self.mandatory_courses = mandatory_courses

  def getRankedMandatoryCourses(self):
    return self.ranked_mandatory_courses

  def setRankedMandatoryCourses(self, ranked_mandatory_courses):
    self.ranked_mandatory_courses = ranked_mandatory_courses

  def getRankedElectives(self):
    return ranked_electives

  def setRankedElectives(self):
    self.ranked_electives = ranked_electives

  def getTakesZeroPeriod(self):
    return self.takesZeroPeriod

  def setTakesZeroPeriod(self, takesZeroPeriod):
    self.takesZeroPeriod = takesZeroPeriod
