
---
"libappindicator": minor
"libappindicator-sys": minor
---

Load exclusively using dynamic linking

This change lets `dlopen` (through `ld.so`) handle what paths to search in for the respective libraries.
Additionally this fixes a mistake with the library filenames. Now using the `SONAME` instead of a symlinked name that happened to work when dev packages are installed.

**Breaking:** Support for `$APPDIR` based appImage detection is removed.
Though it _should_ still work, because appimages provide an `LD_LIBRARY_PATH` that would be equivalent to what our previous detection method was doing in rust.
