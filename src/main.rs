use std::{
    env,
    fs,
    process::exit,
    io::Write,
};

mod menus;
mod guess_mode;

use menus::{
    help_menu,
    version_info,
};
use guess_mode::{
    guess_file_path,
};

fn main() {
    let args: Vec<String> = env::args().collect(); // Remember: args[0] is the file path we were run from
    
    // Check terminating flags
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) || args.len() == 1 {
	help_menu();
    } else if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
	version_info();
    };

    // Determine non-terminating flags
    let mut guess_mode: bool = false;
    if args.contains(&"-g".to_string()) || args.contains(&"--guess".to_string()) {
	guess_mode = true;
    };

    // Get file path
    let file_path: String = match guess_mode {
	true => {
	    guess_file_path()
	},
	false => {
	    args[1].to_owned() // This will be the cause of issues if new flags are added
	},
    };

    let ascii_string: String = match fs::read_to_string(file_path) {
	Ok(file) => file,
	Err(e) => {
	    println!("Fatal - {}", e);
	    exit(1);
	},
    };

    println!("{}", ascii_string);
}
