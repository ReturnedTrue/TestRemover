# Option 1
The easiest option is to install the binary from the [releases](https://github.com/ReturnedTrue/TestRemover/releases) page, I recommend installing the [latest](https://github.com/ReturnedTrue/TestRemover/releases/latest) version.

Then once installed, rename to `trmv` and add it to your PATH. Please search online if you are unaware of how.

# Option 2
Of course, not everyone is happy with installing binaries like so. Furthermore, you may install this with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Simply run install:
```
$ cargo install trmv
```

# Option 3
If you'd like another option, you can build TestRemover yourself with [git](https://git-scm.com/downloads) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

First of all, fetch it from the repo:
```
$ git fetch https://github.com/ReturnedTrue/TestRemover.git
```


Then move into it:
```
$ cd TestRemover
```

Now you can build!
```
$ cargo build --release
```
(Since --release is included it may take some time, however it will be optimised).

Finally, you can locate it at `/target/release/trmv`. Would recommend adding it to your PATH.