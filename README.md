# A benchmark about struct and string representation performance

## Motivation

I'm writing a Rust program that will creates some immutable struct values,
which are, the struct values that won't change after being initialized.
For these values, I can choose multiple way to represent them. I can directly
use the struct type, represent by a `Box` of it, a `Rc`of it and a `Arc` of it.
Also, there are a lot of representation of a immutable string, including `Box<str>` (pretty much like `String`), `Rc<str>` , `Arc<str>` and `bytes::Bytes` .
So I decided to write a benchmark to find out which representation has the best performance under certain situation.

The benchmark result is uploaded to the release. Through analysis, I found:

1. Directly using the struct type often has the best performance. `Box` or `Arc` might have lower move or clone overhead, but the heap allocation overhead is way too high.
1. If the string is never cloned, `Box` , `Rc` and `Bytes` have a similar performance while `Arc` has a little overhead.
1. If the string is cloned once, `Bytes` is pretty slow, even slower than `Box` .
1. If the string is cloned twice, `Rc` and `Arc` have the best performance, while `Bytes` is significant slower and `Box` is even slower than `Bytes`.
