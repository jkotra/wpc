# WPC  
  
[![Build Status](https://travis-ci.org/jkotra/wpc.svg?branch=master)](https://travis-ci.org/jkotra/wpc)![](https://img.shields.io/github/languages/code-size/jkotra/wpc)

WPC *stands for* **W**all **P**aper **C**hanger.
  
WPC is a wallpaper changer for Windows/Linux. 

*optionally*, It can retrieve wallpapers from [wallhaven.cc](https://wallhaven.cc/) and [Bing.com](https://www.bing.com/).  
  
# Usage  
  
```

WPC 0.7.0
Jagadeesh K. <jagadeesh@stdin.top>
Wallpaper changer for Windows/Linux

USAGE:
    wpc [FLAGS] [OPTIONS] --directory <directory>

FLAGS:
        --background    Run WPC in background.
    -b, --bing          Bing Wallpaper of the Day.
    -D, --debug         show debug messages.
    -h, --help          Prints help information
    -l, --local         Offline Mode.
    -o, --only          Only use remotely downloaded wallpapers.
    -S, --startup       start WPC at startup.
    -V, --version       Prints version information
    -w, --wallhaven    wallhaven.cc

OPTIONS:
    -d, --directory <directory>    directory of wallpapers.
    -i, --interval <interval>      interval in Secs. [default: 60]
        --maxage <maxage>          maximum age of wallpaper in hrs. [default: -1]
    -u, --update <update>          Update interval in Secs. [default: 120]


 ```

## How to use?

**WPC** is a command-line application i.e you need to run it from a command prompt or terminal.

1. *cd* to the directory which contains wpc executable.
2. command line applications are launched using the prefix `./` on linux.

`./wpc` in linux and `wpc` or `./wpc.exe` in windows.

3. edit and play with various command line arguments to your liking.

## Example

```
#linux

./wpc -d ~/Documents/wpc_test/ -w -i 60 -u 360 --startup

#windows

wpc.exe -d D:/Pics/ -w -i 60 -u 360 --startup

```

The above command will change wallpaper(that are located at `-d`) every 60 seconds, check for new images every 360 seconds, and add **WPC** to startup with the same settings.


---

# wallhaven API

Complete [wallhaven API](https://wallhaven.cc/help/api) is implemented in [api/wallhaven.rs](src/web/wallhaven_api.rs)

# Building  

`cargo build --release`  
  
Binary will be located at `target/release/`
