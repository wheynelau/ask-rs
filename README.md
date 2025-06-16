# Ask

Toy project for asking quick questions in the terminal. 

## Quick Start

1. Setup a key on gemini since it's free. https://deepmind.google/technologies/gemini/flash/
2. export the key as an environment variable `export ASK_API_KEY=your_key` or `setx ASK_API_KEY "your_key"` on windows
3. Run `ask --configure` and accept the defaults.
4. Ask a question `ask "how do i use sudo tee"`

## Usage

```bash
$ ask --configure
# Follow the prompts
$ ask "what is the capital of France?"
```

## Installation

Download the release from the tags with your distribution.  

```bash
wget <link to release.tar.gz>
tar -xvf release.tar.gz
# move the binary to a location that is on the $PATH, like $HOME/.local/bin for user installs
cp ask*/ask $HOME/.local/bin
```

### Windows instructions:

1. Download the windows zip file
2. Extract the file
3. Copy `ask.exe` into somewhere in your PATH or add the location of this folder to your PATH

Notes: You can set permanent changes to PATH with this cmd `setx PATH "%PATH%;%CD%"` 

### From source

Requires cargo to be installed. 

```bash
git clone
cd ask-rs
cargo build --release 
# Install to bins, this installs to PATH/bin/ask
cargo install --path . --root $HOME/.local
```

## Notes

- This is a toy project, it's not meant to be used in production.
- It uses gemini as a default, but you can configure it to use other openai compatible endpoints.

## TODO
- [ ] Add history or a chat version of the ask command