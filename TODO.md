# TODO

- add canonical `&str` accessor to `Orthographia`
- add unit tests for `iter::char`
- add unit tests for `iter::char::fallible`
- make `CharFilter` accept any `char` iterator
- make `InitialCaps` also accept long vowels
- look into making `Filter::filter()` return an `Iterator` instead of `CharFilter`
- look into using `Cow` or string interning with `Orthographia`
- look into using a small string library compact_str or smol_str 
- document meaning of semivowel
- document `Orthographia` ascii and canonical formats
- naming: `try_from_ascii()` and `to_ascii_format()`
