use std::io;

fn main() {
    println!("Quadratic Equation format: ax² + bx + c = 0");
    println!("Enter the coefficients a, b, and c:");

    let a = read_number("Enter a:");
    if a == 0.0 {
        println!("Not possible, ZERO cannot be an input.");
        return;
    }

    let b = read_number("Enter b:");
    let c = read_number("Enter c:");

    // Calculate discriminant
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The two Real roots are: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!(" The two roots are the same, the Root is: {:.2}", root);
    } else {
        let real = -b / (2.0 * a);
        let imag = (-discriminant).sqrt() / (2.0 * a);
        println!("There are two complex roots: {:.2} + {:.2}i and {:.2} - {:.2}i", real, imag, real, imag);
    }
}

fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}
