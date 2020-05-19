# string_concat!

A useful macro for putting together strings in a low overhead manner for `no_std` + `alloc` apps.

```
let name = "Richard";
let msg = string_concat!("Hello ",name,"!");
```