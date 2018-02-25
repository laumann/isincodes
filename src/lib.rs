//! ISIN code validation
//!
//! An ISIN code consists of
//!
//!  * A two-letter country code
//!  * A nine-character alpha-numeric national security identifier
//!  * A single check digit
//!
//! 

/// Check if input is valid ISIN code or not
///
pub fn validate_isin(input: &str) -> bool {
    if input.len() != 12 {
        return false;
    }
    if !input.chars().all(|c| c.is_ascii()) {
        return false;
    }

    // The trick here is that we should view the array of numbers as a single
    // string of numbers for computing the checksum
    //
    // Iterate through all the numbers computing a running checksum
    let digits = input.bytes()
        .map(|c| c - if c < 58 { 48 } else { 55 })
        .fold(vec![], |mut v, c| {
            if c < 10 {
                v.push(c);
            } else {
                v.push(c / 10);
                v.push(c % 10);
            }
            v
        });

    let mut checksum = 0;
    let mut flag = (digits.len() - 1) % 2 == 0;
    for &d in &digits[0..digits.len()-1] {
        if flag {
            checksum += d;
        } else {
            let p = d << 1;
            if p < 10 {
                checksum += p;
            } else {
                checksum += p / 10;
                checksum += p % 10;
            }
        }
        flag = !flag;
    }

    let c = (10 - (checksum % 10)) % 10;
    c == digits[digits.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = vec![
            ("foo", false),
            ("US30231G1022", true),
            ("US0378331005", true),
            ("US38259P5089", true),
            ("US0378331006", false),
            ("AU0000XVGZA3", true),
            ("AU0000VXGZA3", true),
            ("RO1718CTN047", true),
            ("RO1418DBN040", true),
            ("RO1617CTNOG6", false),
            ("RO1617CTNOG5", true),
            ("RO1718CTN0C3", true),
            ("GB0002634946", true),
            ("NL0000729408", true),
            ("DE000CM7VX13", true),
        ];

        for (input, expected) in cases {
            assert_eq!(validate_isin(input), expected, "input = {} (expected: {})", input, expected);
        }
    }
}
