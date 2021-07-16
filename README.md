# A benchmark about struct and string representation performance

## Motivation

I'm writing a Rust program which will create some immutable struct values,
which are, the struct values that won't change after being initialized.
For these values, there are multiple ways to represent them. They can be represented
directly by themselves, by `Box` of them, by `Rc`of them and by `Arc` of them.
Also, there are a lot of representations of immutable strings, including `Box<str>`, `String`, `Rc<str>` , `Arc<str>` and `bytes::Bytes` .
So I decide to do a benchmark to find out which representation has the best performance under certain situation.

## Benchmark Result

You can download the criterion result from releases.

Through analysis, I found:

1. For struct, heap allocation overhead is too much. Even using `Box` or `Arc` might reduce a little moving or cloning overhead, it's not worth it.
1. For strings, `Arc` and `Rc` requires an extra clone which results in slower construction. However if the string will be cloned, `Arc` and `Rc` is often faster. `Bytes` creation is cheap but cloning cost a bit more than plain `Arc`.
1. For the string which has length of 128, the overhead difference is less than `100ns` , so if the string is not super-long or will be cloned for many times, optimizing for zero-cost might not worth it.

## TODO

Maybe I should add `Arc<String>` in benchmark because it doesn't have construction clone.
