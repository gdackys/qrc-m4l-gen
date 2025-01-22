# Micro QR Code Generator

A Rust implementation of a Micro QR Code generator that creates M4-L version Micro QR codes from alphanumeric input
strings. This implementation follows the ISO/IEC 18004:2024 specification for QR code symbology.

## About

This is my first project written in Rust, developed without any prior Rust programming experience. The goal was to
create a precise and specification-compliant implementation while learning Rust's unique features and paradigms.

The generator follows the ISO/IEC 18004:2024 specification, particularly focusing on the M4-L version of Micro QR codes.
This includes proper implementation of:

- Alphanumeric mode encoding
- Error correction coding (Reed-Solomon)
- Format information generation
- Data masking patterns
- Module placement
- Symbol matrix construction

## Features

- Generates M4-L version Micro QR codes
- Supports alphanumeric input (0-9, A-Z, and special characters: space, $, %, *, +, -, ., /, :)
- Maximum input length of 21 characters
- Configurable module size for output image
- Outputs PNG format images
- Built-in input validation

## Installation

Clone the repository and build using Cargo:

```bash
git clone https://github.com/yourusername/micro-qr-code-generator
cd micro-qr-code-generator
cargo build --release
```

## Usage

```bash
cargo run -- -i "YOUR_INPUT" -m MODULE_SIZE -o output.png
```

Arguments:

- `-i, --input`: Input string (max 21 chars, alphanumeric character set only)
- `-m, --module-size`: Module size in pixels (default: 10)
- `-o, --output`: Output file name (e.g., qr_code.png)

Example:

```bash
cargo run -- -i "HELLO WORLD" -m 10 -o hello_world.png
```

## Technical Details

The generator implements the complete encoding chain for M4-L Micro QR codes:

1. Input validation and mode selection
2. Data encoding in alphanumeric mode
3. Error correction coding using Reed-Solomon codes
4. Format information generation
5. Data masking pattern selection and application
6. Final symbol construction
7. PNG image generation

## Testing

The project includes a comprehensive test suite that covers all major components of the QR code generation process. The
test cases were generated with the assistance of Claude AI to ensure thorough coverage of the specification requirements
and edge cases.

Run the tests using:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

Copyright (c) 2025 Ged Dackys

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.