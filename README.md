# Tide - HTTP Load Testing Tool

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Tide is a powerful and concurrent HTTP load testing tool written in Rust. It's designed to help you quickly and easily assess the performance and scalability of your web applications and APIs. Inspired by the original GoFlood project, Tide provides detailed performance statistics, configurable parameters, and a graceful shutdown mechanism.

## Quick Installation

The easiest way to get started with Tide is to download a pre-built binary for your platform. No Rust installation or compilation required!

### Download Pre-built Binaries

#### Linux (x86_64)

```bash
# Download the binary
curl -L -o tide https://github.com/ao/tide/releases/latest/download/tide-linux-x86_64

# Make it executable
chmod +x tide

# Verify it works
./tide --version
```

#### macOS (Intel x86_64)

```bash
# Download the binary
curl -L -o tide https://github.com/ao/tide/releases/latest/download/tide-macos-x86_64

# Make it executable
chmod +x tide

# Verify it works
./tide --version
```

#### macOS (Apple Silicon ARM64)

```bash
# Download the binary
curl -L -o tide https://github.com/ao/tide/releases/latest/download/tide-macos-arm64

# Make it executable
chmod +x tide

# Verify it works
./tide --version
```

#### Windows (x86_64)

```powershell
# Using PowerShell
Invoke-WebRequest -Uri https://github.com/ao/tide/releases/latest/download/tide-windows-x86_64.exe -OutFile tide.exe

# Verify it works
.\tide.exe --version
```

### How to Determine Your System Architecture

- **Linux**: Run `uname -m` in your terminal. If it shows `x86_64`, use the Linux x86_64 binary.
- **macOS**: Click the Apple menu > About This Mac. For M1/M2/M3 Macs, use the ARM64 binary. For Intel Macs, use the x86_64 binary.
- **Windows**: Right-click on "This PC" > Properties, or run `systeminfo` in Command Prompt and look for "System Type".

### Adding to PATH (Optional but Recommended)

Adding Tide to your PATH allows you to run it from any directory:

#### Linux/macOS

```bash
# Move to a directory in your PATH
sudo mv tide /usr/local/bin/

# Or add the current directory to your PATH in ~/.bashrc or ~/.zshrc
echo 'export PATH="$PATH:$PWD"' >> ~/.bashrc
source ~/.bashrc
```

#### Windows

```powershell
# Move to a directory in your PATH
move tide.exe C:\Windows\

# Or add the current directory to your PATH
$env:Path += ";$pwd"
```

### Updating to Newer Versions

To update Tide, simply download the latest binary and replace your existing one:

```bash
# Example for Linux/macOS
curl -L -o tide https://github.com/ao/tide/releases/latest/download/tide-linux-x86_64
chmod +x tide
```

## Features

-   **Concurrent Requests**: Send multiple HTTP requests simultaneously to simulate real-world traffic.
-   **Configurable Parameters**: Customize concurrency, duration, timeout, and retry settings to tailor the load test to your specific needs.
-   **Retry Logic**: Automatic retry mechanism with exponential backoff to handle transient errors.
-   **Detailed Statistics**: Comprehensive performance metrics including min, max, median, and average response times to identify bottlenecks.
-   **Graceful Shutdown**: Handle CTRL+C interrupts gracefully to avoid data loss and ensure a clean exit.
-   **Colored Output**: Easy-to-read colored console output for quick analysis of results.

## Getting Started

You can use Tide in two ways:

### Option 1: Use Pre-built Binaries (Recommended for Most Users)

Download and use the pre-built binaries as described in the [Quick Installation](#quick-installation) section above.

### Option 2: Build from Source (For Developers)

1.  **Install Rust**:

    Make sure you have Rust installed. You can download it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2.  **Build the project**:

    ```bash
    cargo build --release
    ```

3.  **Run the tool**:

    ```bash
    cargo run -- --url https://example.com
    ```

    You can customize the load test using command-line options (see below).

## Usage

Basic usage:

```bash
cargo run -- --url https://example.com
```

With custom parameters:

```bash
cargo run -- --url https://example.com -n 10 -t 30 --timeout 5 --retries 3
```

### Command Line Options

-   `--url <URL>`: Target URL (required)
-   `-n, --concurrency <N>`: Number of concurrent requests per interval (default: 5)
-   `-t, --duration <SECONDS>`: Duration for which the program should run in seconds (default: 10)
-   `--timeout <SECONDS>`: Timeout for each HTTP request in seconds (default: 10)
-   `--retries <N>`: Number of retries for failed requests (default: 2)
-   `-h, --help`: Show help information
-   `-V, --version`: Show version information

## Example Output

```
*** Summary Report ***
+---------------------------+------------------------------------------+
| Target URL                | https://httpbin.org/get                  |
+---------------------------+------------------------------------------+
| Concurrency               | 3                                        |
+---------------------------+------------------------------------------+
| Duration                  | 5.426s                                   |
+---------------------------+------------------------------------------+
| Total Requests            | 15                                       |
+---------------------------+------------------------------------------+
| Successful Requests       | 15                                       |
+---------------------------+------------------------------------------+
| Failed Requests           | 0                                        |
+---------------------------+------------------------------------------+
| Min Request Time          | 284.264ms                                |
+---------------------------+------------------------------------------+
| Median Request Time       | 649.119ms                                |
+---------------------------+------------------------------------------+
| Max Request Time          | 1904.174ms                               |
+---------------------------+------------------------------------------+
| Avg Request Time          | 824.748ms                                |
+---------------------------+------------------------------------------+
```

## Architecture

The application is structured into three main modules:

-   **main.rs**: Entry point, command-line parsing, orchestration, and reporting
-   **requests.rs**: HTTP request handling with retry logic and metrics collection
-   **banner.rs**: ASCII art banner display

The tool uses Rust's async/await with Tokio for concurrent request handling and provides thread-safe metrics collection using Arc and Mutex.

## Dependencies

-   `clap`: Command-line argument parsing
-   `tokio`: Async runtime and utilities
-   `reqwest`: HTTP client
-   `url`: URL parsing and validation
-   `colored`: Colored terminal output

## Contributing

We welcome contributions to Tide! If you'd like to contribute, please follow these steps:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Implement your changes.
4.  Write tests to ensure your changes work correctly.
5.  Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.