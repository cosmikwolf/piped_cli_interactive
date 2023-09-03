# Crossterm Piping and Interactive Input Example

This example demonstrates how to use Rust and crossterm to accept both piped stdin input and interactive terminal input in a single CLI app. Given the scarcity of examples on piping into crossterm, I created this to provide an example for anyone trying to do the same.

the key to using using piped inputs is to use the `use-dev-tty` feature of crossterm. This allows you to use the `input()` function to read from stdin. Without this feature, the `input()` function will not work with piped input.


## Features

- **Piped `stdin` Input**: Accept input directly from another command using pipes.
- **Interactive Terminal Input**: Type and interact directly within the terminal.
- **Colored UI**: Different colors for prompts, user input, and system responses for clarity.
- **Command Handling**: Recognizes `exit` and `quit` commands for easy termination.
- **Keyboard Support**: Use `CTRL+C` to exit and the backspace key for text corrections.

## Usage

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/cosmikwolf/terminal-input-interface.git
   cd terminal-input-interface
   ```

2. **Run the Application**:
   ```bash
   cargo run
   ```

3. **Experience Piped Input**:
   ```bash
   echo "Sample piped input" | cargo run
   ```

## Dependencies

- [crossterm](https://crates.io/crates/crossterm): Used for creating the terminal interface.

## License

This project is under the MIT License. See the [LICENSE](LICENSE) file for more details.

---