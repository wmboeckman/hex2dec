# Software Requirements

For this project, I will borrow elements of the `Agile` framework and adapt it for solo development. While much of the framework in question focuses on maximizing collaboration, the core set of PM features (User Stories, Story Points, Sprints) will be useful.

### Product Summary

My goal is to create a command-line application that provides a few useful tools for working with `hexadecimal` numbers. The application will designed with the bash shell in mind, aiming to be more ergonomic than opening the python interpreter for the same task.

The application should be able to take user input via `CLI` arguments, and return clean, formatted output. It should be able to handle basic type conversions (base-16 <---> base-10) and offset calculations (difference of two hexadecimals in bytes).

### User Stories

User stories have been broken up into relevant categories, along with a general priority rating (core, extended, optional) and `Story Point` score (1,2,3,5,8).

#### Core: base conversion (5 SP)

- As a user, I want to input one or more hexadecimal numbers in order to get equivalent decimal numbers in return.
- As a user, I want to input one or more decimal numbers in order to get equivalent hexadecimal numbers in return.

#### Core: string literals as input / output (2 SP)

- As a user, I want to use manually-written strings or 'piped' data as CLI inputs.
- As a user, I want the program to print it's output to the console.
    - *Formatting?*

#### Extended: offset calculation (3 SP)

- As a user, I want to input sets of two hexadecimal numbers and get their offset in bytes in return.

#### Extended: CLI color formatting (3 SP)

- As a user, I want the CLI output in color to increase readability in the terminal.
    - *Alternating colors by row?*

#### Optional: files as input / output (5 SP)

- As a user, I want to optionally provide a file path instead of a string input.
- As a user, I want the program to optionally write it's output to a file instead of printing to stdout.

### MVP (Minimum Viable Product)

- base conversion algorithm
- basic CLI interface
    - string literal inputs, stdout outputs

### Deadlines

I must reach the MVP by the end of **February 28th 2026**.

I plan in breaking this project down into 3 weekly sprints:

- **Sprint 1**: Base-conversion algorithms, basic CLI functionality
    - Story Point total: 7
    - Due Feb. 14th
- **Sprint 2**: Code refactor, Offset algorithm
    - Story Point total: 6
    - Due Feb. 21st
- **Sprint 3**: CLI file I/O, CLI colors, final integrations
    - Story Point total: 5
    - Due Feb. 28th

## System Design

I have chosen to write the program in `Rust`. This was primarily for its memory safety, but I also wanted an excuse to actually finish a project with it outside of testing out random cargo crates.

I am also considering the use of libraries to help with the CLI interface and automated testing (see below).

The application should keep the business logic separated from the user interface (i.e. module separation between UI, Algorithms and File IO).

## Research

#### Relevant Rust Libraries:

- [clap](https://docs.rs/clap/latest/clap/index.html): a CLI parser for Rust

## Testing

For the purposes of this project, I will limit my testing to Unit Tests for each algorithm and the File I/O system. Testing the CLI interface presents new sets of challenges beyond the scope of this project, and for something this simple I believe it would be unnecessary.