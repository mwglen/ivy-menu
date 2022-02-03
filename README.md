# Ivy-Menu
## A wrapper for controlling ivy like dmenu

## Features:
- [x] Basic dmenu like functionality 
    - [x] Captures input from stdin
    - [x] Outputs selection to stdout
    - [x] Works with the pipe operator
    - [x] Accepts all dmenu arguments
- [ ] Functionality for all dmenu arguments
  - [ ] Force menu to bottom of screen with "-b"
  - [ ] Grab the keyboard before reading stdin with "-f"
  - [ ] Match case insensitively with "-i"
  - [ ] Set number of lines with "-l"
  - [ ] Choose monitor to display menu on with "-m"
  - [x] Customizable prompt with "-p"
  - [ ] Choose font with "-fn"
  - [ ] Choose normal background color with "-nb"
  - [ ] Choose normal foreground color with "-nf"
  - [ ] Choose selected background color with "-sb"
  - [ ] Choose selected foreground color with "-sf"
  - [ ] Embed into windowid with "-w"

## Examples:
Basic Usage: `echo "Option 1\nOption 2\n Option 3" | ivy-menu -p "Choose an Option: "`
