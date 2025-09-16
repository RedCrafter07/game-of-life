# game-of-life Rust

This is my attempt on implementing Conway's game-of-life in Rust!

## Usage

There are multiple options available to run this.
First of all, get the file from the [releases](https://github.com/RedCrafter07/game-of-life/releases/latest).

- For Linux, you can get the file without a file type and chmod +x <file> it.
- For Windows, you can simply download the .exe file and drag it into your terminal

To execute, run the following:

```bash
file <rule> (start)
```

\<required> \(optional)

If you don't provide the start value, the program will prompt you to enter one. Please note that it has to start and end with a dot (.).

### Advanced usage - environment variables

#### Appearance

You can provide a custom appearance for living and dead cells:

- `DEAD_CELL="."` (has to be one character)
- `LIVING_CELL="x"` (has to be one character as well)

Note: On Windows, you have to append `$env:` in front of the variable names and then add a semicolon after each variable before entering the path to the software.

Linux example:

```bash
DEAD_CELL="." LIVING_CELL="x" file # ...
```

Windows example:

```bash
$env:DEAD_CELL="." ; $env:LIVING_CELL="x" ; file # ...
```

#### Iterations

You can additionally supply `ITERATIONS="<number>"` to specify how often the program should calculate the "next day", or simply the next iterations. By default, this value is 20.

## Legal notice

Copyright (c) 2025 RedCrafter07. All Rights Reserved.
