## Installing Rust on Windows

### Step 1: Download and Run the Installer

1. Go to the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/).
1. Click on the "Install" button.
1. Download the Windows installer (usually a `.exe` file).

### Step 2: Run the Installer

4. Run the installer you downloaded.
4. Follow the installation prompts. You can choose the default settings for most options.

### Step 3: Verify the Installation

6. Open a Command Prompt or Powershell window.
6. Type `rustc --version` and press Enter. You should see the Rust version number, indicating a successful installation.

[Install Rust on Windows without Installing Visual C++ Build Tools](https://www.youtube.com/watch?v=92HoSWgsx-4)

## Installing Rust on Linux and Mac OS

### Step 1: Install Rust via Rustup (Recommended)

1. Open a Terminal.

1. Run the following command to download and install Rust using Rustup:

1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Step 2: Follow the Installation Prompts

3. Follow the prompts to customize your Rust installation if needed. The default options are usually fine.

### Step 3: Configure Rust Environment

4. After installation, Rustup will provide instructions for setting up your environment. Follow these instructions. It typically involves running a command like:
   
   bash

4. `source $HOME/.cargo/env`

### Step 4: Verify the Installation

5. In the Terminal, type `rustc --version` and press Enter. You should see the Rust version number, confirming a successful installation.
