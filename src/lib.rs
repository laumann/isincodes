//! ISIN code validation
//!
//! An ISIN code consists of:
//!
//!  * A two-letter country code
//!  * A nine-character alpha-numeric national security identifier
//!  * A single check digit
//!

/// Compute an ISIN checksum from the given input, if possible.
///
/// Returns the computed checksum and the checksum digit (the last digit) or
/// `None` if the input is malformed in some way.
pub fn compute_isin_checksum(input: &str) -> Option<(u8, u8)> {
    if input.len() != 12 {
        return None
    }
    if !input.chars().all(|c| c.is_ascii()) {
        return None
    }
    for c in input[0..2].bytes() {
        if c < b'A' || c > b'Z' {
            return None
        }
    }

    // The trick here is that we should view the array of numbers as a single
    // string of numbers for computing the checksum
    //
    // Iterate through all the numbers computing a running checksum
    let mut digits = [0u8; 32];
    let mut p = 0;
    for c in input.bytes() {
        let d = c - if c < 58 { 48 } else { 55 };
        p += if d < 10 {
            digits[p] = d;
            1
        } else {
            digits[p] = d / 10;
            digits[p+1] = d % 10;
            2
        };
    }
    /*
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
    */

    // Computing the checksum.
    //
    // Half of the numbers should be multiplied by two: Imagine splitting the
    // digits into two sets, one for even positions and one for odd. The set
    // that contains the last digit of the input (disregarding the checksum
    // digit) should have all its entries multiplied by two.
    //
    // All the numbers are then summed together. If a number is larger than 10
    // (after being doubled) its individiual digits are added, for example 14 is
    // added as 1 and 4.

    let l = p - 1; // digits.len() - 1;
    let mut checksum = 0;
    let mut flag = l % 2 == 0;
    for &d in &digits[0..l] {
        if flag {
            checksum += d;
        } else {
            let p = d << 1;
            if p < 10 {
                checksum += p;
            } else {
                checksum += match p {
                    10 => 1,
                    12 => 3,
                    14 => 5,
                    16 => 7,
                    18 => 9,
                    _ => unreachable!()
                };
            }
        }
        flag = !flag;
    }

    let c = (10 - (checksum % 10)) % 10;
    Some((c, digits[l]))
}

/// Check if input is valid ISIN code or not
///
pub fn validate_isin(input: &str) -> bool {
    compute_isin_checksum(input)
        .map(|(sum, check)| sum == check)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_works() {
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

    #[test]
    fn compute_works() {
        let cases = vec![
            ("foo", None),
            ("US30231G10227", None),
            ("U330231G1022", None),
            ("US30231G1022", Some((2, 2))),
            ("US0378331005", Some((5, 5))),
            ("US38259P5089", Some((9, 9))),
            ("US0378331006", Some((5, 6))),
            ("AU0000XVGZA3", Some((3, 3))),
            ("AU0000VXGZA3", Some((3, 3))),
            ("RO1718CTN047", Some((7, 7))),
            ("RO1418DBN040", Some((0, 0))),
            ("RO1617CTNOG6", Some((5, 6))),
            ("RO1617CTNOG5", Some((5, 5))),
            ("RO1718CTN0C3", Some((3, 3))),
            ("GB0002634946", Some((6, 6))),
            ("NL0000729408", Some((8, 8))),
            ("DE000CM7VX13", Some((3, 3))),
        ];

        for (input, expected) in cases {
            assert_eq!(compute_isin_checksum(input), expected, "input = {:?} (expected: {:?})", input, expected);
        }
    }
}
