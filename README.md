# File Hasher

[![Crates.io](https://img.shields.io/crates/v/file-hasher.svg)](https://crates.io/crates/file-hasher)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast and efficient CLI tool for calculating SHA256, SHA1, and MD5 hashes of files with progress indication and colored output. Optimized for performance with streaming support for large files.

## Features

- **Multiple hash algorithms**: SHA256, MD5, SHA1, or all at once
- **Streaming support**: Efficiently processes large files without loading them into memory
- **Hash verification**: Verify files against known hash values
- **Progress bars**: Visual feedback for files larger than 10MB
- **Colored output**: Easy-to-read terminal output
- **Multiple files**: Process multiple files in a single command

## Installation

### From crates.io

```bash
cargo install file-hasher
```

### From source

```bash
git clone https://github.com/marcuspat/file-hasher
cd file-hasher
cargo install --path .
```

### Requirements

- Rust 1.70 or later
- Works on Linux, macOS, and Windows

## Usage

### Basic Usage

Hash a single file with SHA256 (default):
```bash
file-hasher file.txt
```

### Multiple Files

Hash multiple files at once:
```bash
file-hasher file1.txt file2.txt file3.txt
```

### Algorithm Selection

Choose a specific hash algorithm:
```bash
file-hasher --algorithm md5 file.txt
file-hasher --algorithm sha1 file.txt
file-hasher --algorithm sha256 file.txt
```

Calculate all hash types at once:
```bash
file-hasher --algorithm all file.txt
```

### Hash Verification

Verify a file against a known hash:
```bash
file-hasher --verify dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f file.txt
```

Verify with a specific algorithm:
```bash
file-hasher --algorithm md5 --verify 65a8e27d8879283831b664bd8b7f0ad4 file.txt
```

## Examples

```bash
# Hash a large file (shows progress bar)
$ file-hasher large-video.mp4

File: large-video.mp4
  SHA256: a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456

# Hash multiple files with all algorithms
$ file-hasher --algorithm all doc1.pdf doc2.pdf

File: doc1.pdf
  SHA256: 123abc...
  MD5: 456def...
  SHA1: 789ghi...

File: doc2.pdf
  SHA256: abc123...
  MD5: def456...
  SHA1: ghi789...

# Verify a downloaded file
$ file-hasher --verify expected_hash_here download.zip
✓ MATCH: Hash verification for 'download.zip'
  Expected: expected_hash_here
  Computed: expected_hash_here
```

## Development

### Running Tests

```bash
cargo test
```

### Code Quality Checks

```bash
cargo clippy -- -D warnings
cargo fmt -- --check
```

### CI/CD

The project includes GitHub Actions workflows that run on every push and pull request:
- Run all tests
- Check code with clippy
- Verify code formatting

## Performance

File Hasher is optimized for speed and memory efficiency:

- **Streaming**: Processes files in chunks, using minimal memory regardless of file size
- **Progress indication**: Automatic progress bars for files larger than 10MB
- **Multi-algorithm efficiency**: When using `--algorithm all`, file is read only once
- **Benchmarks**: Can hash a 1GB file in under 3 seconds on modern hardware (SHA256)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
