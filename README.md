# 🔥 gifferno

**Blazingly fast GIFs in your terminal with maximum pixel density.** 🦀🔥

Transform any GIF into stunning visual art that plays directly in your terminal. Built with Rust for maximum performance, featuring high-resolution Unicode rendering and customizable visual presets.

## ✨ Features

- 🎨 **High-Resolution Mode** - Double vertical resolution using Unicode half-blocks (default)
- 🌈 **Visual Presets** - 6 different contrast and brightness styles (`bright`, `dark`, `super-bright`, `retro`, `soft`, `balanced`)
- � **Maximum Pixel Density** - Full terminal utilization with left-aligned rendering
- 🎯 **Enhanced Contrast** - S-curve processing with dynamic highlights and shadows
- ⚡ **Blazingly Fast** - Async animation with optimized frame processing
- 📱 **Cross-platform** - Works on macOS, Linux, and Windows
- 🔧 **Customizable Characters** - Use any character for rendering (when not using Unicode blocks)

## 🚀 Installation

### From Source (Recommended)

```bash
git clone https://github.com/Felix-Blom/gifferno.git
cd gifferno
cargo build --release
```

### Using Cargo

```bash
cargo install --git https://github.com/Felix-Blom/gifferno.git
```

## 📖 Usage

### Basic Usage

```bash
# Play a GIF with high-resolution mode (default)
cargo run -- examples/blink.gif

# Loop the GIF continuously
cargo run -- examples/leo.gif --loop-gif

# Use standard resolution mode
cargo run -- examples/homer.gif --standard-res
```

### Visual Presets

Experiment with different visual styles:

```bash
# Maximum brightness and contrast
cargo run -- examples/surprise.gif --preset super-bright

# High contrast with boosted highlights  
cargo run -- examples/MJ.gif --preset bright --loop-gif

# Retro terminal aesthetic with sharp transitions
cargo run -- examples/mic_drop.gif --preset retro

# Soft and gentle contrast
cargo run -- examples/winning.gif --preset soft

# Dark and moody tones
cargo run -- examples/nothing.gif --preset dark

# Balanced contrast (default)
cargo run -- examples/blink.gif --preset balanced
```

### Custom Characters

```bash
# Use custom character instead of Unicode blocks
cargo run -- examples/leo.gif --character "@" --standard-res

# Different characters create different aesthetics
cargo run -- examples/homer.gif --character "█" --preset bright
```

## 🎛️ Command Reference

| Flag | Description | Default |
|------|-------------|---------|
| `<FILE_PATH>` | Path to the GIF file | Required |
| `--preset <PRESET>` | Visual preset: `balanced`, `bright`, `dark`, `super-bright`, `retro`, `soft` | `balanced` |
| `--character <CHAR>` | Character to use for rendering | `#` |
| `--standard-res` | Use standard resolution instead of high-resolution mode | High-res default |
| `--loop-gif` | Loop the GIF continuously | `true` |

## 🎨 Visual Presets Explained

- **`balanced`** - Standard S-curve contrast for general use
- **`bright`** - High contrast with boosted highlights (1.2x brightness boost)
- **`dark`** - Muted tones with reduced highlights for dark themes
- **`super-bright`** - Maximum contrast and brightness (1.4x boost!)
- **`retro`** - Heavy contrast with sharp transitions for vintage terminal feel
- **`soft`** - Gentle contrast with smooth gradients

## 🔧 Technical Features

- **Unicode Half-Blocks**: `▀▄█` characters provide 2x vertical resolution
- **Enhanced Character Set**: ` ░▒▓▀▄█` for fine gradations
- **S-Curve Processing**: Dynamic shadow and highlight enhancement
- **Full Terminal Utilization**: 100% width, 90% height usage
- **Async Animation**: Smooth playback with proper frame timing
- **Left-Aligned Rendering**: Maximum pixel density, no centering padding

## 🚀 Performance

- **Blazingly Fast**: Optimized Rust implementation
- **Memory Efficient**: Minimal allocations during playback
- **Async Processing**: Non-blocking animation loop
- **Smart Resizing**: Maintains aspect ratio while maximizing display area

## 📱 Examples

```bash
# Show all available options
cargo run -- --help

# Quick test with example GIFs
cargo run -- examples/blink.gif --preset super-bright --loop-gif

# Compare presets
cargo run -- examples/leo.gif --preset bright
cargo run -- examples/leo.gif --preset retro
cargo run -- examples/leo.gif --preset super-bright

# Custom rendering
cargo run -- examples/homer.gif --character "●" --standard-res --preset dark
```

## 🎬 Key Controls

- **Press `q`, `ESC`, or `Ctrl+C`** to quit during playback
- **Terminal resize** is automatically handled

## 🛠️ Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Adding Example GIFs

Place GIF files in the `examples/` directory for testing.

## 🐛 Issues & Bug Reports

Found a bug? Please create an issue with:

1. Your system information (OS, terminal)
2. Command used and expected vs actual behavior
3. GIF file details (if relevant)

[Create an Issue](https://github.com/Felix-Blom/gifferno/issues/new)

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Write tests** for your code
5. **Ensure tests pass**: `cargo test`
6. **Format your code**: `cargo fmt`
7. **Run clippy**: `cargo clippy`
8. **Commit with conventional messages**
9. **Push and open a Pull Request**

### Code Standards

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Update README for new features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- GIF decoding with [gif](https://crates.io/crates/gif)
- Terminal control with [crossterm](https://crates.io/crates/crossterm)
- CLI powered by [clap](https://crates.io/crates/clap)
- Async runtime with [tokio](https://crates.io/crates/tokio)

---

**Made with ❤️ and maximum pixels by the gifferno community**

*Star ⭐ this repo if you find it blazingly fast!*