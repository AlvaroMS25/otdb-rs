# Otdb-rs

An [Open Trivia Database](https://opentdb.com) wrapper written in rust.

***

## Overview
This crate aims to be a full-featured yet simple and easy to use wrapper.

### Basic example
```rust
use otdb::prelude::{Client, Category, Difficulty};

// Let's try getting some music easy questions.
#[tokio::main]
async fn main() {
    let client = Client::new();
    
    let mut request = client.trivia();
    request.category(Category::Music)
        .difficulty(Difficulty::Easy);
    
    // Unwrapping is not a good idea, errors should be handled properly!
    let response = request.send().await.unwrap();
    
    println!("{:?}", response.results);
}
```

### Usage in blocking contexts
This crate also provides a blocking client that can be used within contexts where async is not available. In
order to use that client, the `blocking` feature must be enabled in your `Cargo.toml`, then the client can be accessed
within the `blocking` module.

Let's take the code above and use the blocking client instead of the async one:

```rust
use otdb::prelude::{Category, Difficulty};
use otdb::blocking::Client;

// Let's try getting some music easy questions.
fn main() {
    let client = Client::new();
    
    let mut request = client.trivia();
    request.category(Category::Music)
        .difficulty(Difficulty::Easy);
    
    // Unwrapping is not a good idea, errors should be handled properly!
    let response = request.send().unwrap();
    
    println!("{:?}", response.results);
}
```

As we can see, all we need to do is remove async/await syntax and we're good to go!

The only difference between using the async and blocking clients is that you don't have to
`.await` the send method in a request when using a blocking client, everything else is just the same, so switching
between clients is pretty easy!
