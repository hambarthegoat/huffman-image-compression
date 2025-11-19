# Huffman Image Compression

This project compresses JPEG images using Huffman coding and provides a small CLI for compressing and decompressing `.jpg`/`.jpeg` files.

## Step 1: Install Rust

### Linux
1. Install Rust via `rustup` (if needed):
   ```bash
   curl https://sh.rustup.rs -sSf | sh
   ```
2. Close and reopen your shell so `cargo` is on the `PATH`.

### Windows (PowerShell)
1. Run the `rustup` installer from PowerShell:
   ```powershell
   (Invoke-WebRequest -Uri https://sh.rustup.rs -UseBasicParsing).Content | sh
   ```
2. Restart PowerShell to pull updated `cargo`/`rustc` into the session.

## Step 2: Build the project
```bash
cargo fmt
cargo check
```

## Step 3: Run the CLI (same commands on Linux/Windows)

### Compress a JPEG/PNG into `.himg`
```bash
cargo run -- compress img_file/file.jpg himg_file/file.himg
```
The output path is optional (defaults to `<input>.himg`).

### Decompress a `.himg` back to JPEG
```bash
cargo run -- decompress himg_file/file.himg img_file/image-restored.jpg
```
The CLI enforces `.himg` for the source and `.jpg/.jpeg` for the destination, so use matching file extensions.
