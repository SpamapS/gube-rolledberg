use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn get_h() -> std::io::Result<char> {
    // What ratio of words start with h?
    let dict: File = File::open("/etc/dictionaries-common/words")?;
    let reader = BufReader::new(dict);
    let mut n_lines = 0;
    let mut n_h_lines = 0;
    let mut letter_counts: HashMap<char, usize> = HashMap::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if let Some(c) = line.chars().nth(0) {
                    n_lines += 1;     
                    let l_c = c.to_lowercase().next().unwrap();
                    if l_c == 'h' {
                        n_h_lines += 1;
                    }
                    letter_counts.entry(l_c).and_modify(|e| { *e += 1}).or_insert(1);
                }
            },
            Err(e) => return Err(e)
        }
    };
    let h_ratio  = n_h_lines as f32 / n_lines as f32;
    let entries: Vec<(char, usize)> = letter_counts.drain().filter(|e| e.1 as f32 / n_lines as f32 == h_ratio).collect::<Vec<(char, usize)>>();
    return Ok(entries[0].0);
}

fn main() -> std::io::Result<()> {
    println!("{}", get_h().expect("DOH"));
    return Ok(());
}
