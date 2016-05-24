#!/usr/bin/env bash

# Install required packages.
echo 'Installing packages...'
pkg update &> /dev/null
pkg install -y 'fish' 'neovim' 'rust' 'cargo' &> /dev/null

# Install Rustup.
#echo 'Installing Rustup...'
#curl https://sh.rustup.rs -sSf | sh &> /dev/null
#export PATH="$HOME/.cargo/bin:$PATH"

# Install stable and nightly Rust.
#echo 'Installing stable and nightly Rust...'
#rustup update 'stable' &> /dev/null
#rustup update 'nightly' &> /dev/null
#
#echo 'Setting default toolchain to stable...'
#rustup default 'stable'
