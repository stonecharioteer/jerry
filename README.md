# Jerry - Move the Mouse to a Monitor

Jerry is used to move your mouse pointer to a particular monitor. You can either invoke jerry
through a CLI, or you can bind the CLI commands to shortcuts on your Window Manager.

Jerry supports Linux, macOS, and Windows monitor backends.

Download the binary from the releases and run `jerry --help` to learn how to use the tool.

## Platform Support
- Linux: monitor enumeration through `xrandr` (requires `xrandr` and `xdo` development libraries at build time).
- macOS: monitor enumeration through Quartz Display Services.
- Windows: monitor enumeration through Win32 display APIs.

Linux package requirements (Debian/Ubuntu):
```bash
sudo apt-get install -y libxrandr-dev libxdo-dev
```

## Usage
```
$ jerry --help

jerry 0.1.0
Jerry moves your mouse pointer to a specific monitor or in a direction across monitors

USAGE:
    jerry [FLAGS] [OPTIONS]

FLAGS:
    -a, --animate-mouse  Jiggle the pointer around the target point before final placement
    -h, --help           Prints help information
    -s, --scroll-wheel   Scroll down and up once after moving the pointer
    -V, --version        Prints version information
    -w, --wrap-around

OPTIONS:
    -d, --direction <direction>    Which direction you'd like to move your mouse to
    -m, --monitor <monitor>        Monitor name. Use a configuration file to map the monitors to the names
```
