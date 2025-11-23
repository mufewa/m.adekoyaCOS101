use std::io::{self, Write};

fn main() {
    // menu: (code, description, price in Naira)
    let menu = [
        ('P', "Poundo Yam / Edinkaiko Soup", 3200i64),
        ('F', "Fried Rice & Chicken", 3000),
        ('A', "Amala & Ewedu Soup", 2500),
        ('E', "Eba & Egusi Soup", 2000),
        ('W', "White Rice & Stew", 2500),
    ];

    println!(" MENU ");
    for (c, name, price) in &menu {
        println!("{} => {}  -  N{}", c, name, price);
    }

    let code = loop {
        print!("\nEnter item code (P/F/A/E/W): ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        if let Some(ch) = s.trim().chars().next() {
            let ch = ch.to_ascii_uppercase();
            if menu.iter().any(|(mc, _, _)| *mc == ch) {
                break ch;
            }
        }
        println!("Invalid code. Try again.");
    };

    let qty = loop {
        print!("Enter quantity (positive integer): ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim().parse::<i64>() {
            Ok(n) if n > 0 => break n,
            _ => println!("Invalid quantity. Try again."),
        }
    };

    
    let (_, name, price) = menu.iter().find(|(mc, _, _)| *mc == code).unwrap();
    let total = price * qty;

    println!("\nBILL");
    println!("Item : {} ({})", name, code);
    println!("Price: N{}", price);
    println!("Qty  : {}", qty);
    println/--n-   !("Total: N{}", total);
    println!("Thank you!");
}
