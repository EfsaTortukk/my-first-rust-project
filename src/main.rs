
extern crate reqwest;
use ic_cdk::storage;
use serde_json::Value as JsonValue;


thread_local! {
    static WEATHER_STATE: storage::ThreadLocal<WeatherState> = storage::ThreadLocal::new();
}


#[derive(Debug, Default)]
struct WeatherState {
    temperature: f64,
    pressure: f64,
    humidity: f64,
    wind_speed: f64,
}

fn update_weather_state() -> Result<(), Box<dyn std::error::Error>> {
    let api_endpoint = "http://api.openweathermap.org/data/2.5/weather?q=";
    let city = "Ankara";
    let country_code = "TR";
    let open_weather_map_api_key = "628f9c55c09299e467caccb48d11dfab";

    let full_url = format!(
        "{}{},{}&APPID={}",
        api_endpoint, city, country_code, open_weather_map_api_key
    );

    let response = reqwest::blocking::get(&full_url)?;

    if response.status().is_success() {
        let weather_data: JsonValue = response.json()?;
        
        let temperature = weather_data["main"]["temp"].as_f64().unwrap_or_default();
        let pressure = weather_data["main"]["pressure"].as_f64().unwrap_or_default();
        let humidity = weather_data["main"]["humidity"].as_f64().unwrap_or_default();
        let wind_speed = weather_data["wind"]["speed"].as_f64().unwrap_or_default();


        WEATHER_STATE.with(|state| {
            state.set(WeatherState {
                temperature,
                pressure,
                humidity,
                wind_speed,
            });
        });

        Ok(())
    } else {
        Err("HTTP request failed".into())
    }
}

fn get_weather_state() -> WeatherState {
    WEATHER_STATE.with(|state| state.get().unwrap_or_default())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    update_weather_state()?;

    let current_weather = get_weather_state();

    println!("Temperature: {}", current_weather.temperature);
    println!("Pressure: {}", current_weather.pressure);
    println!("Humidity: {}", current_weather.humidity);
    println!("Wind Speed: {}", current_weather.wind_speed);

    Ok(())
}
