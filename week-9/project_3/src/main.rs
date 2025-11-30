fn main() {
    // 1. Define the separate datasets as Vectors (simulating the retrieved cloud data)
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // 2. Create a structure to represent the merged data
    // This allows us to treat the combined data as a single object per row
    struct EfccRecord {
        id: usize,
        name: String,
        ministry: String,
        zone: String,
    }

    // A vector to hold the final merged output
    let mut merged_dataset: Vec<EfccRecord> = Vec::new();

    // 3. Logic to merge the datasets
    // We assume all vectors are the same length. We loop through the index of one.
    for i in 0..names.len() {
        let record = EfccRecord {
            id: i + 1, // S/N starts at 1, index starts at 0
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            zone: zones[i].to_string(),
        };
        
        merged_dataset.push(record);
    }

    // 4. Display the single unified output
    println!("---------------------------------------------------------------------------------");
    println!("{:<5} | {:<30} | {:<20} | {:<15}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
    println!("---------------------------------------------------------------------------------");

    for record in merged_dataset {
        println!(
            "{:<5} | {:<30} | {:<20} | {:<15}",
            record.id, record.name, record.ministry, record.zone
        );
    }
    println!("---------------------------------------------------------------------------------");
}