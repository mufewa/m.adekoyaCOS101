use std::fs::File;
use std::io::{Write, Error};

// We define a struct to represent the Portfolio of drinks.
// This helps organize the data logically before writing it.
struct DrinkPortfolio {
    lager: Vec<String>,
    stout: Vec<String>,
    non_alcoholic: Vec<String>,
}

impl DrinkPortfolio {
    // A method to initialize the data exactly as shown in the image table
    fn new() -> Self {
        DrinkPortfolio {
            lager: vec![
                "33 Export".to_string(),
                "Desperados".to_string(),
                "Goldberg".to_string(),
                "Gulder".to_string(),
                "Heineken".to_string(),
                "Star".to_string(),
            ],
            stout: vec![
                "Legend".to_string(),
                "Turbo King".to_string(),
                "Williams".to_string(),
            ],
            non_alcoholic: vec![
                "Maltina".to_string(),
                "Amstel Malta".to_string(),
                "Malta Gold".to_string(),
                "Fayrouz".to_string(),
            ],
        }
    }

    // A method to save the portfolio to a text file
    fn save_to_file(&self, filename: &str) -> Result<(), Error> {
        // Create (or overwrite) the file
        let mut file = File::create(filename)?;

        // Write the Header
        writeln!(file, "Nigerian Breweries Plc - Drink Portfolio")?;
        writeln!(file, "======================================\n")?;

        // Write Lager Category
        writeln!(file, "LAGER:")?;
        for drink in &self.lager {
            writeln!(file, "- {}", drink)?;
        }
        writeln!(file, "")?; // Add an empty line for spacing

        // Write Stout Category
        writeln!(file, "STOUT:")?;
        for drink in &self.stout {
            writeln!(file, "- {}", drink)?;
        }
        writeln!(file, "")?; 

        // Write Non-Alcoholic Category
        writeln!(file, "NON-ALCOHOLIC:")?;
        for drink in &self.non_alcoholic {
            writeln!(file, "- {}", drink)?;
        }

        println!("Successfully saved data to {}", filename);
        Ok(())
    }
}

fn main() {
    // 1. Initialize the portfolio with the data from the table
    let portfolio = DrinkPortfolio::new();

    // 2. Attempt to save the file
    let filename = "nigerian_breweries_drinks.txt";
    
    match portfolio.save_to_file(filename) {
        Ok(_) => println!("Process completed."),
        Err(e) => eprintln!("Error saving file: {}", e),
    }
}