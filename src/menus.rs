use std::process::exit;

pub fn help_menu() {
       println!("usage: logo-fancier [-h | --help] [-v | --version] [-g | --guess]
   [path] [<flags>]

Logo-Fancier: Make logos fancier...

To use: Run 'logo-fancier (path to your fetch program's ascii art that you wish to alter)
Alternative use: Run 'logo-fancier -g', logo-fancier will attempt to guess, based on your operating system and installed fetch program(s) which ascii art to use.
");

   exit(0);
}

pub fn version_info() {
    println!("Logo-Fancier version: {}", env!("CARGO_PKG_VERSION"));

    exit(0);
}
