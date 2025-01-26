# 1.7.0

- [**NEW**] Dynamic wallpaper

# 1.6.1

- Fix Dark Theme Wallpaper Setting on Gnome >= 43

# 1.6.0

- [**NEW**] set theme based on wallpaper brightness.
- [**FIX**] [WINDOWS] adding to startup no longer requires admin privileges.
- [**FIX**] [LINUX] fix padding issue in systemd unit file.

# 1.5.0

---

* [**NEW**] use systemd unit for startup on linux.
* [**FIX**] crash on gnome-shell < 42.

# 1.4.0

---

* [**NEW**] commandline args are now parsed into struct (`settings.rs`), clean up `main.rs`
* [**NEW**] Enable `LTO` for release build, resulting in reduced binary size.
* [**NEW**] Unified function call for changing the wallpaper and adding to startup defined in `changer/mod.rs`
* [**REFACTOR**] simplify `main.rs`.
* [**REFACTOR**] divide plugins into `web` and `changer`.
* [**REFACTOR**] `wallhaven.rs` API calls are now asynchronous.
* [**FIX**] possible crash on `wallhaven` use in debug build. caused due to `reqwest` blocking API which has now been replaced with default async.
* [**DEPRECATED**] `--rm-grayscale` - grayscale images are now stored in `tmp` and converted on every change.
* [**DEPRECATED**] `--bing` - unreliable API. use other alternatives.

# version = "1.9.4"

---

* [**FIX**] fix startup flag `-S`.