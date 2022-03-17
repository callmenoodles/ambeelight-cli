# Ambeelight
Cross-platform ambilight for Yeelight.

## Platforms
- Windows
- Linux
### macOS
macOS is not supported yet, because it relies on `captrs` which does not support macOS yet either.

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
