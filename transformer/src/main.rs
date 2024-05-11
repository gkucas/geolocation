use std::env::args;
use std::fs::File;
use std::io::Write;
use std::ops::Index;

fn main() {
    let db_file = args().into_iter().max().expect("Expected db file");
    let mut rdr = csv::Reader::from_path(db_file).expect("Unable to open db file for reading");
    let mut new_db = File::create("new_db.txt").expect("Unable to create new db file");
    for result in rdr.records() {
        let record = result.expect("Unable to read file record");
        let location = record.index(2).to_owned() + "," + record.index(5);
        let min: u32 = record.index(0).parse().unwrap();
        let max: u32 = record.index(1).parse().unwrap();
        if min < 16777215 || max > 3758096383 {
            continue
        }
        for _i in 0..(max - min) / 256 + 1{
            new_db.write_all(format!("{:<46}", location).as_bytes()).expect("Unable to write to new db file");
        };
    }
}