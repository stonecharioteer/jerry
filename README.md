# Jerry - Move the Mouse to a Monitor

Jerry is used to move your mouse pointer to a particular monitor. You can either invoke jerry
through a CLI, or you can bind the CLI commands to shortcuts on your Window Manager.

Jerry only supports devices that use `xrandr` for now.

Download the binary from the releases and run `jerry --help` to learn how to use the tool.

## Usage
```
$ jerry --help

jerry 0.1.0
Jerry is a tool that I wrote to help me move my mouse to a specific monitor when I'm using a tiling window manager on
Linux. Qtile doesn't seem to move the mouse focus to a specific monitor when moving focus to a new monitor, that makes
dmenu stick to the original monitor, which is rather annoying

USAGE:
    jerry [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -V, --version        Prints version information
    -w, --wrap-around

OPTIONS:
    -d, --direction <direction>    Which direction you'd like to move your mouse to
    -m, --monitor <monitor>        Monitor name. Use a configuration file to map the monitors to the names
```
