use std::io;

// Function to read input as f64
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}

// Area of Trapezium
fn area_trapezium() {
    let height = read_input("Enter the height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2:");
    let area = (height / 2.0) * (base1 + base2);
    println!("Area of the Trapezium = {}", area);
}

// Area of Rhombus
fn area_rhombus() {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of the Rhombus = {}", area);
}

// Area of Parallelogram
fn area_parallelogram() {
    let base = read_input("Enter the base:");
    let altitude = read_input("Enter the altitude:");
    let area = base * altitude;
    println!("Area of the Parallelogram = {}", area);
}

// Area of Cube
fn area_cube() {
    let side = read_input("Enter the length of one side of the cube:");
    let area = 6.0 * side * side;
    println!("Total Surface Area of the Cube = {}", area);
}

// Volume of Cylinder
fn volume_cylinder() {
    let radius = read_input("Enter the radius:");
    let height = read_input("Enter the height:");
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume of the Cylinder = {}", volume);
}

fn main() {
    println!("=== AREA & VOLUME CALCULATOR ===");
    println!("Choose an option:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_input("Enter your choice (1–5):");

    match choice as i32 {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => println!("Invalid choice! Please enter a number between 1 and 5."),
    }
}
