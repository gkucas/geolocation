
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::net::Ipv4Addr;
use std::process::ExitCode;
use std::time::Duration;

const OK: &str = "OK";
const ERR: &str = "ERR";

/// Reads the database and returns a tuple with a vector of (country_code, location_code) and a dictionary of unique reference strings, inverted to map codes to strings.
fn read_db(file_name: &str) -> Result<(Vec<(u16, u16)>, HashMap<String, u16>), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_name)?;
    let mut locations = Vec::with_capacity(14614528);
    let mut original_string_dict = HashMap::new();  // Maps string to u8 code

    let mut counter = 0u16;  // Unique code counter

    for record in rdr.records() {
        let record = record?;

        // Get the country and location strings
        let country = record.get(2).unwrap().to_owned();
        let location = record.get(5).unwrap().to_owned();

        // Ensure unique code for the country
        let country_code = *original_string_dict.entry(country.clone())
            .or_insert_with(|| {
                let code = counter;
                counter += 1;
                code
            });

        // Ensure unique code for the location
        let location_code = *original_string_dict.entry(location.clone())
            .or_insert_with(|| {
                let code = counter;
                counter += 1;
                code
            });

        // Calculate the IP range entries and add to locations
        let min: u32 = record.get(0).unwrap().parse()?;
        let max: u32 = record.get(1).unwrap().parse()?;

        if min < 16777215 || max > 3758096383 {
            continue;
        }

        let entries_needed = ((max - min) / 256) + 1;
        for _ in 0..entries_needed {
            locations.push((country_code, location_code));
        }
    }

    Ok((locations, original_string_dict))
}

fn main() -> ExitCode {
    let mut db: Vec<(u16, u16)> = Vec::new();
    let mut inverted_string_dict: HashMap<u16, String> = HashMap::new();

    println!("READY");
    loop {
        let mut guess = String::new();
        if io::stdin().read_line(&mut guess).is_err() {
            println!("Error reading input");
            return ExitCode::FAILURE;
        }

        let trimmed_guess = guess.trim();
        if trimmed_guess.starts_with("LOOKUP") {
            let ip = trimmed_guess.split_whitespace().nth(1);
            if ip.is_none() {
                println!("Invalid LOOKUP command");
                continue;
            }

            let ip_num = ip_to_number(ip.unwrap()) as usize;
            if db.is_empty() {
                println!("Database not loaded");
            } else {
                let index = (ip_num / 256) - 65536;
                if index < db.len() {
                    let (country_code, location_code) = db[index];

                    let country = inverted_string_dict.get(&country_code).unwrap();
                    let location = inverted_string_dict.get(&location_code).unwrap();

                    println!("{},{}", country, location);
                } else {
                    println!("IP out of range");
                }
            }
        } else if trimmed_guess == "LOAD" {
            match read_db("/Users/gkucas/RustroverProjects/geolocator/database.csv") {
                Ok((locations, orig_dictionary)) => {
                    db = locations;
                    inverted_string_dict = orig_dictionary.into_iter().map(|(k, v)| (v, k)).collect();
                    println!("{}", OK);
                }
                Err(_) => {
                    println!("{}", ERR);
                    return ExitCode::FAILURE;
                }
            }
        } else if trimmed_guess == "EXIT" {
            println!("OK");
            break;
        } else {
            println!("Unknown command");
            return ExitCode::FAILURE;
        }
    }

    std::thread::sleep(Duration::from_secs(1));
    ExitCode::SUCCESS
}

fn ip_to_number(ip: &str) -> u32 {
    ip.parse::<Ipv4Addr>().expect("Cannot parse IP address").into()
}
