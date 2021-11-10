# TODO

- remove `Default` for `Orthographia`
- update `Debug` for `Orthographia` to use iterators
- inline local variable in `Display` impl for `Orthographia`
- add unit tests for `iter::char`
- add unit tests for `iter::char::fallible`
- make `CharFilter` accept any `char` iterator
- make `InitialCaps` also accept long vowels
- look into making `Filter::filter()` return an `Iterator` instead of `CharFilter`
- look into using `Cow` or string interning with `Orthographia`
- document meaning of semivowel