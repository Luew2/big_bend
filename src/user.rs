use crate::workout::Workout;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub workouts: Vec<Workout>, // A vector of workouts
    pub average_calories: f32, // Average calories consumed per day
}

impl Default for User {
    fn default() -> Self {
        User {
            name: String::from("Default"),
            workouts: Vec::new(),
            average_calories: 0.0,
        }
    }
}

impl User {
    pub fn add_workout(&mut self, workout: Workout) {
        self.workouts.push(workout);
    }

    pub fn view_progress(&self) {
        for workout in &self.workouts {
            println!("Workout: {}, Weight: {}, Reps: {}", workout.name, workout.weight, workout.reps);
        }
    }
}
