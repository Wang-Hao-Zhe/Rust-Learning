#!/bin/bash
cd ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html
python3 -m http.server 8000 --bind 127.0.0.1