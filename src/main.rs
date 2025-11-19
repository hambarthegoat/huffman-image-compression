use huffman_image_compression::input_handler;

fn main() {
    if let Err(err) = input_handler::run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
