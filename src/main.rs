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

    // The first argument (args[1]) is the image path
    let img_path = Path::new(&args[1]);

    if !img_path.exists() {
        eprintln!("File not found: {:?}", img_path);
        return Err(0);
    }

    // Display `img.jpg` with dimensions 80×25 in terminal cells.
    // The image resolution will be 80×50 because each cell contains two pixels.
    print_from_file(img_path, &conf).expect("Image printing failed.");

    Ok(())
}

