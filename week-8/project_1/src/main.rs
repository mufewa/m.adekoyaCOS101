use std::io;

// Define a struct to hold the data for one row of the table
struct APSRow {
    min_exp: u32,       // Minimum years required for this level
    aps_level: String,  // The Label (e.g., "APS 5-8")
    admin: String,      // Office Administrator title
    academic: String,   // Academic title
    lawyer: String,     // Lawyer title
    teacher: String,    // Teacher title
}

fn main() {
    // 1. Create the Vector containing the full table data
    // We use the lower bound of the years to determine the level (based on your prompt example: 5 years = APS 5-8)
    let table_data: Vec<APSRow> = vec![
        APSRow {
            min_exp: 1,
            aps_level: String::from("APS 1-2"),
            admin: String::from("Intern"),
            academic: String::from("-"),
            lawyer: String::from("Paralegal"),
            teacher: String::from("Placement"),
        },
        APSRow {
            min_exp: 3,
            aps_level: String::from("APS 3-5"),
            admin: String::from("Administrator"),
            academic: String::from("Research Assistant"),
            lawyer: String::from("Junior Associate"),
            teacher: String::from("Classroom Teacher"),
        },
        APSRow {
            min_exp: 5,
            aps_level: String::from("APS 5-8"),
            admin: String::from("Senior Administrator"),
            academic: String::from("PhD Candidate"),
            lawyer: String::from("Associate"),
            teacher: String::from("Snr Teacher"),
        },
        APSRow {
            min_exp: 8,
            aps_level: String::from("EL1 8-10"),
            admin: String::from("Office Manager"),
            academic: String::from("Post-Doc Researcher"),
            lawyer: String::from("Senior Associate 1-2"),
            teacher: String::from("Leading Teacher"),
        },
        APSRow {
            min_exp: 10,
            aps_level: String::from("EL2 10-13"),
            admin: String::from("Director"),
            academic: String::from("Senior Lecturer"),
            lawyer: String::from("Senior Associate 3-4"),
            teacher: String::from("Deputy Principal"),
        },
        APSRow {
            min_exp: 13, // Assumed SES is for the highest bracket
            aps_level: String::from("SES"),
            admin: String::from("CEO"),
            academic: String::from("Dean"),
            lawyer: String::from("Partner"),
            teacher: String::from("Principal"),
        },
    ];

    println!("-------------------------------------------------");
    println!("   Federal Government APS Level Checker");
    println!("-------------------------------------------------");

    // 2. Collect Input: Profession
    println!("Select your Profession:");
    println!("1. Office Administrator");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    
    let profession_choice = get_input("Enter number (1-4):");

    // 3. Collect Input: Experience
    let exp_str = get_input("Enter your years of experience (e.g., 5):");
    let experience: u32 = match exp_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered. Exiting.");
            return;
        }
    };

    // 4. Logic to find the correct row
    // We look for the highest level where experience >= min_exp
    // We initialize with the first row as default
    let mut selected_row = &table_data[0]; 
    
    for row in &table_data {
        if experience >= row.min_exp {
            selected_row = row;
        }
    }

    // 5. Determine Position and Profession Name based on choice
    let (profession_name, position) = match profession_choice.trim() {
        "1" => ("Office Administrator", &selected_row.admin),
        "2" => ("Academic", &selected_row.academic),
        "3" => ("Lawyer", &selected_row.lawyer),
        "4" => ("Teacher", &selected_row.teacher),
        _ => {
            println!("Invalid profession selection.");
            return;
        }
    };

    // 6. Final Output
    println!("\n-------------------------------------------------");
    println!("               RESULT SUMMARY");
    println!("-------------------------------------------------");
    println!("Profession:          {}", profession_name);
    println!("Years of Experience: {}", experience);
    println!("APS Level:           {}", selected_row.aps_level);
    println!("Position:            {}", position);
    println!("-------------------------------------------------");
}

// Helper function to read user input
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
