# Installation instructions

We highly recommend installing all the prerequisites in advance.

- git
- rust toolchain manager [rustup](https://rustup.rs/)
- rust stable toolchain (installed through rustup)
- An IDE to write your code in
  - We recommend Visual Studio Code (with rust-rls)
  - Alternatively, CLion is a great option (with JetBrains’ Rust-plugin).

Should you run into any trouble, please don’t hesitate to contact us in advance to help you get up running.

## Windows

### Visual Studio Code (VSCode):

For reference: https://www.brycevandyk.com/debug-rust-on-windows-with-visual-studio-code-and-the-msvc-debugger/

- Install git bash for windows.
  - [https://git-scm.com/downloads](https://git-scm.com/downloads)
- Install [https://rustup.rs/](https://rustup.rs/)
- Install Visual Studio Code
  - [https://code.visualstudio.com/download](https://code.visualstudio.com/download)
- Install C/C++ toolchain for VSCode
  - [https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)
- Install the rust-rls-plugin for VSCode
  - Visit extensions within VSCode and search for `Rust (rls)`

### Alternatively, you could use Visual Studio

For reference: https://www.jonathanturner.org/2017/03/rust-in-windows.html

### Windows 10

We have gotten a few reports in the past that the above instructions aren't always enough.
If you are having problems, check out the following instructions:

- [http://asyncbulbs.blogspot.com/2017/06/workaround-for-rustc-with-new-visual.html](http://asyncbulbs.blogspot.com/2017/06/workaround-for-rustc-with-new-visual.html)

## macOS

- Install [https://rustup.rs/](https://rustup.rs/)
- Install Visual Studio Code
  - `brew cask install vscode`
  - Alternatively: https://code.visualstudio.com/download
- Install the rust-rls-plugin for VSCode
  - Visit extensions within VSCode and search for `Rust (rls)`

### Alternatively

- CLion (JetBrains-product)
  - You will need xcode available for C/C++ toolchain. You can enable it by running the following statement in your terminal.
  - `xcode-select --install`
  - [https://www.jetbrains.com/clion/](https://www.jetbrains.com/clion/)
  - Install the official Rust-plugin available from the Marketplace
    `Clion > Preferences > Plugins > Search for Rust`

## Linux

- Install [https://rustup.rs/](https://rustup.rs/)
- Install Visual Studio Code
  - [https://code.visualstudio.com/download](https://code.visualstudio.com/download)
  - Admittedly, there are probably tons of ways of doing this.
  - Install the rust-rls-plugin for VSCode
    - Visit extensions within VSCode and search for `Rust (rls)`

### Alternatively

- CLion (JetBrains-product)
  - https://www.jetbrains.com/clion/
  - Install the official Rust-plugin available from the Marketplace
    `Clion > Preferences > Plugins > Search for Rust`
