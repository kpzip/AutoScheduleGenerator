trait Schedule {
    fn get_section(&self) {}
}

struct StudentSchedule {}

impl Schedule for StudentSchedule {}
