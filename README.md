# WPC  
  
[![Build Status](https://travis-ci.org/jkotra/wpc.svg?branch=master)](https://travis-ci.org/jkotra/wpc)![](https://img.shields.io/github/languages/code-size/jkotra/wpc)

WPC *stands for* **W**all **P**aper **C**hanger.
  
WPC is a wallpaper changer for Windows/Linux. 

*optionally*, It can retrieve wallpapers from [Wallheaven.cc](https://wallhaven.cc/) and [Bing.com](https://www.bing.com/).  
  
# Usage  
  
```batch  
WPC 0.1.3
Jagadeesh K. <jagadeesh@stdin.top>
Wallpaper changer for Windows/Lniux

USAGE:
    wpc [FLAGS] --directory <directory> --interval <interval> --update <update>

FLAGS:
    -b, --bing          Bing Wallpaper of the Day.
    -D, --debug         show debug messages.
    -h, --help          Prints help information
    -l, --local         Offline Mode.
    -o, --only          Only use remotely downloaded wallpapers.
    -S, --startup       start WPC at startup.
    -V, --version       Prints version information
    -w, --wallheaven    wallheaven.cc

OPTIONS:
    -d, --directory <directory>    directory of wallpapers.
    -i, --interval <interval>      interval in Secs.
    -u, --update <update>          Update interval in Secs.

 ```  
  
# Example 
  
* Retrieve and use images from [wallheaven](https://wallhaven.cc/). *60sec* interval.

`./wpc --directory "/home/jojo/Pictures/wpc/" -w -u 60 -i 60`

  
* Use local folder with images.

`./wpc --directory "/home/jojo/Pictures/wpc/" -l -u 60 -i 60`
  
* [Bing](https://www.bing.com/) wallpaper of the day.  

`./wpc --directory "/home/jojo/Pictures/wpc/" -b -u 60 -i 60`


# Wallheaven API

Complete [wallheaven API](https://wallhaven.cc/help/api) is implemented in [api/wallheaven.rs](https://github.com/jkotra/wpc/blob/master/src/api/wallheaven.rs)

# Building  

`cargo build --release`  
  
Binary is located at `target/release/`
