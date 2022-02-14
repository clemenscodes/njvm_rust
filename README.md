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

    njvm [options] <code file>

### Flags

    --debug      start virtual machine in debug mode
    --version    show version and exit
    --help       show this help and exit

## Debugger

To debug a ninja binary, pass ```--debug``` as an option.

The debugger has several features:

- Inspect:
  - Stack
  - Static Data Area
- List Instruction Register
- Step through the binary an instruction at a time
- Run the entire program until the end
- Set a breakpoint at which the running program will jump in debug mode
- Quit the VM

### Inspecting the VM

- ```inspect``` or simply ```i```.

  - ```stack``` or ```s``` to inspect the current stack.

  - ```data``` or ```d``` to inspect the current static data area.

### Listing the instruction register

- ```list``` or simply ```l```.

### Stepping through the binary

- ```step``` or simply ```s```.

### Running the binary

- ```run``` or simply ```r```.

### Setting a breakpoint

- ```breakpoint``` or simply ```b```.

- The debugger will expect an address of the instruction register at which the debugger should jump in.
- If the entered address is ```-1```, a currently existing breakpoint will be cleared.
Otherwise the entered address must be an unsigned integer.

### Quitting the VM

- ```quit``` or simply ```q```.

## Documentation

    cargo doc --open
