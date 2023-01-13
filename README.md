# WPC  
  
[![Build Status](https://travis-ci.org/jkotra/wpc.svg?branch=master)](https://travis-ci.org/jkotra/wpc) ![](https://img.shields.io/github/languages/code-size/jkotra/wpc)

WPC *stands for* **W**all **P**aper **C**hanger.
  
WPC is a wallpaper changer for Microsoft Windows and Linux. 

*optionally*, It can retrieve wallpapers from [wallhaven.cc](https://wallhaven.cc/) and [subreddit](https://reddit.com) of your choice (for example: [/r/wallpaper](https://www.reddit.com/r/wallpaper/))

---

# Install

[Available on AUR](https://aur.archlinux.org/packages/wpc)

## Building  

`cargo build --release`  

Binary will be located at `target/release/`

---

# Usage  
  
```
WPC 1.7.0
Jagadeesh K. <jagadeesh@stdin.top>
Wallpaper changer for Windows/Linux

USAGE:
    wpc [FLAGS] [OPTIONS] --directory <directory>

FLAGS:
        --background     Run WPC in background.
        --grayscale      convert image to grayscale.
    -h, --help           Prints help information
    -l, --local          Include only local files.
        --set-theme      set light / dark theme based on the brightness of the wallpaper.
    -S, --startup        start WPC at startup.
        --theme-dark     Only set dark theme compatible (set by theme-threshold) wallpapers.
        --theme-light    Only set light theme compatible (set by --theme-threshold) wallpapers.
    -V, --version        Prints version information
    -w, --wallhaven      wallhaven.cc plugin.

OPTIONS:
    -d, --directory <directory>                    directory of wallpapers.
        --dynamic <dynamic>                        Dynamically set wallpaper based on time. [default: config.json]
    -i, --interval <interval>                      interval in Seconds. [default: 300]
        --maxage <maxage>                          maximum age of wallpaper in Hours(h). [default: -1]
    -r, --reddit <reddit>                          Reddit subreddit (/r/something) [default: wallpaper]
        --reddit-min-height <reddit-min-height>    Image.height >= reddit-min-height [default: 1080]
        --reddit-min-width <reddit-min-width>      Image.width >= reddit-min-width [default: 1920]
        --reddit-n <reddit-n>                      no. of images to download from subreddit. [default: 1]
        --reddit-sort <reddit-sort>                Reddit sorting order. [ Hot, New, Top, Rising ] [default: hot]
        --theme-threshold <theme-threshold>        brightness threshold to determine theme [0 - 100] [default: 50]
    -u, --update <update>                          Update interval in Seconds. [default: 3600]
```

## How to use?

**WPC** is a command-line application i.e you need to run it from a command prompt or terminal.

1. *cd* to the directory which contains wpc executable.
2. command line applications are launched using the prefix `./` on linux.
3. edit and play with various options to your liking.

## Example


### Linux

`./wpc -d . -i 60 -u 360 --startup`


### Windows (10+)

`wpc.exe -d . -i 60 -u 360 --startup`


The above command(s) will change wallpaper(that are located at `-d`) every 60 seconds, check for new images every 360 seconds, and add **WPC** to startup with the same settings.

*(Tested on Windows 10)*


## Dynamic Wallpaper

`--dynamic` option can be used to set dynamic wallpaper based on system time. the wallpaper is chosen from the provided `json` file.

**example**:

```
./wpc -d . --dynamic ~/Pictures/Fluent/config.json
```

`config.json`:

```json
{
  "configs": [
    {
      "hour": 0,
      "path": "Fluent-2.jpg",
      "darkmode": false
    },
    {
      "hour": 11,
      "path": "Fluent-1.jpg",
      "darkmode": false
    },
    {
      "hour": 16,
      "path": "Fluent-2.jpg",
      "darkmode": true
    },
    {
      "hour": 18,
      "path": "Fluent-3.jpg",
      "darkmode": true
    }
  ]
}
```

**Note**: Wallpaper hour is evaluated from 00 (Midnight). Make sure to edit your config accordingly. 

---

# Web Plugins

### wallhaven.cc

`./wpc -d . -w`

The program will run setup wizard if `wallhaven.json` file is not found.enter your username, collection ID and API key(required only for private collections).


### Reddit.com

```
./wpc -d . --reddit {subreddit_name} --reddit-n {quantity} --reddit-sort {hot|new|rising|top} --reddit-min-width 1920 --reddit-min-height 1080
```
- Reddit example:

```
./wpc -d . --reddit art --reddit-n 10 --reddit-sort top --reddit-min-width 1920 --reddit-min-height 1080
```

---

# wallhaven API

Complete [wallhaven API](https://wallhaven.cc/help/api) is implemented in [api/wallhaven.rs](src/web/wallhaven_api.rs)

---

# Debug

use `RUST_LOG={LEVEL}` as prefix.

example: `RUST_LOG=debug ./wpc -d .`

on windows / powershell: `$env:RUST_LOG = "DEBUG"`