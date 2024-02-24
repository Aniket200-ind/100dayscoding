# URL Sniffer

URL Sniffer is a Rust-based tool designed to capture, analyze, and process URLs for hyperlinks extraction. This project leverages the power of Rust's asynchronous programming capabilities to efficiently handle network requests and parsing tasks.

## Features

- **URL Scraping**: Enter a URL to scrape for links and either print them to the console or save them to a file.
- **Asynchronous Processing**: Built on top of `tokio`, this tool performs asynchronous network requests to handle multiple URLs simultaneously.
- **Flexible Output**: Users can choose to either view the scraped URLs directly in the console or save them to a file for further analysis.

## Getting Started

### Prerequisites

- Rust (edition 2021 or later)
- Cargo package manager

### Installation

1. Clone the repository:

```bash
git clone https://github.com/Aniket200-ind/100dayscoding/tree/main/url-sniffer
```

2. Navigate to the project directory:

```bash
cd url-sniffer
```

3. Build the project:

```bash
cargo build --release
```

4. Run the executable:

```bash
cargo run --release
```

## Usage

After running the tool, you will be greeted with a simple CLI interface:

1. Enter a URL to scrape for links and print them to the console.
2. Enter a URL to scrape for links and save them to a file.
3. Exit the program.

Choose an appropriate option by entering the corresponding number.

## Dependencies

- [reqwest](https://crates.io/crates/reqwest): An ergonomic HTTP client for Rust.

- [error-chain](https://crates.io/crates/error-chain): A crate for defining custom error types.

- [tokio](https://crates.io/crates/tokio): An asynchronous runtime for Rust.

- [select](https://crates.io/crates/select): A Rust library for selecting and extracting elements from HTML documents.
