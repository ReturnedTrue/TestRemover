# TestRemover
TestRemover (trmv) is a simple program to remove TestEZ files from your project and output a new directory.

TestRemover is written in the incredibly fast Rust programming language and hence is available for any 64-bit OS.

# Installation
Please see [here](/INSTALLATION.md) to see how you can install TestRemover

# Usage
Using TestRemover is easy, just run it in your project directory:
```
$ trmv
```
If you'd like to see what flags TestRemover offers, run it with -h or --help
```
$ trmv --help
```

# Configuration
TestRemover can be configured to your liking with a `trmv.toml` file in your project directory.

Below is what a default `trmv.toml` would look like, all properties are optional.
```toml
out = "out" # Out directory
source = "source" # Source directory
test_ending = ".spec.lua" # The suffix of the test files
verbose = false # If TestRemover should output more information, can be overriden by --verbose
```

# Thanks
I'd like to give a special thanks to all the members in the Rust community Discord server who helped with my issues.