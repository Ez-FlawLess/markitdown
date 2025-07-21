# MarkItDown 📋

A simple command-line tool to read the contents of specified files or directories, format them into Markdown code blocks, and copy the result directly to your clipboard. It's perfect for quickly sharing code snippets or entire projects in platforms like GitHub, Discord, forums or AI chat bots.

-----

## 🚀 Installation

To use `MarkItDown`, you'll need to build it from the source code.

### Prerequisites

You must have the **Rust toolchain** (which includes `cargo`) installed. If you don't have it, you can get it from the official Rust website: [rust-lang.org](https://www.rust-lang.org/tools/install).

### Build Instructions

1.  **Clone the repository** and navigate into the project directory:

    ```bash
    git clone https://github.com/your-username/markitdown.git
    cd markitdown
    ```

2.  **Build the project** in release mode for the best performance:

    ```bash
    cargo build --release
    ```

3.  **Find the binary.** The executable will be in the `target/release/` directory.

      * On **Windows**: `target\release\markitdown.exe`
      * On **Linux/macOS**: `target/release/markitdown`

### Add to PATH (Recommended)

To run `markitdown` from any location in your terminal, you should add its location to your system's `PATH`.

  * **Move the binary** from the `target/release` folder to a dedicated directory (e.g., `C:\bin` on Windows or `~/.local/bin` on Linux).
  * Add this directory to your system's `PATH` environment variable.

-----

## 💻 How to Use

Using `MarkItDown` is straightforward. Just run the command and provide an optional path to the file or directory you want to process.

### Syntax

```bash
markitdown [PATH]
```

  * **`[PATH]`**: (Optional) The path to a file or a directory. **If you don't provide a path, it defaults to the current directory (`.`)**.

After running the command, the formatted content is automatically copied to your clipboard.

### Examples

1.  **Process the current directory**
    This will recursively find all supported files in your current location, format them, and copy them to the clipboard.

    ```bash
    markitdown
    ```

2.  **Process a specific directory**
    Provide the path to a directory. The tool will process all supported files within it.

    ```bash
    markitdown ./src
    ```

3.  **Process a single file**
    Provide the path to a specific file.

    ```bash
    markitdown ./src/main.rs
    ```

### Supported Files

The tool will only include files with the following extensions:

  * **Rust**: `.rs`, `.rs.in`
  * **TOML**: `.toml`
  * **Linker Script**: `.ld`, `.lds`, `.x`

Files with other extensions will be ignored.