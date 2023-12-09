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
Usage: wpc [OPTIONS] --directory <DIRECTORY>

Options:
  -d, --directory <DIRECTORY>
          save / source directory.
  -c, --change-interval <c_seconds>
          interval between wallpaper change. [default: 300] [aliases: interval] [short aliases: i]
  -f, --fetch-interval <f_seconds>
          interval between each refresh from configures sources. [default: 3600] [aliases: update] [short aliases: u]
      --maxage <MAXAGE>
          maximum age of wallpaper. [default: -1]
  -s, --startup
          add WPC to startup. [short aliases: S]
      --rm-startup
          remove WPC from startup.
  -b, --background
          run WPC as background process.
      --set-theme
          
      --grayscale
          
      --force-dark-theme
          
      --theme-th <theme-brigness-threshold>
          [default: 50]
      --theme-dark-only
          
      --theme-light-only
          
      --trigger <TRIGGER_CONFIG_FILE>
          
  -w, --wallhaven
          wallhaven.cc plugin.
      --wallhaven-config <WALLHAVEN_CONFIG_FILE>
          wallhaven config file
  -r, --reddit
          reddit plugin.
      --subreddit <subreddit>
          [default: wallpaper]
      --reddit-n <reddit-n>
          [default: 6]
      --reddit-sort <reddit-sort>
          [default: hot] [possible values: hot, popular, new, top, rising]
      --reddit-min-height <reddit-min-height>
          [default: 1920]
      --reddit-min-width <reddit-min-width>
          [default: 1080]
  -l, --local
          Include only local files.
      --dynamic <DYNAMIC_CONFIG_FILE>
          Dynamically set wallpaper based on time.
  -h, --help
          Print help
  -V, --version
          Print version
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
| Linux        | `./wpc -d . -c 60 -f 360 --startup`   |
| Windows      | `wpc.exe -d . -c 60 -f 360 --startup` |


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
| Reddit     | `./wpc -d . --reddit --subreddit art --reddit-n 10 --reddit-sort top --reddit-min-width 1920 --reddit-min-height 1080` |

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