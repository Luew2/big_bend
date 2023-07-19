use chrono::NaiveDate; // Chrono for data handling.

pub struct Progress {
    pub date: NaiveDate, //the date the workout was done
    pub workout: Workout,
}