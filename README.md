# Ask

Toy project for asking quick questions in the terminal. The problem was that I  
occasionally needed to ask a quick question, and I didn't want to open a browser.  
It is additionally used for learning purposes.

## Usage

```bash
$ ask --configure
# Follow the prompts
$ ask what is the capital of France
```

## Installation

```
testing
```

### From source

```bash
git clone
cd ask-rs
cargo build --release 
# Install to bins, this installs to PATH/bin/ask
cargo install --path . --root $HOME/.local
```

## Notes

- This is a toy project, it's not meant to be used in production.
- It does not handle some special characters well, like `?`.
- It uses the openai API, but gemini flash works great and is free.

## Issues

- Having some issues cross compiling for aarch64 linux on cargo-dist.