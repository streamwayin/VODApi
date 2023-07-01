# Setting up Cargo and Rust on Linux

This guide will walk you through the steps to set up Cargo and Rust on Linux.

## Prerequisites

Before starting, ensure that you have the following prerequisites installed:

- Linux operating system (e.g., Ubuntu, Fedora)
- Internet connection

## Installation Steps

Follow these steps to install Cargo and Rust:

1. Open a terminal on your Linux system.

2. Update the package manager and upgrade existing packages by running the following command:
   ```shell
   sudo apt update && sudo apt upgrade


3. sudo apt install build-essential

4. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

5. The script will prompt you to proceed with the installation. Press '1' and then press 'Enter' to proceed with the default installation.

6. Once the installation is complete, the script will display instructions to configure your current shell to use the newly installed Rust. Follow the instructions provided. For     example, if you're using Bash, you can run the following command:

    source $HOME/.cargo/env

7. rustc --version

8. cargo --version

9. go to the project and run the following comand for build a project:
   
   cargo build

10. Now for run the project:
    
    cargo run 


