# Roadmap

Some goals for further versions and current development are listed below.
Please send an [e-mail](mailto:anton.gepting@gmail.com) or open an
[issue](https://github.com/AntonGepting/tmux-interface-rs/issues/new)
if any feature is missing or if you have a request, an improvment, an idea etc.


**tmux_project_cfg v1.0.0**

- [ ] Support all tmux subcommands
- [ ] Documentation
- [ ] Tests
- [ ] Improve output, return
- [ ] Freeze basic architecture
- [ ] Freeze API
- [ ] Add Travis CI all tmux releases test


**tmux_project_cfg v0.1.0**

- [ ] Support basic tmux subcommands (references:
[tmuxinator](https://github.com/tmuxinator/tmuxinator),
[libtmux](https://github.com/tmux-python/libtmux), rust crates using tmux)

Parsing objects and supported tmux variables:
- [ ] Parsed structures check Type
    - [ ] Session
    - [ ] Window
    - [ ] Pane
- [ ] Parsed structures check Option
    - [ ] Session
    - [ ] Window
    - [ ] Pane
- [ ] Parse all tmux variables
- [ ] Prepare documentation
- [ ] Prepare tests
- [ ] Function results and errors


**tmux_project_cfg v0.0.5**

- [ ] Documentation for all existing functionality and items
- [ ] All tmux functions output return in right way
- [ ] No panics, no unwrap in lib functions
- [ ] Error reporting information num, enum, string like in std
- [ ] Better names for tmux subcommands wrapper function arguments


**tmux_project_cfg v0.0.1**

- [ ] Prepare sources for publication on github.com
    - [ ] .clippy.toml
    - [ ] .rustfmt.toml
    - [ ] rust-toolchain
    - [x] .editorconfig
    - [x] .travis.yml
    - [x] LICENSE.md
    - [x] README.md
    - [x] ROADMAP.md
    - [x] CHANGELOG.md
- [ ] Prepare crate for publication on crates.io
    - [ ] Cargo.toml
    - [ ] Documentation for existing functionality


# Wishlist
- does `Option<bool>` as function arguments and structure fields make sense
- mb store `PathBuf` or other type for paths in parsed structures?
