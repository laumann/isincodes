isincodes
=========

An ISIN code validator for your Rust projects, as fast as I've been able to make
it.

The author found many ISIN code validators online, none of which actually
provided an actual implementation (just an interface to use it). This
implementation is pieced together from various sources, primarily [1] and [2].

An ISIN code is defined in ISO 6166, as a 12-character alpha-numeric code. At
some point in time you want to validate that a given 12-character string (or
byte sequence) is a valid ISIN code.

This implementation handles just validating that the structure of the ISIN is
correct. Whether the given code is an _actual_ ISIN code issued by the ISIN
organization is not checked at all.

License
-------

The code is licensed under the [GNU
GPLv3](https://choosealicense.com/licenses/gpl-3.0/). See the `COPYING` file.

References
----------

 - https://www.isin.org/
 - https://www.isincodes.net/validate-isin/
 - https://en.wikipedia.org/wiki/International_Securities_Identification_Number#Examples

[1]: https://en.wikipedia.org/wiki/International_Securities_Identification_Number#Examples
[2]: https://rosettacode.org/wiki/Validate_International_Securities_Identification_Number
