import numpy as np

class Section:

  def __init__(self, course, sectionnum, teacher, students, room):
    self.course = course
    self.sectionnum = sectionnum
    self.teacher = teacher
    self.students = students
    self.room = room
  
  def getCourse(self):
    return self.course
  
  def setCourse(self, course):
    self.course = course
  
  def getSectionNum(self):
    return self.sectionnum
  
  def setSectionNum(self, sectionnum):
    self.sectionnum = sectionnum
  
  def getTeacher(self):
    return self.teacher
  
  def setTeacher(self, teacher):
    self.teacher = teacher
  
  def getStudents(self):
    return self.students
  
  def setStudents(self, students):
    self.students = students
  
  def getRoom(self):
    return self.room
  
  def setRoom(self, room):
    self.room = room