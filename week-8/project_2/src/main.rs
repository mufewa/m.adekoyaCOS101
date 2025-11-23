use std::io;

// 1. Define a Struct (A Compound Data Type) to hold developer details
struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    // 2. Define a Vector (Another Compound Data Type) to store the list of developers
    let mut candidates: Vec<Developer> = Vec::new();

    println!("--- EY Nigeria Developer Scouting Program ---");

    // Start the loop to gather input
    loop {
        // --- Input: Name ---
        println!("\nEnter Developer's Name:");
        let mut name_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read line");
        let name = name_input.trim().to_string();

        // --- Input: Experience ---
        println!("Enter Years of Experience:");
        let mut years_input = String::new();
        io::stdin()
            .read_line(&mut years_input)
            .expect("Failed to read line");

        // Convert the string input to a number (u32)
        let years_of_experience: u32 = match years_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number entered. Setting experience to 0.");
                0
            }
        };

        // Create a new Developer struct and push it into the vector
        let new_dev = Developer {
            name,
            years_of_experience,
        };
        candidates.push(new_dev);

        // --- Loop Check: Continue or Stop ---
        println!("Do you want to add another candidate? (yes/no):");
        let mut continue_input = String::new();
        io::stdin()
            .read_line(&mut continue_input)
            .expect("Failed to read line");

        // If user types "no", break the loop
        if continue_input.trim().to_lowercase() == "no" {
            break;
        }
    }

    // Logic to find the developer with the highest experience
    // Check if the list is not empty
    if candidates.len() > 0 {
        let mut highest_years = 0;
        let mut best_dev_name = String::new();

        // Iterate through the vector
        for dev in &candidates {
            if dev.years_of_experience > highest_years {
                highest_years = dev.years_of_experience;
                best_dev_name = dev.name.clone();
            }
        }

        println!("\n---------------- RESULTS ----------------");
        println!(
            "The developer with the highest experience is: {} with {} years.",
            best_dev_name, highest_years
        );
    } else {
        println!("\nNo candidates were entered.");
    }
}