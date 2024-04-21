# dds-wasm

Very rudimentary wrapper around https://crates.io/crates/image_dds just to see if you can display a DDS file in-browser.

For an example of how to use it check index.html.

Two functions are exported, `mipmap_count` and `decode`. Both take an `Int8Array` as first argument which should be the DDS file. mipmap_count will return a number if possible, and decode takes a number as second argument for you to pick a mipmap, which is bound between `0` to `mipmap_count - 1`.
