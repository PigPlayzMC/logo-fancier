use std::{
    env,
    fs,
    process::exit,
};

fn main() {
   let args: Vec<String> = env::args().collect();
   let mut file_path: String = Default::default();

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
       	      eprintln!("Error opening {}: {}", file_path,  e);
	      exit(1);
       },
   };

   println!("{}", ascii_art);
}

fn help_menu() {
   println!("usage: logo-fancier [-h | --help] [-v | --version] [-g | --guess]
   [path] [<flags>]

Logo-Fancier: Make logos fancier...

To use: Run 'logo-fancier (path to your fetch program's ascii art that you wish to alter)
Alternative use: Run 'logo-fancier -h', logo-fancier will attempt to guess, based on your operating system and installed fetch program(s) which ascii art to use.
");

   exit(0);
}