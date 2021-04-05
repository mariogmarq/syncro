#!/bin/bash
# Install syncro
cd /tmp/ && git clone "https://github.com/mariogmarq/syncro"
cd syncro
cargo install --path .
