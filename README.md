# backup
**backup** is a simple backup utility written in Rust that compresses a file or directory into a timestamped zip archive. It preserves the directory structure and can be customized via an evnironment variable.

## Features
- **File and Directory Backup:** Back up single files or entire directories.
- **Timestamped Archives:** Automatically adds the current date to the backup file name.
- **Custom Backup Location:** Uses the ```BACKUP_PATH``` environment variable to determine where backups are stored. Defaults to ```$HOME/.config/backups``` if not set.
- **Preserves Directory Structure:** Ensures the top-level directory is included in the backup.

## Dependencies
This project uses the following Rust crates:
- **chrono:** Used to obtain and format the current date, which is appended as a timestamp in the backup file name.
- **walkdir:** Enables recursive traversal of directories to ensure all files and subdirectories are included in the backup.
- **zip:** Provides functionality for creating and writing zip archives to compress the backup.

## Installation
1. **Clone the repository:**

```bash
git clone https://github.com/stigsec/backup.git
cd backup
```
2. **Build the Project:**

```bash
cargo build --release
```
   The compiled binary will be located in ```target/release/backup```

## Usage
Run the binary with the path to the file or directory you want to backup:

```bash
backup /path/to/file/or/directory
```

By default, backups are stored in ```$HOME/.config/backups```. To specify a custom backup directory, set the ```BACKUP_PATH``` environment variable by adding it to your shell configuration file (e.g., ```~/.bashrc``` or ```~/.zshrc```):

```bash
export BACKUP_PATH="$HOME/.config/backups"
```
After updating your shell configuration file, reload it with:
```bash
source ~/.bashrc
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE file](LICENSE) for more details.



---

Developed by [stigsec](https://github.com/stigsec).
