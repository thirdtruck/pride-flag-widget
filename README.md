# pride-flag-widget

A TUI widget for displaying Pride flags on rotation.

Edit `flags.kdl` to change the rotation speed, defaults on name displays, and to change out flags!

# Usage

`cargo run`

## Keyboard Commands

+ `q` - Quit
+ `k` - Advance to the next flag
+ `f` - Toggle display of flag name
+ `c` - Toggle display of individual colors within a flag

### Flags

+ `t` - Switch to the Transgender Pride flag (index 0)
+ `b` - Switch to the Bisexual Pride flag (index 1)
+ `g` - Switch to the Genderqueer Pride flag (index 2)
+ `p` - Switch to the Pansexual Pride flag (index 3)

# Todo

+ Add support for more complex flags, e.g. Progress Pride flag
+ Add more flags
+ Add support for enabling/disabling flags in `flags.kdl` (instead of having to remove them entirely)
+ Add number key commands for switching to flags by index
+ Add alpha key commands for more flags
+ Add transition animations

