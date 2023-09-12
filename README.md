# Odds Analyzer

Hello, and welcome to Odds Analyzer! In this project, I'm using example lines from the past fetched from the OddsJam API to analyze sports betting odds data and identify potentially profitable betting opportunities.

## Getting Started

Before running the program, make sure you have Rust installed on your system. You can download and install Rust by following the instructions on the [official website](https://www.rust-lang.org/).

## Project Structure

This project is organized as follows:

- `main.rs`: The main entry point of the program.
- `utils`: A module containing utility functions and constants.
  - `constants.rs`: Defines constants used throughout the program.
  - `helper.rs`: Contains helper functions for odds calculations and data manipulation.

## Usage

To use this program:

1. **Data Preparation:** I've used example lines from the past fetched from the OddsJam API for this project. However, in a live environment, you would replace the following lines in `main.rs` with your actual data source or API integration:

```rust
let mut odds: File = File::open("example_odds.json").expect("Failed to find the lines.");
let mut json_string: String = String::new();
odds.read_to_string(&mut json_string).expect("Failed to read the line");
```

2. **Running the Program:** Execute the program using the `cargo run` command:

```bash
cargo run
```

3. **Understanding the Output:** The program will deserialize the JSON data, sort the betting options, calculate expected values (EV), and identify positive EV bets. It will then print the results to the console.

4. **Customization:** You can customize various aspects of the program by modifying the code in `main.rs` and the `utils` module to suit your specific requirements.
