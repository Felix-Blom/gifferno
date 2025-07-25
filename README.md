# 🔥 gifferno

**Blazingly fast GIFs in your terminal.** 🦀🔥

Transform any GIF into beautiful art that plays directly in your terminal. Built with Rust for maximum performance and terminal aesthetics.

## ✨ Features

- 🎨 **Customizable characters** - Use any character to render your GIFs
- 💾 **Save & manage** - Store your favorite GIFs locally with custom names
- 🔍 **Interactive picker** - Browse and select 
- 🚀 **Blazingly fast** - Written in Rust for optimal performance
- 📱 **Cross-platform** - Works on macOS, Linux, and Windows

## 🚀 Installation

### From Source (Recommended)

In alpha still, releases with easier build strategies will come.
```bash
git clone https://github.com/username/gifferno.git
cd gifferno
cargo build --release
cargo install --path .
```

### Using Cargo

```bash
cargo install gifferno
```

## 📖 Usage


## 🛠️ Commands Reference

| Command | Description | Arguments |
|---------|-------------|-----------|
| `run` | Play a GIF directly from file path | `--file-path`, `--print-character` |
| `save` | Save a GIF to local database | `--file-path`, `--name` |
| `get` | Retrieve and play a saved GIF | `--name`, `--print-character` |
| `pick` | Interactive GIF selector | None |
| `delete` | Remove a saved GIF | `--name` |
| `clear` | Clear entire GIF database | None |

### Argument Details

- `--file-path` / `-f`: Path to the GIF file
- `--name` / `-n`: Custom name for saving/retrieving GIFs
- `--print-character` / `-p`: Character used for rendering (default: `#`)

### Running

```bash
# Run a GIF directly from file
gifferno run --file-path ./my-gif.gif

# Use custom characters for rendering
gifferno run --file-path ./nyan-cat.gif --print-character "?"
```

### Managing Your GIF Collection

```bash
# Save a GIF with a memorable name
gifferno save --file-path ./epic-fail.gif --name "fail"

# Retrieve and play a saved GIF
gifferno get --name "fail"

# Play with custom characters
gifferno get --name "fail" --print-character ""

# Browse your collection interactively
gifferno pick

# Remove a specific GIF
gifferno delete --name "fail"

# Clear your entire collection (be careful!)
gifferno clear
```

### Example Commands

```bash
# Basic usage
gifferno run -f ./dance.gif

# Custom character rendering
gifferno run -f ./fire.gif -p "^"

# Save for later
gifferno save -f ./cat.gif -n "grumpy-cat"

# Quick access to saved GIFs
gifferno get -n "grumpy-cat" -p "~"

# Interactive mode
gifferno pick
```





## 🐛 Issues & Bug Reports

Found a bug? We'd love to hear about it! Please:

1. Check existing issues first
2. Provide detailed reproduction steps
3. Include your system information
4. Add relevant error messages or logs

[Create an Issue](https://github.com/username/gifferno/issues/new)


## 🤝 Contributing
### Getting Started

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
4. **Write tests** for your code
5. **Ensure tests pass**
   ```bash
   cargo test
   ```
6. **Format your code**
   ```bash
   cargo fmt
   ```
7. **Run clippy**
   ```bash
   cargo clippy
   ```
8. **Commit your changes**
- Use [gitmoji](https://gitmoji.dev/) or [howmoji](https://github.com/Felix-Blom/howmoji) conventional commit messages.
   ```bash
   # Choose one
   gitmoji -c 
   howmoji -c
   ```
9. **Push to your branch**
   ```bash
   git push origin feature/amazing-feature
   ```
10. **Open a Pull Request**
   - Please make sure your pull request is rebased on the latest version of the application.

### Development Workflow

- Create issues for bugs and feature requests
- Use [gitmoji](https://gitmoji.dev/) or [howmoji](https://github.com/Felix-Blom/howmoji) conventional commit messages.
- Ensure all tests pass before submitting PR
- Update documentation for new features
- Follow Rust best practices and idioms

### Code Standards

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Update README if adding new commands or features

### 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- CLI powered by [clap](https://github.com/clap-rs/clap)

---
**Made with ❤️ and lots of 🔥 by the gifferno community**

*Star ⭐ this repo if you find it useful!*