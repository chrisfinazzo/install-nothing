# Install-nothing

A terminal application that simulates installing things. It doesn't actually install anything.

[![asciicast](https://asciinema.org/a/757039.svg)](https://asciinema.org/a/757039)

## Installation

### Docker

```bash
docker build -t install-nothing .
docker run -it --rm --init install-nothing
```

### Homebrew

```bash
brew install install-nothing
```

## Usage

```bash
cargo build --release
cargo run --release
```

Press `Ctrl+C` to stop.

### Pick what to install

```bash
# Install specific stages
cargo run --release -- deno

# Install everything (default)
cargo run --release -- --all
```

## Docker

Build and run:

```bash
docker build -t install-nothing .
docker run -it --rm --init install-nothing
```

See available stages:
```bash
cargo run --release -- --help
```
## License

Do whatever you want with it. Well, except for movies. If you use this in a movie, credit me or something.
