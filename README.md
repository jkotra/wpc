# WPC  
  
[![Build Status](https://travis-ci.org/jkotra/wpc.svg?branch=master)](https://travis-ci.org/jkotra/wpc)![](https://img.shields.io/github/languages/code-size/jkotra/wpc)

WPC stands for **W**all **P**aper **C**hanger  
  
WPC is a wallpaper changer for Windows/Linux. It can retrieve wallpapers from [wallheaven](https://wallhaven.cc/) and [Bing](https://www.bing.com/) *optionally*.  
  
# Usage  
  
```batch  
WPC 0.1.0  
Jagadeesh K. <jagadeesh@stdin.top>  
Wall Paper Changer  
  
USAGE:  
 wpc.exe [FLAGS] [OPTIONS] --directory <directory> --interval <interval> --update <update>  
FLAGS:  
 -b, --bing       Bing Wallpaper of the Day. -D, --debug      show debug messages. -h, --help       Prints help information -l, --local      Offline Mode. -V, --version    Prints version information  
OPTIONS:  
 -d, --directory <directory>                directory of wallpapers. -i, --interval <interval>                  interval in Secs. -u, --update <update>                      Update interval in Secs. --wh_id <wallheaven_id>                Collection ID. --wh_username <wallheaven_username>    Wallheaven username.
 ```  
  
# Example  
  
* Retrieve and use images from [wallheaven](https://wallhaven.cc/).
  
`./wpc.exe -u 10000 -i 10 -d C:\Users\thanos\Pictures\wh\ --wh_id 648286 --wh_username th4n0s`  
  
* Use local images from folder  .
  
`./wpc.exe --local -u 10000 -i 10 -d C:\Users\thanos\Pictures\`  
  
* [Bing](https://www.bing.com/) wallpaper of the day.  
  
`./wpc.exe --b -u -1 -i -1 -d C:\Users\thanos\Pictures\bing\`  

# Wallheaven API

Complete [wallheaven API](https://wallhaven.cc/help/api) is implemented in [api/wallheaven.rs](https://github.com/jkotra/wpc/blob/master/src/api/wallheaven.rs)

Feel free to use it in any of your projects.

# Building  
  
* Windows and Linux  
  
`cargo build --release`  
  
Binary is located at `target/release/`
