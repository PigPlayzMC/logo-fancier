use std::{
    env,
    fs,
    process::exit,
};

fn main() {
   let args: Vec<String> = env::args().collect();

   // Check for valid input
   if args.len() == 1 {
      help_menu();
   } if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
     help_menu();
   } else if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
     println!("Logo-Fancier version: {}", env!("CARGO_PKG_VERSION"));
     exit(0);
   }

   // Try to open provided file path
   todo!();
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