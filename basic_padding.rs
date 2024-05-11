use std::error::Error;
use std::io;
use std::net::Ipv4Addr;
use std::ops::Index;
use std::process::ExitCode;
use std::time::Duration;

const OK: &str = "OK";
const ERR: &str = "ERR";

fn read_db(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let rdr = csv::Reader::from_path(file_name);
    let mut locations = Vec::with_capacity(14614528);
    for result in rdr?.records() {
        let record = result?;
        let location = record.index(2).to_owned() + "," + record.index(5);
        let min: u32 = record.index(0).parse().unwrap();
        let max: u32 = record.index(1).parse().unwrap();
        if min < 16777215 || max > 3758096383 {
            continue
        }
        for _i in 0..(max - min -1 ) / 256 + 1{
            locations.push(location.clone());
        };
    }
    Ok(locations)
}

fn main() -> ExitCode {
    let mut db: Vec<String> = Vec::new();
    println!("READY");
    loop {
        let mut guess: String = String::from("");
        _ = io::stdin()
            .read_line(&mut guess);
        if guess.starts_with("LOOKUP") {
            let (_, ip) = guess.split_at(6);
            let ip_num= ip_to_number(ip.to_string()) as usize;
            println!("{}", &db[(ip_num / 256) - 65536]);
        } else if guess == "LOAD\n" {
            match read_db("database.csv") {
                Ok(data) => {
                    db = data;
                    println!("{}", OK);
                }
                Err(_) => {
                    println!("{}", ERR);
                    return ExitCode::FAILURE;
                }
            }
        } else if guess == "EXIT\n" {
            println!("{}", OK);
            break;
        } else {
            return ExitCode::FAILURE;
        }
    }
    std::thread::sleep(Duration::from_secs(1));
    return ExitCode::SUCCESS;
}

fn ip_to_number(ip: String) -> u32 {
    ip.trim().parse::<Ipv4Addr>().expect(&*("Cannot parse ip".to_owned() + &*ip)).into()
}