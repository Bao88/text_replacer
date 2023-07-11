# text_replacer
I wanted to create a small program in Rust replace char/strings in a text.

Requirements:
- Install Rust
    - https://doc.rust-lang.org/cargo/getting-started/installation.html

How to use:
- cargo run -- <"replace from"> <"replace to"> <"Sentence">

Example:
- cargo run -- " " "_" "This is a test"
- Replace space (" ") with underscore("_") in the text ("This is a test").
- The output will be this_is_a_test