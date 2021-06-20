use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_h() -> std::io::Result<char> {
    // What ratio of words start with h?
    let dict: File = File::open("/etc/dictionaries-common/words")?;
    let reader = BufReader::new(dict);
    let mut n_lines = 0;
    for line in reader.lines() {
        n_lines += 1;     
        if n_lines == 69 {
            println!("nice: {}", line.unwrap());
        }
    };
    println!("{}", n_lines);
    return Ok('h');
}

fn main() -> std::io::Result<()> {
    println!("{}", get_h().expect("DOH"));
    return Ok(());
}
