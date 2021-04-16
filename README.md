# qute-cookie-block
Blocking cookies for the qutebrowser.

# How it works
This userscript tries to block the cookies of a website by clicking on HTML elements, like buttons.
To support many websites, it also uses custom blocklist made for Chrome/Firefox plugins like [ublock origin](https://ublockorigin.com/).
Look at `Installation of a blocklist` for more details.

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

You can also take the binary and folder from the `Releases` tab, but they might outdated.

## Installation of a blocklist
This program supports blocklists/filterlists, that are written for [ublock origin](https://ublockorigin.com/).
To do that, download the blocklist you want (I recommend [this one](https://github.com/easylist/easylist/blob/master/easylist_cookie/easylist_cookie_general_hide.txt)), rename it to `blocklist.txt` (important!) and put it in your qutebrowser data directory under `cookie-blockers` in (`~/.local/qutebrowser/` in Linux).

Note that currently only very basic features of the blocklists are supported.

# Running
Run this script by executing `:spawn --userscript qute-cookie-block` in qutebrowser.
As you probably don't want to type it out all the time, I recommend setting up a alias in `qute://settings`.

# Contributing
This userscript heavily depends on contributions to fill out the list of supported websites.
Please try to contribute by submitting pull requests.
You do not even have to write any code, just XML.

To learn more about how to contribute, please look at the [wiki](https://github.com/Schmiddiii/qute-cookie-block/wiki) of this repository.
