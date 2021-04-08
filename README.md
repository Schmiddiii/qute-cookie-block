# qute-cookie-block
Blocking cookies for the qutebrowser.

# WARNING
This is currently very alpha software and almost no websites are supported.

# How it works
This userscript tries to block the cookies of a website by clicking on HTML elements, like buttons.
Note that this was designed for [qutebrowser](https://qutebrowser.org/) and will not work on any other browsers.

# Installation
## Prerequesites
- [rust](https://www.rust-lang.org/)
- [qutebrowser](https://qutebrowser.org/)
- A local copy of this repository.

## Linux
Just execute the `make.sh` in the repositories folder.

## Other
Run `cargo build` in the repositories folder. 
Copy the created binary from `target/debug/qute-cookie-block` to the userscripts directory of qutebrowser.
Copy the folder `cookie-blockers` to the data directory of qutebrowser.

# Running
Run this script by executing `:spawn --userscript qute-cookie-block` in qutebrowser.
As you probably don't want to type it out all the time, I recommend setting up a alias in `qute://settings`.

# Contributing
This userscript heavily depends on contributions to fill out the list of supported websites.
Please try to contribute by submitting pull requests.
Even if the rust code in this repository looks scary, I have tried to simplify the process of adding new websites.

To learn more about how to contribute, please look at the [wiki](https://github.com/Schmiddiii/qute-cookie-block/wiki) of this repository.
