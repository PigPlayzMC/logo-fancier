// Note: Formatting in this file is unreliable as I only installed
// Rust-mode half way through and Fundamental mode behaved unexpectedly
// when faced with a rust file

use std::{
    env,
    fs,
    process::exit,
};

fn main() {
   let args: Vec<String> = env::args().collect();
   let mut file_path: String = Default::default();

   // Char arrays (If ascii artifacts are present, this is probably the root cause)
   let solid_chars: Vec<char>  = vec!['M', 'o', 's', '+', 'i', 'x', 'k', ';', 'j', 'I'];
    let half_width_chars: Vec<char> = vec!['/', ':'];
    let half_height_chars: Vec<char> = vec!['-']; // This is checked in a second pass as the chosen step char is used...
   let step_chars: Vec<char> = vec![]; // Archive, reason: Lazy
    let disregarded_chars: Vec<char> = vec![' ', '`', '.', ':', '\''];

    let combination_half: Vec<char> = vec!['\'', '`', '.', '-', ',']; // Any 2 of these will
    // be replaced with a half block

    let high_chars: Vec<char> = vec!['\'', '`'];
    let low_chars: Vec<char> = vec![',', '.']; // Not high_chairs

   if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
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

    let mut skip_next_char: bool = false;

   let mut error_occurred: bool = false;
   let mut error_count: u32 = 0;
   let mut block_art: String = "".to_string();
   let mut index = 0;

   for char in ascii_art.chars() {
       if !skip_next_char {
	   if char == ' ' {
	      block_art += " ";
	   } else if char == '\n' {
	     block_art += "\n";
	   } else if char == '$' {
	      // This is something to do with colour maybe?
	      skip_next_char = true;
	   } else if solid_chars.contains(&char) {
	      block_art += "█";
	   } else if half_width_chars.contains(&char) {

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

	     block_art += &pick_best_half_char(prior_char, next_char, &disregarded_chars, &combination_half);
	   } else if step_chars.contains(&char) {
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

	       block_art += &pick_best_step_char(prior_char, next_char, &half_height_chars);
	   } else if combination_half.contains(&char) {
	       let prior_char: char;
	       let next_char: char;

	       // Gets funky if run on edge characters so this aims to exclude them...
	       let prior_prior_char: char;
	       let next_next_char: char;

	       // Safety checks first (consider modularity)
	       // Considered: Lazy
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

	       // Check if the prior_char is a left bounding char
	       let bounding_char: bool;
	       if index == 1 {
		   bounding_char = true;
	       } else if ascii_art.chars().nth(index - 2).expect("Checked") == ' ' {
		   bounding_char = true;
	       } else {
		   bounding_char = false;
	       };

	       let run: bool;
	       if char == '\'' && (prior_char == ' ' || next_char == '\n') {
		   run = false;
	       } else {
		   run = true;
	       };

	       // Process right here, like a "man" (arch+rust btw)
	       if (combination_half.contains(&prior_char) || combination_half.contains(&next_char)) && run {
		   // Definitely add a half char, but which one

		   // Manual hack to get around some weird logic error
		   let mut cont = true;
		   if prior_char == '`' && high_chars.contains(&char) && next_char == '.' {
		       println!("Progress: Hacky workaround triggered...");
		       block_art += "▀";
		       cont = false;
		   };

		   // See definitions of low_chars and high_chars
		   if cont {
		       if high_chars.contains(&prior_char) && !bounding_char || high_chars.contains(&next_char) {
			   block_art += "▀";
		       } else if low_chars.contains(&prior_char) || low_chars.contains(&next_char) {
			   block_art += "▄";
		       } else if high_chars.contains(&char) {
			   block_art += "▀";
		       } else if low_chars.contains(&char) {
			   block_art += "▄";
		       } else {
			   // How has this happened???
			   error_occurred = true;
			   error_count += 1;
			   block_art += &char.to_string(); // See after...
			   eprintln!("Nonfatal - Could not decide which half character to use...");
		       };
		   };
	       } else {
		   error_occurred = true;
		   error_count += 1;
		   block_art += &char.to_string(); // See after...
		   eprintln!("Nonfatal - unexpected char encountered...");
	       };
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

fn pick_best_half_char(prior_char: char, next_char: char, disregarded_chars: &Vec<char>, combination_halfs: &Vec<char>) -> String {
    let mut new_disregarded_chars: Vec<char> = disregarded_chars.to_vec();
    new_disregarded_chars.push('-');
    
    if next_char == ':' || prior_char == ':' || combination_halfs.contains(&next_char) || combination_halfs.contains(&prior_char) { // This ends up looking better
	return "█".to_string()
    };
	
   if new_disregarded_chars.contains(&prior_char) && !new_disregarded_chars.contains(&next_char) {
      return "▐".to_string()
   } else if !new_disregarded_chars.contains(&prior_char) && new_disregarded_chars.contains(&next_char) {
     return "▌".to_string()
   } else { // This looks best imo
     return "█".to_string()
   };
}

fn pick_best_step_char(prior_char: char, next_char: char, half_height_chars: &Vec<char>) -> String {
    if prior_char == ' ' || prior_char == '\n' {
	return "🬵".to_string()
    } else if next_char == ' ' || next_char == '\n' {
	return "🬱".to_string()
    };
    
    if half_height_chars.contains(&prior_char) && !half_height_chars.contains(&next_char) {
	return "🬵".to_string()
    } else if !half_height_chars.contains(&prior_char) && half_height_chars.contains(&next_char) {
	return "🬱".to_string()
    } else {
	eprintln!("Progress - Can't guess which step character to use, going left");
	return "🬵".to_string()
    };
}
