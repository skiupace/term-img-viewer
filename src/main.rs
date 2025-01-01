use viuer::{print_from_file, Config};
use std::path::Path;
use std::env;

fn main()-> Result<(), u16> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    let conf = Config {
        // Set dimensions.
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };

    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return Err(0);
    }

    let img_path = Path::new(&args[1]);

    if !img_path.exists() {
        eprintln!("File not found: {:?}", img_path);
        return Err(0);
    }

    print_from_file(img_path, &conf).expect("Image printing failed.");

    Ok(())
}

