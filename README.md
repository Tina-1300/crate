# crate

Crate is a simple, cross-platform tool that helps you clean up your Rust environment by clearing all installed crates from the .cargo/registry folder. This tool helps free up disk space by removing unused or outdated Rust dependencies. It works seamlessly on Windows, macOS, and Linux.

## Features

- Cleans the .cargo/registry folder, removing all installed crates.

- Cross-platform support: works on Windows, macOS, and Linux.

- Easy-to-use and lightweight.

- Frees up disk space by removing unnecessary Rust packages.

## Installation

You can install crate via cargo, the Rust package manager. Simply run the following command :

```bash
cargo install crate
```

This will download and compile the tool, installing the crates binary in your system's cargo bin folder (usually located at `~/.cargo/bin/` on Linux/macOS or `%USERPROFILE%\.cargo\bin\` on Windows).

## Usage

Once installed, you can use crate to clean your Cargo registry with the following command :

```bash
crate --clean
```

## Steps

1. Run the command :

```bash
crate --clean
```

2. Confirmation prompt :

The tool will display a warning about the directory that will be deleted, and ask you to confirm whether you want to proceed (type y for Yes or n for No)

3. Nettoyage :

After confirmation, `crate` will remove all installed crates from the `.cargo/registry` folder, freeing up space on your disk.

## Supported Platforms

- Windows

- macOS

- Linux

## Contributing

Feel free to fork the repository, submit issues, or make pull requests if you'd like to contribute improvements or report bugs.


## License

This project is licensed under the MIT License.
