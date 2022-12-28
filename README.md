## openai CLI

This is a command line interface for OpenAI's API. It's a work in progress, but it's already useful for some things, such as:
- Text generation
    - Given a description, openai generates a text response
- Image generation
    - Given a description, openai generates an image response


### Requirements

- Rust
- Cargo

### Running the cli

```bash
cargo run -- --help

# Text completion command:
cargo run -- text --description "What is the origin of the christmas tree?"

# Image completion command:
cargo run -- image --description "A cat astronaut coding in space"
```

### Building the cli

```bash
cargo build --release
```

