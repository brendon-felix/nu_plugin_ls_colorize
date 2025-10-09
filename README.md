# nu_plugin_ls_colorize

A simple plugin for [Nushell](https://nushell.sh/) which colorizes the input path

## Installing

```nushell
cargo install nu_plugin_ls_colorize
plugin add ~/.cargo/bin/nu_plugin_ls_colorize
```

## Usage

Colorize a single path
```nushell
let colorized = $env.PWD | ls-colorize
$colorized | debug -v
#
# "\u{1b}[38;5;81m/current/working/directory\u{1b}[0m"
```

Colorize a list of paths
```nushell
cd nu_plugin_ls_colorize
let colorized = ls | get name | ls-colorize
$colorized | debug -v
#
# ╭──────────────────────────────────────────────╮
# │ "\u{1b}[38;5;243mCargo.lock\u{1b}[0m"        │
# │ "\u{1b}[38;5;149mCargo.toml\u{1b}[0m"        │
# │ "\u{1b}[38;5;249mLICENSE\u{1b}[0m"           │
# │ "\u{1b}[48;5;186;38;5;16mREADME.md\u{1b}[0m" │
# │ "\u{1b}[38;5;81msrc\u{1b}[0m"                │
# │ "\u{1b}[38;5;81mtarget\u{1b}[0m"             │
# ╰──────────────────────────────────────────────╯
```

Get ansi colors instead
```nushell
cd nu_plugin_ls_colorize
ls | get name | ls-colorize --get-color
#
# ╭──────────────────┬─────────────────╮
# │        fg        │       bg        │
# ├──────────────────┼─────────────────┤
# │ grey46           │       ❎        │
# │ darkolivegreen3c │       ❎        │
# │ grey70           │       ❎        │
# │ grey0            │ lightgoldenrod2 │
# │ steelblue1b      │       ❎        │
# │ steelblue1b      │       ❎        │
# ╰──────────────────┴─────────────────╯
```

Get ansi colors for a custom LS_COLORS string generated using [vivid](https://github.com/sharkdp/vivid)
```nushell
$env.LS_COLORS = (vivid generate zenburn)
cd nu_plugin_ls_colorize
ls | get name | ls-colorize --get-color
#
# ╭─────────┬─────────┬──────╮
# │   fg    │   bg    │ attr │
# ├─────────┼─────────┼──────┤
# │ #7e7e7e │   ❎    │  ❎  │
# │ #e8bc92 │   ❎    │  ❎  │
# │ #878787 │   ❎    │  ❎  │
# │ #393939 │ #e8bc92 │  ❎  │
# │ #70a2d1 │   ❎    │ u    │
# │ #70a2d1 │   ❎    │ u    │
# ╰─────────┴─────────┴──────╯
```
