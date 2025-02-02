use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::error::Error;
use rand::Rng;

#[derive(Debug)]
struct PasswordManager {
    passwords: HashMap<String, String>, // Store passwords with a username as key
}

impl PasswordManager {
    // Create a new password manager
    fn new() -> Self {
        PasswordManager {
            passwords: HashMap::new(),
        }
    }

    // Add a new password to the manager with basic validation
    fn add_password(&mut self, username: String, password: String) {
        if !self.is_valid_password(&password) {
            println!("Password is too weak. It must be at least 8 characters long.");
            return;
        }
        self.passwords.insert(username, password);
        println!("Password added successfully.");
    }

    // Validate password strength (basic check)
    fn is_valid_password(&self, password: &str) -> bool {
        password.len() >= 8
    }

    // Retrieve a password by username
    fn get_password(&self, username: &str) -> Option<&String> {
        self.passwords.get(username)
    }

    // Generate a random strong password
    fn generate_password(&self) -> String {
        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
        let mut rng = rand::thread_rng();
        let password: String = (0..12)
            .map(|_| charset.chars().choose(&mut rng).unwrap())
            .collect();
        password
    }

    // Display all stored passwords (for testing purposes)
    fn display_all(&self) {
        for (username, password) in &self.passwords {
            println!("Username: {}, Password: {}", username, password);
        }
    }

    // Save passwords to a file
    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().create(true).append(true).open("passwords.txt")?;
        for (username, password) in &self.passwords {
            writeln!(file, "{}: {}", username, password)?;
        }
        Ok(())
    }

    // Load passwords from a file
    fn load_from_file(&mut self) -> Result<(), Box<dyn Error>> {
        let mut file = File::open("passwords.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                self.passwords.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
        Ok(())
    }
}

fn main() {
    let mut password_manager = PasswordManager::new();
    if let Err(e) = password_manager.load_from_file() {
        eprintln!("Error loading passwords from file: {}", e);
    }

    loop {
        println!("\nPassword Manager Menu:");
        println!("1. Add a new password");
        println!("2. Retrieve a password");
        println!("3. Display all passwords");
        println!("4. Generate a random password");
        println!("5. Save passwords to file");
        println!("6. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                print!("Enter username: ");
                io::stdout().flush().unwrap();
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim().to_string();

                print!("Enter password: ");
                io::stdout().flush().unwrap();
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                let password = password.trim().to_string();

                password_manager.add_password(username, password);
            }
            2 => {
                print!("Enter username to retrieve password: ");
                io::stdout().flush().unwrap();
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim();

                match password_manager.get_password(username) {
                    Some(password) => println!("Password for {}: {}", username, password),
                    None => println!("No password found for {}", username),
                }
            }
            3 => password_manager.display_all(),
            4 => {
                let password = password_manager.generate_password();
                println!("Generated password: {}", password);
            }
            5 => {
                if let Err(e) = password_manager.save_to_file() {
                    eprintln!("Error saving passwords: {}", e);
                } else {
                    println!("Passwords saved successfully.");
                }
            }
            6 => break,
            _ => println!("Invalid option. Please try again."),
        }
    }
}
