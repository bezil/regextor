use regex::Regex;
use rand::Rng;

pub fn generate_match(pattern: &str, max_attempts: u32) -> Option<String> {
    let regex = Regex::new(pattern).unwrap();
    let mut rng = rand::thread_rng();
    let character_set = "0123456789!@#$%^&*()-_+=[]{}|\\;:,.<>/?`~";
    for _ in 0..(max_attempts+1000) {
        let generated_string: String = (0..10)
            .map(|_| {
                let index = rng.gen_range(0..character_set.len());
                character_set.chars().nth(index).unwrap()
            })
            .collect();
        if regex.is_match(&generated_string) {
            print!("{}", generated_string);
            return Some(generated_string);
        }
        print!("...{}", generated_string);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_generate_match() {
        let pattern = r"^[A-Z]\d$"; // Example pattern
        let max_attempts = 100; // Adjust the maximum attempts as needed
        let mut attempts = 0;

        while attempts < max_attempts {
            let generated_string = generate_match(pattern, 1);

            match generated_string {
                Some(matching_string) => {
                    assert_eq!(Regex::new(pattern).unwrap().is_match(&matching_string), true);
                    break; // Break the loop on successful match
                }
                None => {
                    attempts += 1;
                    println!("Failed to generate a matching string. Attempt {}/{}", attempts, max_attempts);

                    if attempts < max_attempts {
                        // Allow retry if attempts haven't reached the limit
                        println!("Do you want to try again? (y/n)");
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input.");

                        if !input.trim().to_lowercase().starts_with('y') {
                            break; // Exit the loop if the user chooses not to retry
                        }
                    } else {
                        // All attempts exhausted without success
                        assert!(false, "Failed to generate a matching string within the specified attempts.");
                    }
                }
            }
        }
    }
}
