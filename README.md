# Ambeelight
A cross-platform ambient lighting solution for Yeelight devices. The program will update Yeelight devices to display the color of your computer's display output creating an amazing atmosphere when watching videos or gaming.

## Table of Contents
- [Platforms](Platforms)
- [Installation](Installation)
    - [Build from source](Build-from-source)
    - [Download binary](Download-binary)
- [Usage](Usage)
    - [CLI](CLI)
    - [IP](IP)
    - [Host](Host)
- [Contribution](Contribution)
- [License](License) 

## Platforms
- [x] Windows
- [x] Linux
- [ ] macOS ([captrs](https://crates.io/crates/captrs) doesn't support macOS)

## Installation
### Build from source
In order to build on *debian-based* distros, make sure you have the build dependency `libxrandr-dev` package installed.
```
sudo apt install -y libxrandr-dev
```

Clone the repo with and install the binary through cargo. The binary will be installed at `~/.cargo/bin` and can be unsinstalled with `cargo uninstall`
```
git clone git@github.com:callmenoodles/ambeelight.git
cargo install --path .
```

### Download Binary
Download and install the binary for your platform from [Releases.](https://github.com/callmenoodles/ambeelight/releases)

## Usage
### CLI
```
USAGE:
    ambeelight [OPTIONS] --ip <IP> --host <HOST>

OPTIONS:
    -b, --brightness <BRIGHTNESS>    Yeelight brightness between 1-100
    -h, --help                       Print help information
        --host <HOST>                IP address of the host [env: AMBEELIGHT_HOST_IP=]
    -i, --interval <INTERVAL>        Time between screen reads in milliseconds [default: 250]
        --ip <IP>                    IP address of the Yeelight [env: AMBEELIGHT_YEELIGHT_IP=]
    -t, --transition <TRANSITION>    Transition duration in milliseconds [default: 250]
    -V, --version                    Print version information
```
### IP
Find your Yeelight's IP address in the Yeelight app under <Your Lamp> -> Settings -> Device Info

### Host
Usually your local network IP address.

## Contribution
Feel free to contribute by sending pull requests, opening issues for bugs or feature requests or by fixing issues.

## License
[MIT](LICENSE)
