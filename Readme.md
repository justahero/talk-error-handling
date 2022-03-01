Rust Error Handling
-------------------

Presentation tool is [MARP](https://marp.app/#get-started), a markdown to slides framework.

### References

* https://doc.rust-lang.org/book/ch09-00-error-handling.html
* https://nick.groenen.me/posts/rust-error-handling/
* https://fettblog.eu/rust-error-handling/
* https://www.lpalmieri.com/posts/error-handling-rust/
* https://www.sheshbabu.com/posts/rust-error-handling/
* https://kerkour.com/rust-error-handling
* https://www.unwoundstack.com/blog/rust-error-handling.html

## Topics

Error handling in Rust

* excluding compile errors
* Option (Some & None), Result (Ok, Err)
* panic!


* recoverable / unrecoverable errors
  * recoverable, program may handle error then continue
  * unrecoverable, program immediately exits


* Result (Ok, Err) cannot be ignored, has to be explicitly handled!
* compared with other languages
  * for example Java, Ruby, Python C++ exception handling not explicit, try / catch
  * Java allowed to annotate methods with possible exceptions (optional)


More error handling topics

* anyhow, thiserror, tracing
* miette (https://lib.rs/crates/miette)