#![feature(test)]
extern crate test;
extern crate isincodes;

use test::Bencher;

#[bench]
fn validate(b: &mut Bencher) {

    const ISIN: &'static str = "US38259P5089";
    b.iter(|| isincodes::validate_isin(ISIN));
}
