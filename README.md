# Java Installer

This is a simple Java Installer CLI tool built with Rust.

## Usage

To run the installer, use the following command:

```bash
cargo run install <version>
```

### Options

- `--path <path>`  
    Specify the installation path.

- `--package-type <package_type>`  
    Specify the type of package to install. Default is `jdk`.

- `--force`  
    Force the installation, even if the version is already installed.

- `--help`  
    Display help information about the command.

## Example

```bash
cargo run install 17 --package-type jre --force
```
