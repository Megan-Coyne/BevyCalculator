# Bevy Calculator

A simple calculator application built using the Bevy game engine. This project serves as a demonstration of using Bevy for UI elements and interactive functionality. The calculator supports basic arithmetic operations like addition, subtraction, multiplication, and division.

## Features

- **Responsive Buttons**: UI buttons for digits (`0-9`) and operations (`+`, `-`, `*`, `/`, `=`).
- **Interactive UI**: Clickable buttons with visual feedback.
- **Basic Calculator Functionality**: Performs simple arithmetic operations.

## Getting Started

### Prerequisites

Ensure you have the following installed:

- **Rust**: The Rust programming language. If you don’t have it installed, follow [this link](https://www.rust-lang.org/tools/install).
- **Bevy**: The Bevy game engine, which is a Rust-based ECS (Entity-Component-System) framework. 

### Installing

1. Clone the repository:

   ```bash
   git clone https://github.com/Megan-Coyne/BevyCalculator.git
   ```

2. Navigate into the project directory:

   ```bash
   cd BevyCalculator/calculator
   ```

3. Run the project:

   ```bash
   cargo run
   ```

### Project Structure

```
bevy-calculator/
│
├── src/
│   ├── main.rs             # The main entry point of the application.
│   ├── components.rs       # Contains custom components for the calculator buttons.
│   ├── systems.rs          # Systems handling button interactions and logic.
│   └── styles.rs           # Styling properties for UI components.
│
├── assets/
│   └── fonts/              # Directory for storing fonts used in the project.
│       └── FiraSans-Bold.ttf
│
├── Cargo.toml              # Cargo configuration file, with Bevy as a dependency.
└── README.md               # Project README with setup and usage instructions.
```

### Usage

- Run the application.
- Click on the buttons to enter numbers and select arithmetic operations.
- The result will be displayed when you press the `=` button.

### Known Issues

- **Limited Arithmetic Operations**: Currently supports only basic operations. Additional features like parentheses and advanced calculations are not yet implemented.
- **UI Alignment**: If UI elements are not aligned properly, ensure that the style properties for `JustifyContent` and `AlignItems` are correctly set.

### Future Improvements

- Implement advanced arithmetic operations.
- Add keyboard support for number and operation entry.
- Improve UI/UX design with animations and hover effects.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

### Acknowledgments

- [Bevy Engine](https://bevyengine.org/) - The game engine used for this project.
- [Rust Programming Language](https://www.rust-lang.org/) - For making systems programming safer and more enjoyable.
