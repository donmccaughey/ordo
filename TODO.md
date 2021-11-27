# TODO

- add canonical `&str` accessor to `Orthographia`
- look into making `FallibleChars` accept any `char` iterator
- look into making `CharSequences::fallible_chars()` return an `Iterator` instead of `FallibleChars`
- look into using `Cow` or string interning with `Orthographia`
- look into using a small string library compact_str or smol_str 
- look into a compressed representation of `Locutio` like compact_str or smol_str for short phrases
- document meaning of semivowel
- document `Orthographia` ascii and canonical formats
- naming: `try_from_ascii()` and `to_ascii_format()`
