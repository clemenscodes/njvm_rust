# NinjaVM in Rust

## Installation

### Prerequisites

    sudo apt update
    sudo apt upgrade
    sudo apt install git

### Install Rust Toolchain

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env

### Compile

    git clone https://github.com/clemenscodes/njvm.git
    cd njvm
    cargo install --path .

## Usage

    njvm [option] [option] ...

### Flags

    --prog1      select program 1 to execute 
    --prog2      select program 2 to execute 
    --prog3      select program 3 to execute 
    --version    show version and exit
    --help       show this help and exit

## Documentation

    cargo doc --workspace --no-deps --open
