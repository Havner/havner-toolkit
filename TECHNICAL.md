# Why?

Why does it exist when OpenDeck already has input simulation? OpenDeck's [input
simulator](https://github.com/nekename/OpenDeck/blob/main/plugins/com.amansprojects.starterpack.sdPlugin/src/input_simulation.rs)
uses [Enigo](https://github.com/enigo-rs/enigo) for its implementation. While
the library is cross-platform and generally nice to use, it has some issues
under Linux. Some of those are library's bugs, most are due to the fact the
Linux graphical environments are very fragmented.

The library provides several backends to work under Linux. And unfortunately
none of those are bullet proof. Some require specific wayland protocols
implemented (`libei` works under Gnome, but `wayland` doesn't, for Hyprland it's
the other way around). Some of those backends when run under Wayland require
support for xdg-desktop protocol for Remote Desktop. Those are only supported by
big DEs (Gnome, KDE) and they require a confirmation every time OpenDeck is
started (sometimes even more often). And there is also no support for token that
would remember this decision currently.

Long story short there is no backed that "just works". Always, without a
hiccup. And here comes `uinput`. It's a kernel level input simulation mechanism
that works always, for every graphical environment (be it X11 or any Wayland
compositor). Even in the Linux text console.

There are two major differences from Enigo. There is a reason modern Wayland
environments don't allow input simulation just like that (usually) and the
Remote Desktop portal asks for permission. Safety. Uinput safety is done on a
level of `/dev/uinput` device. You need write access to it. This isn't anything
extraordinary as lots of software actually use it
([Solaar](https://github.com/pwr-Solaar/Solaar) and
[StreamController](https://github.com/StreamController/StreamController) on top
of my head).

This is usually done through uaccess udev rule. An example one can be found
[here](udev/42-uinput-uaccess.rules). This rule will automatically give access
to the device to users that have a physical seat to the computer upon logging
in. No other users (remote) will. This is actually the same mechanism that gives
users access to `/dev` files for stream deck in OpenDeck. In any case, this is
something to keep in mind if that is an issue for you.

The second difference from Enigo is that Enigo works on keysyms level. It
"thinks" in symbols. You need to press 'k', fine. You need to press 'K',
fine. And you want to press '@', that's also fine. But there is no key on a
keyboard that produces either capital 'K' or '@'. That's what keymaps are
for. They translate keysyms (symbols) into keycodes (physical keys). The char
'@' can be achieved in different ways on different keyboard mappings. Enigo will
handle that automatically. Uinput doesn't know anything about keysyms or
keymaps. It "thinks" purely with keys (keycodes). You want 'k', you press
'k'. You want capital 'K', you press Shift+k. You want '@', you press Shift+2 in
case that is how your keymap is configured or something else if you have a
different keymap. Uinput doesn't know about that, it just presses keys. On an
upside it's actually easier to work with uinput when you just need to map
keyboard shortcuts (as is my main use case here).

# How?

To compile the plugin simply run:

```bash
./release.sh
```

In the checked out sources. Providing you have rust installed properly this will
create `com.havner.uinput.sdPlugin` directory. Put it in
`~/.config/opendeck/plugins` and restart OpenDeck.

Make sure you have write access do `/dev/uinput`. Either through the rule
[here](udev/42-uinput-uaccess.rules) (put the file in `/etc/udev/rules.d` and
restart), or by giving it yourself in any other way.

The plugin usage is very similar to OpenDeck's input simulator. On the plugin
page within OpenDeck there is a simple help text with links to keycodes and
possible tokens.
