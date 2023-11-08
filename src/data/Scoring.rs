use super::Schedule::MasterSchedule;
use super::Schedule::RoomSchedule;

pub trait Scoreable {
    fn score(&self) -> f64;
}
