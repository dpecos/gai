# GAI (Git Auto-Information)

GAI (Git Auto-Information) is a command-line interface (CLI) tool written in Rust that generates a commit message from a `git diff` by reading from its standard input (stdin). It helps automate the process of creating informative and concise commit messages based on the changes made to your Git repository.

## Features

- Automatically generates commit messages based on `git diff` input.
- Uses a smart algorithm to analyze the changes and generate descriptive commit messages.
- Helps to save time and effort in writing commit messages manually.

## Getting Started

### Prerequisites

Before using GAI, ensure that you have the following prerequisites installed on your system:

- [Git](https://git-scm.com/downloads)
- [Rust](https://www.rust-lang.org/tools/install)

### Installation

To install GAI, follow these steps:

1. Clone the GAI repository:

   ```shell
   git clone https://github.com/dpecos/gai.git
   ```

2. Change into the cloned repository directory:

   ```shell
   cd gai
   ```

3. Build and install GAI using Cargo:

   ```shell
   cargo install --path .
   ```

   This will build GAI and install it into your system.

### Usage

To use GAI, follow these steps:

1. Navigate to your Git repository's root directory.
2. Run the `git diff` command to generate the diff of your changes.
3. Pipe the `git diff` output to GAI:

   ```shell
   git diff | gai
   ```

   GAI will process the input from `git diff` and generate a commit message based on the changes.

   **Note:** GAI reads from the standard input (stdin), so make sure to pipe the `git diff` output correctly.

4. Review the generated commit message and make any necessary adjustments.
5. Commit your changes using the generated commit message:

   ```shell
   git commit -m "$(git diff | gai)"
   ```

   This will create a new commit with the generated commit message.

## Contributing

Contributions to GAI are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request to the [GitHub repository](https://github.com/dpecos/gai).

Before contributing, please review the [Contribution Guidelines](CONTRIBUTING.md) for important information.

## License

GAI is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute this project as per the license terms.

## Acknowledgments

GAI was created by Daniel Pecos Martinez. Special thanks to all the contributors who have helped make this project better.

## Contact

If you have any questions, suggestions, or feedback, you can reach out to the project maintainer at [me@danielpecos.com](mailto:me@danielpecos.com).

Happy coding!
