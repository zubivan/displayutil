*displayutil* is a command line tool for display managment on macOS built with Rust.

As the first feature it fights macOS forgeting your display layout everytime you connect that external monitors.

Usage
-----

``` sh
$ displayutil --arrangment save
Configuration is saved to '/Users/robot/.displayutil'.
```

``` sh
$ displayutil --arrangment restore
Configuration finished.
```

> Instead of typing in the whole `--arrangement` command you can use `-a` shorthand.

Roadmap
-----

- [x] Support saving and restoring arrangment of external monitors
- [ ] Support multiple configurations, i.e. office, home.
- [ ] Store and restore screen rotation information
- [ ] Run as a service and apply configuration to on a fly
- [ ] Expose more display settings
- [ ] GUI
