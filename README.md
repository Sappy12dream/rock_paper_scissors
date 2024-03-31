# Rock, Paper, Scissors Game

This is a simple console-based Rock, Paper, Scissors game implemented in Rust.

## About

This project serves as an introduction to Rust programming, demonstrating basic concepts such as:

- Enumerations (`enum Choices` representing the player and computer choices: Rock, Paper, or Scissors).
- Methods for enums (`from_str` and `to_emoticon` for converting input strings and representing choices with emoticons).
- Error handling using `Option` and `expect`.
- Looping (`loop`) and conditional branching (`if`, `match`) for game flow control.
- Random number generation using the `rand` crate.

## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rock-paper-scissors.git
   ```

2. Navigate into the project directory:
   ```bash
   cd rock-paper-scissors
   ```

3. Build and run the application:
   ```bash
   cargo run
   ```

4. Follow the on-screen instructions to play Rock, Paper, Scissors:
   - Choose between Rock (🪨), Paper (📃), or Scissors (✂️).
   - To quit the game, enter `q`.

## Features

- Play Rock, Paper, Scissors against the computer.
- Simple and intuitive user interface.
- Emoticon representation of choices for visual appeal.

## Dependencies

This application uses the standard library `std::io` for user input/output and the `rand` crate for random number generation.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
