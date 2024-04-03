use libretranslate::{Language, translate};
use reqwest;
use serde::Deserialize;
use tokio;

// WeatherData struct holds main temperature and weather description
#[derive(Deserialize)]
struct WeatherData {
    main: Main,
    weather: Vec<Weather>,
}
// Main struct contains the temperature
#[derive(Deserialize)]
struct Main {
    temp: f64,
}
// Weather struct holds the weather description
#[derive(Deserialize)]
struct Weather {
    description: String,
}

// Async function to fetch weather data
async fn fetch_weather(api_key: &str, city: String) -> Result<(), reqwest::Error> {
    // Build the URL for the OpenWeatherMap API request
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city.trim(),
        api_key
    );
// Make the API request and handle errors
    let response = reqwest::get(&url).await?;
    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Parse the JSON response into our WeatherData struct
        let weather_data: WeatherData = response.json().await?;
        // Extract and print relevant weather information
        let temperature = weather_data.main.temp - 273.0;
        let description = &weather_data.weather[0].description;
        println!("Weather in {}: {:.1}°C, {}", city.trim(), temperature, description);
    } else {
        // Print an error message if the request was not successful
        println!("Error: {}", response.status());
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // Set your OpenWeatherMap API key here
    let api_key = "6fbaead269907a1b093ca3476b2ddc86";
// Get the city name from the user
    let  city = "Jastków".to_string();
    //println!("Enter the city name:");
    //std::io::stdin().read_line(&mut city).expect("Failed to read line");
    // Clone the city string before passing it into the asynchronous function
    let city_clone = city.clone();
    // Spawn a Tokio task to run the asynchronous fetch_weather function
    tokio::spawn(fetch_weather(api_key, city_clone));
    // Sleep to keep the program running while the asynchronous task completes
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
