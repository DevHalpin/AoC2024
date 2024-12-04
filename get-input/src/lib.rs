use reqwest::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[tokio::main]
pub async fn fetch_input(day: &str) -> Result<String, Error> {
    //Define the cache file name
    let cache_file = format!("input_day_{}.txt", day);

    // Check if the cache file exists
    if Path::new(&cache_file).exists() {
        // If the cache file exists, read the input from it
        let input = fs::read_to_string(&cache_file).expect("Failed to read cache file");
        println!("Using cached input for day {}", day);
        println!("{}", input);
        return Ok(input);
    }

    println!("Fetching input for day {}", day);
    //Prompt for the session cookie
    println!("Enter your session cookie:");
    io::stdout().flush().unwrap();
    let mut cookie = String::new();
    io::stdin().read_line(&mut cookie).unwrap();
    let cookie = cookie.trim();
    // The URL for the input page
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    
    // Create an HTTP client
    let client = reqwest::Client::new();
    
    // Send the GET request with the session cookie
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", cookie))
        .send()
        .await?;
    
    // Check if the request was successful
    if response.status().is_success() {
        // Get the response text
        let input = response.text().await?;
        
        // Print the input or save it to a file
        println!("Input:\n{}", input);
        // Save the input to a file
        fs::write(&cache_file, input.clone()).expect("Failed to write cache file");
        return Ok(input);
    } else {
        // Handle the error
        println!("Failed to fetch input. Status code: {}", response.status());
        return Err(response.error_for_status().unwrap_err());
    }
    
}
