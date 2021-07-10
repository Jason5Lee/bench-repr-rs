# A benchmark about struct and string representation performance

## Motivation

I'm writing a Rust program which will create some immutable struct values,
which are, the struct values that won't change after being initialized.
For these values, there are multiple ways to represent them. They can be represented
directly by themselves, by `Box` of them, by `Rc`of them and by `Arc` of them.
Also, there are a lot of representations of immutable strings, including `Box<str>`, `String`, `Rc<str>` , `Arc<str>` and `bytes::Bytes` .
So I decide to do a benchmark to find out which representation has the best performance under certain situation.
