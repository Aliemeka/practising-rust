// CLI app that creates acts a class register and adds students a class
/*
  You can also find out if how many students is in the class, get the information about the student
  The students will have the following information, name, gender(M or F), time they join and age

  The app can be extended to get other information like number of people of a certain gender registered,
  Last student to join
  Adding student to class
  Remove student from class
  Getting the average age of all the student
*/

use chrono::prelude::*;
use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    age: u16,
    gender: char,
    date_joined: DateTime<Local>,
}

fn take_input() -> String {
    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("Please enter a valid input");

    return target;
}

fn create_student() -> Student {
    println!("Enter your name:");
    let name = take_input();

    println!("Enter your age");
    let age_input = take_input();

    let age: u16 = age_input
        .trim()
        .parse()
        .expect("Age must be an integer number");

    println!("Enter your gender");
    let gender_input = take_input();
    let gen_vec: Vec<char> = gender_input.trim().chars().collect();
    let gender = gen_vec[0];

    Student {
        name,
        age,
        date_joined: Local::now(),
        gender,
    }
}

fn reference_student(student: &Student) {
    println!(
        "Added {}.\n Stats-\nAge: {}\nGender: {}\nJoined: {}",
        student.name, student.age, student.gender, student.date_joined
    );
}

fn add_student_to_list(class_list: &mut Vec<Student>, student: Student) {
    reference_student(&student);
    class_list.push(student);
}

fn add_new_student(class_list: &mut Vec<Student>) {
    let student = create_student();

    add_student_to_list(class_list, student);

    println!("{:?}", class_list);
}

fn main() {
    let mut class_list: Vec<Student> = vec![];

    println!("CLI Class Registry");

    loop {
        println!("What action do you want to take?\n A - add student, B - get number of students, C - close");
        let choice_input: Vec<char> = take_input().trim().chars().collect();
        let choice = choice_input[0];
        match choice {
            'A' => add_new_student(&mut class_list),
            'C' => {
                println!("Thank you! Bye!");
                break;
            }
            _ => println!("Please enter a valid value"),
        }
    }
}
