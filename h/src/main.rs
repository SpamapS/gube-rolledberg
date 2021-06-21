// gube-rolledberg
// Copyright (C) 2021  Clint Byrum
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use itertools::Itertools;

mod nysiis;

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

fn get_e() -> std::io::Result<char> {
    let dict: File = File::open("/etc/dictionaries-common/words")?;
    let reader = BufReader::new(dict);
    let mut chars_left: HashMap<char, usize> = HashMap::new();
    for line_read in reader.lines() {
        match line_read {
            Ok(ref line) => {
                // What letters get removed?
                let code =  nysiis::get_nysiis(line.clone());
                for c in line.to_uppercase().chars().unique() {
                    let this_cc = code.chars().filter(|cc| *cc == c).count();
                    chars_left.entry(c).and_modify(|e| { *e += this_cc }).or_insert(this_cc);
                }
            },
            Err(e) => return Err(e)
        }
    }
    // XXX: 10967 was the count for e with my words file which is likely to only work on my machine
    let left: String = chars_left.drain().filter(|(_, v)| *v >= 10900 && *v <= 10975).map(|(k,_)| k).collect::<String>().to_lowercase();
    return Ok(left.chars().nth(0).unwrap());
}

fn main() -> std::io::Result<()> {
    println!("{}{}", get_h().expect("DOH"), get_e().unwrap());
    return Ok(());
}
