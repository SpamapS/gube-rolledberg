// nysiis
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
// https://en.wikipedia.org/wiki/New_York_State_Identification_and_Intelligence_System
//
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_get_nysiis() {
    assert_eq!(get_nysiis(String::from("tree")), "TRY"); 
    assert_eq!(get_nysiis(String::from("flea")), "FL"); 
    assert_eq!(get_nysiis(String::from("beet")), "BAT"); 
    assert_eq!(get_nysiis(String::from("vasquez")), "VASG");
    assert_eq!(get_nysiis(String::from("evers")), "EVAR");
    assert_eq!(get_nysiis(String::from("macintosh")), "MCANT");
    assert_eq!(get_nysiis(String::from("knuth")), "NNAT");
    assert_eq!(get_nysiis(String::from("SCHOENHOEFT")), "SSANAFT");
    assert_eq!(get_nysiis(String::from("KRATZER")), "CRATSAR");
    assert_eq!(get_nysiis(String::from("SHOULDERS")), "SHALDAR");
    assert_eq!(get_nysiis(String::from("SHRIVER")), "SHRAVAR");
    assert_eq!(get_nysiis(String::from("KOEHN")), "CAN");
    assert_eq!(get_nysiis(String::from("A")), "A");
    assert_eq!(get_nysiis(String::from("CAN'T")), "CAD");
}

pub fn get_nysiis(name: String) -> String {
    let mut name: String = name.clone().to_uppercase().chars().filter(|c| *c >= 'A' && *c <= 'Z').collect::<String>();
    //    The algorithm, as described in Name Search Techniques,[2] is:
    //
    //    If the first letters of the name are
    //
    //        'MAC' then change these letters to 'MCC'
    //        'KN' then change these letters to 'NN'
    //        'K' then change this letter to 'C'
    //        'PH' then change these letters to 'FF'
    //        'PF' then change these letters to 'FF'
    //        'SCH' then change these letters to 'SSS'
    if name.len() > 3 && &name[0..3] == "MAC" {
        name.replace_range(0..3, "MCC");
    } else if name.len() > 2 && &name[0..2] == "KN" {
        name.replace_range(0..2, "NN");
    } else if &name[0..1] == "K" {
        name.replace_range(0..1, "C");
    } else if name.len() > 2 && (&name[0..2] == "PH" || &name[0..2] == "PF") {
        name.replace_range(0..2, "FF");
    } else if name.len() > 2 && &name[0..3] == "SCH" {
        name.replace_range(0..3, "SSS");
    };
    //
    //    If the last letters of the name are[3]
    //
    //        'EE' then change these letters to 'Y???'
    //        'IE' then change these letters to 'Y???'
    //        'DT' or 'RT' or 'RD' or 'NT' or 'ND' then change these letters to 'D???'
    //
    if name.len() == 1 {
        return name;
    }
    let l2_pos = name.len() - 2;
    let l2_set: HashSet<&&str> = HashSet::from_iter(["DT", "RT", "RD", "NT", "ND"].iter());
    let l2_str = &name[l2_pos..l2_pos+2];
    if l2_str == "EE" || l2_str == "IE" {
        name.replace_range(l2_pos..l2_pos+2, "Y ");
    } else if l2_set.contains(&l2_str) {
        name.replace_range(l2_pos..l2_pos+2, "D ");
    }
    //    The first character of the NYSIIS code is the first character of the name.
    let mut nysiis_code: String = String::from(&name[0..1]);
    //    In the following rules, a scan is performed on the characters of the name. This is described in terms of a program loop. A pointer is used to point to the current position under consideration in the name. Step 4 is to set this pointer to point to the second character of the name.
    let mut pos = 1;
    //    Considering the position of the pointer, only one of the following statements can be executed.
    let vowels: HashSet<&char> = HashSet::from_iter(['A', 'E', 'I', 'O', 'U'].iter());
    while pos < name.len() {
        let pos_char = name.chars().nth(pos).unwrap(); // pos is checked one line above
        //        If blank then go to rule 7.
        if name.chars().nth(pos).unwrap() == ' ' {
            break;
        }
        //        If the current position is a vowel (AEIOU) then if equal to 'EV' then change to 'AF' otherwise change current position to 'A'.
        if pos < name.len() - 1 && vowels.contains(&pos_char) {
            if &name[pos..pos+2] == "EV" {
                name.replace_range(pos..pos+2, "AF");
            } else {
                name.replace_range(pos..pos+1, "A");
            }
        //        If the current position is the letter
        //            'Q' then change the letter to 'G'
        } else if pos_char == 'Q' {
            name.replace_range(pos..pos+1, "G");
        //            'Z' then change the letter to 'S'
        } else if pos_char == 'Z' {
            name.replace_range(pos..pos+1, "S");
        //            'M' then change the letter to 'N'
        } else if pos_char == 'M' {
            name.replace_range(pos..pos+1, "N");
        //        If the current position is the letter 'K' then if the next letter is 'N' then replace the current position by 'N' otherwise replace the current position by 'C'
        //        If the current position points to the letter string
        //
        //            'SCH' then replace the string with 'SSS'
        //            'PH' then replace the string with 'FF'
        //
        } else if pos_char == 'K' {
            if pos < name.len() -1 && name.chars().nth(pos+1).unwrap() == 'K' { 
                name.replace_range(pos..pos+1, "N");
            } else {
                name.replace_range(pos..pos+1, "C");
            }
        } else if pos < name.len() - 2 && &name[pos..pos+3] == "SCH" {
            name.replace_range(pos..pos+3, "SSS");
        } else if pos < name.len() - 1 && &name[pos..pos+2] == "PH" {
            name.replace_range(pos..pos+2, "FF");
        //        If the current position is the letter 'H' and either the preceding or following letter is not a vowel (AEIOU) then replace the current position with the preceding letter.
        } else if pos > 1 && pos_char == 'H' && (!vowels.contains(&name.chars().nth(pos - 1).unwrap())
                                          || (pos < name.len() - 1 && !vowels.contains(&name.chars().nth(pos + 1).unwrap()))) {
            let replacement = String::from(&name[pos-1..pos]).clone();
            name.replace_range(pos..pos+1, &replacement);
        //        If the current position is the letter 'W' and the preceding letter is a vowel then replace the current position with the preceding position.
        } else if pos_char == 'W' && vowels.contains(&name.chars().nth(pos - 1).unwrap()) {
            let replacement = String::from(&name[pos-1..pos]).clone();
            name.replace_range(pos..pos+1, &replacement);
        //        If none of these rules applies, then retain the current position letter value.
        }
        //    If the current position letter is equal to the last letter placed in the code then set the pointer to point to the next letter and go to step 5.
        //    XXX this seems to be a little different in some implementations. Harumph.
        if name.chars().nth(pos).unwrap() != nysiis_code.chars().last().unwrap() || pos == 1 {
            nysiis_code.push(name.chars().nth(pos).unwrap());
        }
        //    The next character of the NYSIIS code is the current position letter.
        //    Increment the pointer to point at the next letter.
        pos += 1;
        //    Go to step 5.
    }
    //    If the last character of the NYSIIS code is the letter 'S' then remove it.
    if nysiis_code.chars().last().unwrap() == 'S' {
        nysiis_code.truncate(nysiis_code.len() - 1);
    }
    //    If the last two characters of the NYSIIS code are the letters 'AY' then replace them with the single character 'Y'.
    let n_code_l = nysiis_code.len();
    if n_code_l > 2 && &nysiis_code[n_code_l-2..n_code_l] == "AY" {
        nysiis_code.replace_range(n_code_l-2..n_code_l, "Y");
        // I think replace_range will do this? -- nysiis_code.split_off(nysiis_code.len() - 1);
    }
    //    If the last character of the NYSIIS code is the letter 'A' then remove this letter.
    if nysiis_code.chars().last().unwrap() == 'A' {
        nysiis_code.truncate(nysiis_code.len() - 1);
    }
    return nysiis_code;
}
