# torpid
Tiny linux utility to turn CPU cores on/off

## Installation
cargo install --force torpid

## Usage
`sudo torpid <number of cores to enable>`. E.g: `sudo torpid 4`, will keep 4 cores on and turn others off.

Also see `torpid --help`, it should output something like:
```
torpid 0.1.0
Nipun Kumar <nipunkumar@outlook.com>
Turn off some CPU cores

USAGE:
    torpid <NUM_CPU_CORES>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <NUM_CPU_CORES>    Number of cores to enable. 
                       Must be within 1 <= ... <= number of cores present.
```
