mod to_do;

use clap::{Arg, Command};

use to_do::enums::TaskStatus;

use to_do::structs::done::Done;

fn main() {
    let app = Command::new("booking")
        .version("1.0")
        .about("Books in a user")
        .author("Maxwell Flitton");

    let first_name = Arg::new("first name")
        .long("f")
        .takes_value(true)
        .help("first name of user")
        .required(true);

    let last_name = Arg::new("last name")
        .long("l")
        .takes_value(true)
        .help("first name of user")
        .required(true);
    let age = Arg::new("age")
        .long("a")
        .takes_value(true)
        .help("age of the user")
        .required(true);

    let app = app.arg(first_name).arg(last_name).arg(age);
    let matches = app.get_matches();
    let name = matches
        .value_of("first name")
        .expect("First name is required");
    let surname = matches.value_of("last name").expect("Surname is required");
    let age: i8 = matches
        .value_of("age")
        .expect("Age is required")
        .parse()
        .unwrap();
    println!("{:?}", name);
    println!("{:?}", surname);
    println!("{:?}", age);

    println!("{}", TaskStatus::DONE);
    println!("{}", TaskStatus::PENDING);
    let outcome = TaskStatus::DONE.to_string();
    println!("{}", outcome);

    let a = "dassdada";

    let doneTask = Done::new("washing");
    println!("{}", doneTask.super_struct.title);
    println!("{}", doneTask.super_struct.status);
}
