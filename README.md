# WPC  
  
[![Build Status](https://travis-ci.org/jkotra/wpc.svg?branch=master)](https://travis-ci.org/jkotra/wpc)![](https://img.shields.io/github/languages/code-size/jkotra/wpc)

WPC *stands for* **W**all **P**aper **C**hanger.
  
WPC is a wallpaper changer for Windows/Linux. 

*optionally*, It can retrieve wallpapers from [Wallheaven.cc](https://wallhaven.cc/) and [Bing.com](https://www.bing.com/).  
  
# Usage  
  
```

WPC 0.1.5
Jagadeesh K. <jagadeesh@stdin.top>
Wallpaper changer for Windows/Linux

USAGE:
    wpc [FLAGS] [OPTIONS] --directory <directory>

FLAGS:
    -b, --bing          Bing Wallpaper of the Day.
        --daemon        start as daemon.
    -D, --debug         show debug messages.
    -h, --help          Prints help information
    -l, --local         Offline Mode.
    -o, --only          Only use remotely downloaded wallpapers.
    -S, --startup       start WPC at startup.
    -V, --version       Prints version information
    -w, --wallheaven    wallheaven.cc

OPTIONS:
    -d, --directory <directory>    directory of wallpapers.
    -i, --interval <interval>      interval in Secs. [default: 60]
    -u, --update <update>          Update interval in Secs. [default: 120]

 ```  

---

# Wallheaven API

Complete [wallheaven API](https://wallhaven.cc/help/api) is implemented in [api/wallheaven.rs](https://github.com/jkotra/wpc/blob/master/src/api/wallheaven.rs)

# Building  

`cargo build --release`  
  
Binary will be located at `target/release/`
