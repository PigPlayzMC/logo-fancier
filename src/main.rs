use std::{
    env,
    fs,
    process::exit,
};

fn main() {
   let args: Vec<String> = env::args().collect();
   let mut file_path: String = Default::default();

   // Char arrays (If ascii artifacts are present, this is probably the root cause)
   let solid_chars: Vec<char>  = vec!['M', 'o', 's', '+'];
   let half_right_chars: Vec<char> = vec!['/'];
   let disregarded_chars: Vec<char> = vec![' ', '`', '.'];

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
   let mut error_count: u32 = 0;
   let mut block_art: String = "".to_string();
   let mut skip_next_char: bool = false;
   let mut index = 0;

   for char in ascii_art.chars() {
       if !skip_next_char {
	   if char == ' ' {
	      block_art += " ";
	   } else if char == '\n' { // This is a new line
	     block_art += "\n";
	   } else if char == '$' {
	      // This is something to do with colour maybe?
	      skip_next_char = true;
	   } else if solid_chars.contains(&char) {
	      block_art += "█";
	   } else if half_right_chars.contains(&char) {
	     /* NOTE: This breaks... Check must be done to ensure the correct
	     element is chosen (Could be right or left)*/

	     let prior_char: char;
	     let next_char: char;

	     // Safety checks first
	     if index == 0 {
	     	prior_char = ' ';
	     } else {
	       	prior_char = ascii_art.chars().nth(index - 1).expect("Already checled for failure point");
	     };

	     if index == ascii_art.len() - 1 {
	     	next_char = ' ';
	     } else {
	       next_char = ascii_art.chars().nth(index + 1).expect("Already checked for failure point");
	     };

	     block_art += &pick_best_half_char(prior_char, next_char, &disregarded_chars);
	   } else {
	     error_occurred = true;
	     error_count += 1;
	     block_art += &char.to_string(); // This is not meant to happen, but might result in a more salvagable file
	     eprintln!("Nonfatal - unexpected char encountered...");
	   };
       } else {
       	 skip_next_char = false;
       };

       index += 1;
   };

   if error_occurred {
      eprintln!("Nonfatal - {} error(s) occured during processing, the result file may be inaccurate.", error_count);
   };
   println!("Processing complete! Saving not implemented...");

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

fn pick_best_half_char(prior_char: char, next_char: char, disregarded_chars: &Vec<char>) -> String {
   if disregarded_chars.contains(&prior_char) && !disregarded_chars.contains(&next_char) {
      return "▐".to_string()
   } else if !disregarded_chars.contains(&prior_char) && disregarded_chars.contains(&next_char) {
     return "▌".to_string()
   } else {
     eprintln!("Progress - Can't guess which half character to use, going left..."); // As good are guess as right
     return "▌".to_string()
   };

   unreachable!();
}