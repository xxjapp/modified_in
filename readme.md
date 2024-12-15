# `modified_in`

A command-line tool to filter input file paths by their last modified time.

## **Overview**

`modified_in` reads file paths from standard input (stdin) and filters out files that were modified outside a specified time interval. This tool is especially useful when combined with other commands to dynamically filter files based on their recent activity.

## **Features**

- Filter files based on a customizable time interval (in seconds).
- Easy integration with Unix pipelines.

## **Usage**

```bash
<other_command> | modified_in [diff_in_seconds]
modified_in --help
modified_in --version
```

### **Examples**

1. **Check if a file was modified recently:**

   To check if `Cargo.toml` was modified within the last 10 seconds:

   ```bash
   echo "Cargo.toml" | modified_in 10
   ```

2. **Filter files from `locate` results:**

   To filter files returned by the `locate` command, keeping only those modified within the last 1 second:

   ```bash
   locate "XXXX" | modified_in 1
   ```

## **Installation**

### **Prerequisites**
- Rust programming language and `cargo` installed on your system. If you don't have Rust installed, refer to the [Rust installation guide](https://www.rust-lang.org/tools/install).

### **Build and Install**

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd <repository_name>
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary:
   ```bash
   cargo install --path .
   ```

   The `modified_in` executable will be available in your system's `PATH`.

## **How It Works**

- The tool reads file paths line by line from `stdin`.
- It retrieves the last modified time of each file.
- If the difference between the current time and the file's last modified time is less than or equal to the specified interval, the file path is printed to `stdout`.
