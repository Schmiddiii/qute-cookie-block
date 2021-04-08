#!/bin/bash

cargo build
cp target/debug/qute-cookie-block ~/.local/share/qutebrowser/userscripts/
cp -r cookie-blockers ~/.local/share/qutebrowser/
