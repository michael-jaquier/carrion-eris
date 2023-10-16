use static_config::parse_items;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let source_code = parse_items().unwrap();

    // Define the output file path.
    let output_path = "src/constructed.rs";

    // Create the output directory if it doesn't exist.
    fs::create_dir_all(Path::new(output_path).parent().unwrap()).unwrap();

    // Write the Rust source code to the output file.
    let mut output_file = File::create(output_path).expect("Failed to create output file");
    output_file
        .write_all(source_code.as_bytes())
        .expect("Failed to write output file");
    Ok(())
}
