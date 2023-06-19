class Student:

	"""
	name: string, student's full name First, middle, last
	id: string, student's id, should be unique
	gradenum: integer, student's grade level
	element: string, name of student's element
	mandatory_courses: list of Course, courses the student must take
	ranked_mandatory_courses: list of list of Course, ranked choices for courses where one is mandatory, but students may not get their prefered choice
	ranked_electives: list of Course, the electives a student wishes to take

	"""
	def __init__(self, name, id, gradenum, element, mandatory_courses, ranked_mandatory_courses, ranked_electives):
		self.name = name
		self.id = id
		self.gradenum = gradenum
		self.element = element
		self.mandatory_courses = mandatory_courses
		self.ranked_mandatory_courses = ranked_mandatory_courses
		self.ranked_electives = ranked_electives
