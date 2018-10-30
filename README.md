isincodes
=========

An ISIN code validator for your Rust projects, as fast as I've been able to make
it.

The author found many ISIN code validators online, none of which actually
provided an actual implementation (just an interface to use it). This
implementation is pieced together from various sources, primarily [[1]] and [[2]].

An ISIN code is defined in ISO 6166 [[5]], as a 12-character alpha-numeric code. At
some point in time you may want to validate that a given 12-character string (or
byte sequence) is a valid ISIN code.

This implementation handles just validating that the structure of the ISIN is
correct. Whether the given code is an _actual_ ISIN code issued by the ISIN
organization is not checked at all.

Usage
-----

It is simple to use, there are two exposed functions:

    fn compute_isin_checksum(input: &str) -> Option<(u8, u8)>
    fn validate_isin(input: &str) -> bool

The latter just builds on the former. `compute_isin_checksum()` returns the
computed ISIN code (a number between 0 and 9) and the check digit (the last
digit of the code). For the ISIN to be considered valid the computed code must
equal the check digit. `validate_isin()` just perfoms this check.

A `None` is returned if the format can be determined to be incorrect before
computing the check digit. The checks are:

 - Is it _exactly_ 12 alphanumeric characters?
 - Are the first two characters letters (in `A` to `Z`)?
 - Is the last character a digit?

If these conditions hold up, a checksum can be computed and a `Some((checksum,
digit))` will be returned.

License
-------

The code is licensed under the [GNU
GPLv3](https://choosealicense.com/licenses/gpl-3.0/). See the `COPYING` file.

References
----------
 1. https://en.wikipedia.org/wiki/International_Securities_Identification_Number#Examples
 2. https://rosettacode.org/wiki/Validate_International_Securities_Identification_Number
 3. https://www.isin.org/
 4. https://www.isincodes.net/validate-isin/
 5. https://www.iso.org/standard/44811.html

[1]: https://en.wikipedia.org/wiki/International_Securities_Identification_Number#Examples
[2]: https://rosettacode.org/wiki/Validate_International_Securities_Identification_Number
[5]: https://www.iso.org/standard/44811.html
