use std::io;
use std::fs::File;
use std::io::{ Read, Write};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?; // Use `?` to propagate the error
    let mut content = String::new();
    file.read_to_string(&mut content)?; // Use `?` to propagate the error
    Ok(content)
}

fn write_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?; // Correct usage of `File::create`
    file.write_all(content.as_bytes())?; // Use `?` to propagate the error
    Ok(()) // Correctly return Ok
}

fn main() {
    let path = r"C:\Users\User\dsa.txt";
    let mut input = String::new();
    println!("Do you wish to add to this file or read it? Use 1 to add or 2 to read: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let choice = input.trim().parse::<i32>().expect("Please type a valid number!");

    if choice == 1 {
        println!("Enter the text you wish to add to the file: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let content = input.trim();
        
        match write_file(path, content) {
            Ok(()) => println!("Content written successfully!"),
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
    } else if choice == 2 {
        match read_file(path) {
            Ok(content) => println!("File content: {}", content),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    } else {
        eprintln!("Invalid choice. Please enter 1 or 2.");
    }
}
