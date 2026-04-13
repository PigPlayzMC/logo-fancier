use std::{
    env,
    fs,
    process::exit,
};

fn main() {
   let args: Vec<String> = env::args().collect();
   let mut file_path: String = Default::default();

   // Char arrays (If ascii artifacts are present, this is probably the root cause)
   let solid_chars: Vec<char>  = vec!['M', 'o'];

   // Check for valid input
   if args.len() == 1 {
      help_menu();
   } if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
     help_menu();
   } else if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
     println!("Logo-Fancier version: {}", env!("CARGO_PKG_VERSION"));
     exit(0);
   } else if args.contains(&"-g".to_string()) || args.contains(&"--guess".to_string()) {
     todo!();
   } else {
     file_path = args[1].clone();
   };

   // Try to open provided file path
   let ascii_art = match fs::read_to_string(&file_path) {
       Ok(vec) => vec,
       Err(e) => {
       	      eprintln!("Fatal - Error opening {}: {}", file_path,  e);
	      exit(1);
       },
   };

   println!("{}", ascii_art);

   let mut error_occurred: bool = false;
   let mut block_art: String = "".to_string();

   for char in ascii_art.chars() {
       if char == ' ' {
       	  block_art += " ";
       } else if solid_chars.contains(&char) {
       	  block_art += "█";
       } else {
       	 error_occurred = true;
	 block_art += &char.to_string(); // This is not meant to happen, but might result in a more salvagable file
	 eprintln!("Nonfatal - unexpected char encountered...");
       };
   };

   if error_occurred {
      eprintln!("Nonfatal - an error occured during processing, the result file may be inaccurate.");
   };
   println!("Proccessing complete! Saving not implemented...");

   println!("{}", block_art);
}

fn help_menu() {
   println!("usage: logo-fancier [-h | --help] [-v | --version] [-g | --guess]
   [path] [<flags>]

Logo-Fancier: Make logos fancier...

To use: Run 'logo-fancier (path to your fetch program's ascii art that you wish to alter)
Alternative use: Run 'logo-fancier -g', logo-fancier will attempt to guess, based on your operating system and installed fetch program(s) which ascii art to use.
");

   exit(0);
}