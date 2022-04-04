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