# string_concat!

A useful macro for putting together strings in a low overhead manner for `no_std` + `alloc` apps.

```rust
let name = "Richard";
let msg:String = string_concat!("Hello ",name,"!");
```

saves you from typing

```rust
let name = "Richard";
let msg:String = {
  let temp_string = String::new();
  temp_string.push_str("Hello");
  temp_string.push_str(name);
  temp_string.push_str("!");
  temp_string
}
```
