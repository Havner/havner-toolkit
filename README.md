# OpenAction Uinput Simulation plugin

This is a simple [OpenDeck](https://github.com/nekename/OpenDeck) plugin for
simulating keyboard and mouse events using
[uinput](https://www.kernel.org/doc/html/latest/input/uinput.html).

## Actions

- Simulate Input

## Requirements

For this plugin to work `/dev/uinput` needs to exist and you need write access
to it. It is entirely possible you already have that, either through your
distribution provided configuration or some software that installed udev
rules. If not download [this](udev/42-uinput-uaccess.rules) file, put it in
`/etc/udev/rules.d` and restart.

See [here](TECHNICAL.md) for more details (e.g. how it differs from OpenDeck
provided input simulation plugin).

## Usage

The usage of this plugin is very similar to OpenDeck input simulation
plugin. See the plugin's page within OpenDeck for details.

# Acknowledgements

This plugin is based on Nekename's
[starterpack](https://github.com/nekename/OpenDeck/tree/main/plugins/com.amansprojects.starterpack.sdPlugin).

It also borrows agent idea (Tokens) from
[Enigo](https://github.com/enigo-rs/enigo/blob/main/src/agent.rs).
