# Logo Fancier
<p align="center">
<img width="325" height="302" src="https://github.com/user-attachments/assets/acadc57b-e7d3-4796-b393-11b7ffe04636" alt="Artix logo in block characters"/><img width="299" height="258" alt="elementary_blocks" src="https://github.com/user-attachments/assets/969be146-5463-476e-810b-4b3f5885b453" />
</p><br>

Convert distro ASCII art to Unicode block characters for fancier display in your prefered fetch program.

# Use
*This message is avaliable in the program though the use of -h or --help.*<br>
usage: logo-fancier [-h | --help] [-v | --version] [-g | --guess]<br>
&emsp;&emsp;   [path] [\<flags\>]

To use: Run 'logo-fancier (path to your fetch program's ascii art that you wish to alter)
Alternative use: Run 'logo-fancier -g', logo-fancier will attempt to guess, based on your operating system and installed fetch program(s) which ascii art to use.

# Caveats
Please be aware, the program is a mess and is in very early development, and therefore has many bugs, especially where ascii art uses many '-' characters. I hope to improve this
but at present, this is not an ultimately reliable program. Even during successful operation, you may need to manually edit the output, though hopefully less than doing the entire conversion yourself.

## Planned (unimplemented) features
- Ability to halt early in the processing stage to maximise customisability
- Guess mode to reduce unneeded inputs
- Better support for '-' characters
- Rewrite to account for two dimensions in logic decisions, rather than relying on one dimension and hope

# Further examples
<img width="282" height="243" alt="archstrike_blocks" src="https://github.com/user-attachments/assets/4e3a72d5-fb02-40be-a4ed-6bae32a7ceb8" />
<img width="353" height="285" alt="amazon_blocks" src="https://github.com/user-attachments/assets/815422af-0fd4-4326-b32e-410e29d3d602" />
<img width="327" height="301" alt="alpine_blocks" src="https://github.com/user-attachments/assets/fdb61e00-767e-4f80-9d01-db7772fb9a6f" />

