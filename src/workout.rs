use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workout {
    pub name: String,
    pub weight: f32, // in lbs
    pub reps: i32, // number of repetitions
}

impl Workout {
    pub fn new(name: &str, weight: f32, reps: i32) -> Workout {
        Workout {
            name: String::from(name),
            weight,
            reps,
        }
    }
}