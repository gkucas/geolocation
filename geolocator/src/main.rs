use std::fs::File;
use std::io;
use std::io::Write;
use std::net::Ipv4Addr;
use std::os::unix::fs::FileExt;
use std::process::ExitCode;
use std::str;
use std::time::Duration;

fn main() -> ExitCode {
    let db_file = match File::open("new_db.txt") {
        Ok(file) => { file }
        Err(_) => {
            print_err();
            return ExitCode::FAILURE;
        }
    };
    println!("READY");
    loop {
        let mut guess: String = String::new();
        _ = io::stdin()
            .read_line(&mut guess);
        if guess.starts_with("LOOKUP") {
            let (_, ip) = guess.split_at(6);
            match ip.trim().parse::<Ipv4Addr>() {
                Ok(ipv4) => {
                    let number: u32 = ipv4.into();
                    let offset: u64 = (((number / 256) - 65536) * 46) as u64;
                    let mut buf = [0u8; 46];
                    match db_file.read_exact_at(&mut buf, offset) {
                        Err(_) => {
                            print_err();
                        }
                        Ok(_) => {
                            io::stdout().write(str::from_utf8(&buf).unwrap().trim().as_ref()).unwrap();
                            io::stdout().write("\n".as_ref()).unwrap();
                            io::stdout().flush().unwrap();
                        }
                    }
                }
                Err(_) => { print_err() }
            }
        } else if guess == "LOAD\n" {
            print_ok();
        } else if guess == "EXIT\n" {
            print_ok();
            break;
        } else {
            return ExitCode::FAILURE;
        }
    }
    // std::thread::sleep(Duration::from_secs(1));
    return ExitCode::SUCCESS;
}

fn print_ok() {
    io::stdout().write("OK\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

fn print_err() {
    io::stdout().write("ERR\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}