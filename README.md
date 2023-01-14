# ducks

A simple program in Rust to find fat files and folders in your directories.

```sh
❯ ducks ~ 15
/home/francesco/Downloads                             7.3 GiB
/home/francesco/data                                  2.6 GiB
/home/francesco/bin2                                  1.2 GiB
/home/francesco/Documents_noTM                        168.2 MiB
/home/francesco/Wallpapers                            30.2 MiB
/home/francesco/docker_vol_exports                    30.1 MiB
/home/francesco/.cache                                11.9 MiB
/home/francesco/Documents                             2.5 MiB
/home/francesco/.zcompcache                           1.2 MiB
/home/francesco/.gnupg                                459.1 KiB
/home/francesco/.lyx                                  372 KiB
/home/francesco/.nvm                                  237.3 KiB
/home/francesco/.zsh_history                          226.6 KiB
/home/francesco/.pyenv                                188.5 KiB
/home/francesco/.fzf                                  131.2 KiB
```

NB: I've just started learning Rust and this may not be the best Rust code you've ever read. It needs tons of optimizations that I'll do as I get better at it. Please don't take this code as an example!

## Get it

Build, add to `PATH`, and run it. Binary releases coming soon.

## Usage

```sh
ducks DIR [LIMIT]
```

`DIR`, required.

`LIMIT` defaults to 10.

### Examples

```sh
ducks /some/dir
ducks . 20
```

## Build

```sh
cargo build
```
