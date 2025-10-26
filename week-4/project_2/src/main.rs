use std::io;
fn main () {
    println!("Input if experienced or not (y/n)");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Can't read input");

    println!("Input worker's age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Can't read input");
    let age:i32 = age.trim().parse().expect("Not a valid input");

    if experience.trim() == "y" && age >= 40 {
        println!("The incentive of the employee is N 1,560,000 ");
    }
    else if experience.trim() == "y" && 30 <= age  && age < 40 {
        println!("The incentive of the employee is N 1,480,000 ");
    } 
    else if experience.trim() == "y" && age < 28 {
        println!("The incentive of the employee is N 1,300,000 ");
    }
    else {
        println!("The incentive of the employee is N 100,000 ");
    }
}