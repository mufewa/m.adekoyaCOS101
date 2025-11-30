use std::fs::File;
use std::io::Write;

// 1. Define a struct to represent a Student
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    // 2. Create a vector containing the specific student data from the image
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // 3. Display the details to the console
    println!("\n--- PAU SMIS STUDENT DATA ---");
    // {:<20} means left-align with 20 characters of padding for neat columns
    println!("{:<20} {:<15} {:<15} {:<10}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{}", "-".repeat(60)); // Print a separator line

    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<10}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // 4. Save the details into a file (csv format which opens in Excel)
    let file_path = "pau_smis.csv";
    match save_to_file(&students, file_path) {
        Ok(_) => println!("\n[Success] Data successfully saved to '{}'.", file_path),
        Err(e) => println!("\n[Error] Failed to save file: {}", e),
    }
}

// Helper function to handle file writing
fn save_to_file(students: &Vec<Student>, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write the Header Row
    writeln!(file, "Student Name,Matric. Number,Department,Level")?;

    // Write the Data Rows
    for student in students {
        writeln!(
            file,
            "{},{},{},{}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }
    Ok(())
}