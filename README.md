# WPC  
  
[![Build Status](https://travis-ci.org/jkotra/wpc.svg?branch=master)](https://travis-ci.org/jkotra/wpc) ![](https://img.shields.io/github/languages/code-size/jkotra/wpc)
![](https://img.shields.io/aur/version/wpc)

WPC *stands for* **W**all **P**aper **C**hanger.
  
WPC is a wallpaper changer for Microsoft Windows and Linux. 

*optionally*, It can retrieve wallpapers from [wallhaven.cc](https://wallhaven.cc/) and [subreddit](https://reddit.com) of your choice (for example: [/r/wallpaper](https://www.reddit.com/r/wallpaper/))

---

<details>
  <summary> Usage (<b>-h</b>) </summary>

```
WPC 1.8.0
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
        --trigger <trigger>                        Execute command on walpaper change [default: trigger.json]
    -u, --update <update>                          Update interval in Seconds. [default: 3600]
```
</details>

---

# Installation

## Building  

`cargo build --release`  

Binary will be located at `target/release/`

---

## How to use?

**WPC** is a command-line application i.e you need to run it from a command prompt or terminal.

## Example

| **Platform** |              **Command**              |
|--------------|:-------------------------------------:|
| Linux        | `./wpc -d . -i 60 -u 360 --startup`   |
| Windows      | `wpc.exe -d . -i 60 -u 360 --startup` |


The above command(s) will change wallpaper(that are located at `-d`) every 60 seconds, check for new images every 360 seconds, and add **WPC** to startup with the same settings.


## Dynamic Wallpaper

`--dynamic` option can be used to set dynamic wallpaper based on system time. the wallpaper is chosen from the provided `json` file.

<details>
  <summary> example <code>config.json</code> </summary>

```sh
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

</details>

---

## Trigger Command on Wallpaper Change

You can invoke a custom command on every wallpaper change with `--trigger` arg. chosen computed parameters (such as `Brightness`) and options (theme variants) will be passed to chosen command in the form of arguments.

<details>
  <summary> example <code>trigger.json</code> </summary>

**NOTE**: *Only* use complete paths. 

```json
{
	"enabled": true,
	"bin": "/usr/bin/python",
	"file": "/home/jkotra/playground.py",
	"args": ["Brightness", "Grayscale", "ThemeDarkOnly", "ThemeLightOnly"]
}
```

```py
import sys

print('Hello WPC')

with open("args.txt", "a+") as f:
    f.write(str(sys.argv))
    f.close()
```

</details>

---

# Web Plugins

| **Plugin** |                                                 **Example**                                                |
|------------|:----------------------------------------------------------------------------------------------------------:|
| Wallhaven  | `./wpc -d . -w`                                                                                            |
| Reddit     | `./wpc -d . --reddit art --reddit-n 10 --reddit-sort top --reddit-min-width 1920 --reddit-min-height 1080` |

---

# Misc.

## wallhaven API

Complete [wallhaven API](https://wallhaven.cc/help/api) is implemented in [api/wallhaven.rs](src/web/wallhaven_api.rs)

---

## Debug

| **Platform** |                                          **Command**                                          |
|------------|:----------------------------------------------------------------------------------------------------------:|
| Linux  | `RUST_LOG=DEBUG ./wpc -d  .`                                                                                            |
| Windows 10+ (PS)     | `$env:RUST_LOG = "DEBUG"` |