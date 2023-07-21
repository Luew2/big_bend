use clap::{App, Arg, SubCommand};
use crate::workout::Workout;
use crate::io::{read_user_from_file, write_user_to_file};
use crate::user::User;

pub fn run_cli() {
    let matches = App::new("Gym Progress Tracker")
    .version("1.0")
    .author("Lucas Ewing <lucasbewing@gmail.com>")
    .about("Tracking gym progress")
    .subcommand(
        SubCommand::with_name("add")
        .about("Add a new workout")
        .arg(
            Arg::with_name("USER")
                .help("Username of the user")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("TYPE")
                .help("Type of workout")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("WEIGHT")
                .help("The weight lifted")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("REPS")
                .help("Number of repetitions")
                .required(true)
                .index(4)
        ),
    )
    .subcommand(
        SubCommand::with_name("view")
            .about("View your progress")
            .arg(
                Arg::with_name("USER")
                    .help("Username of the user")
                    .required(true)
                    .index(1),),
    )
    .get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let username = add_matches.value_of("USER").unwrap();
            let filename = format!("{}.json", username);
            let mut user = match read_user_from_file(&filename) {
                Ok(user) => user,
                Err(error) => {
                    println!("An error has occured while reading user data: {}", error);
                    User::default()
                }
            };
            let workout_type = add_matches.value_of("TYPE").unwrap();
            let weight: f32 = add_matches.value_of("WEIGHT").unwrap().parse().unwrap();
            let reps: i32 = add_matches.value_of("REPS").unwrap().parse().unwrap();
            user.add_workout(Workout::new(workout_type, weight, reps));
            match write_user_to_file(&filename, &user) {
                Ok(_) => user.view_progress(),
                Err(error) => println!("An error occured while saving user data: {}", error),
            };
        },

        Some(("view", view_matches)) => {
            let username = view_matches.value_of("USER").unwrap();
            let filename = format!("{}.json", username);
            let user = match read_user_from_file(&filename) {
                Ok(user) => user,
                Err(error) => {
                    println!("An error occurred while reading user data: {}", error);
                    User::default()
                }
            };
            user.view_progress();
        }
        _ => (),
    }
}
