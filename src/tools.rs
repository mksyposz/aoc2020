use chrono::{DateTime, Datelike, Local};

fn get_dom() -> u32 {
    let now: DateTime<Local> = Local::now();
    now.day()
}

pub fn get_day_relative_path(day: u32) -> String {
    String::from(if day < 10 {"data/day0"} else {"data/day"}) + &day.to_string()
        + ".txt"
}

fn day_input_exist(day: u32) -> bool {
    let mut file_name = std::env::current_dir().unwrap();
    let relative_name = get_day_relative_path(day);
    file_name.push(relative_name);
    std::fs::metadata(file_name).is_ok()
}

fn get_day_input(day: u32, cookie: String) -> Result<String, ()>{
    let url = format!("https://adventofcode.com/2020/day/{}/input", day);
    let client = reqwest::blocking::Client::new();
    let response = client.get(url.as_str())
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .unwrap();
    if response.status().is_success() {
        println!("We have successfully retrived the data from the website");
        Ok(response.text().unwrap())
    } else {
        println!("We failed with {}", response.status());
        Err(())
    }
}

fn save_input_to_file(input: String, day: u32) {
    let file_name = get_day_relative_path(day);
    std::fs::write(file_name.as_str(), (input + "\n").as_str()).unwrap()
}

pub fn get_data(day: Option<u32>) -> u32 {
    let day = if day == None {get_dom()} else {day.unwrap()};
    if !day_input_exist(day) {
        println!("We need to get data for day {}", day);
        let cookie = get_session_cookie();
        if let Ok(input) = get_day_input(day, cookie) {
            save_input_to_file(input, day);
        } else {
            panic!("");
        }
    } else {
        println!("Input data exists for day {}", day);
    }
    day
}

const SESSION_COOKIE : &str = "AOC_SESSION_COOKIE";
fn get_session_cookie() -> String {
    use dotenv::{dotenv, var};
    dotenv().unwrap();
    let session_cookie = var(SESSION_COOKIE).unwrap();
    format!("session={}", session_cookie)
}
