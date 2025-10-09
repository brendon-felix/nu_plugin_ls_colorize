# nu_plugin_ls_colorize

A simple plugin for [Nushell](https://nushell.sh/) which colorizes the input path

## Installing

```nushell
> cargo install --path .
```

## Usage

```nushell
> plugin add ~/.cargo/bin/nu_plugin_ls_colorize
> plugin use ls_colorize
> ls | get name | ls-colorize
```
