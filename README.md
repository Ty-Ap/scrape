# Web Scraper

## Overview

This Rust program is designed to scrape a webpage for specific keywords and save the results to a text file. It utilizes `reqwest` for HTTP requests, `scraper` for parsing HTML, and `regex` for matching specific patterns.

## Features

- Asynchronous HTTP requests to fetch webpage content.
- HTML parsing to extract relevant text.
- Regex pattern matching to find specific words.
- Writing the results to a text file.

## Requirements

- Rust Programming Language
- Cargo (Rust's Package Manager)

## Dependencies

- `reqwest`: For making HTTP requests.
- `scraper`: For parsing and extracting information from HTML.
- `regex`: For regex pattern matching.
- `tokio`: An asynchronous runtime.

## Installation

1. Ensure Rust and Cargo are installed.
2. Clone the repository.
3. Navigate to the project directory.

## Usage

1. change URL target and regex matching as needed.
2. Run the program using Cargo:

   ```shell
   cargo run

The program will scrape the specified URL (e.g., "<https://en.wikipedia.org/wiki/State_of_Palestine>")

Keywords found will be saved in scraped.txt in the project directory.

## Configuration

The URL to scrape and the regex patterns can be modified in the main function.
The text selector in the scrape function can be adjusted based on the HTML structure of the target webpage.

## Contributing

Contributions to improve the scraper or extend its functionality are welcome. Please follow standard Rust coding practices for contributions.

## License

This project is released under the Creative Commons Zero v1.0 Universal License. This means the work is free to use for any purpose, without any conditions, unless such conditions are required by law.

## Disclaimer

This scraper is for educational purposes. Ensure compliance with the website's terms of service and legal regulations regarding web scraping.