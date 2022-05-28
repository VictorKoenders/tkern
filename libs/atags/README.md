
This is based on <https://www.simtec.co.uk/products/SWLINUX/files/booting_article.html#appendix_tag_reference>. Please open a pull request for missing tags.

To get started with this crate, create a [`Atags`] struct with a given memory position. The `iter()` method will return an iterator that returns [`Atag`] entries.

```rust
use atags::{Atag, Atags};

let mut buffer = [
    // Core tag
    0x00, 0x00, 0x00, 0x05, // size
    0x54, 0x41, 0x00, 0x01, // tag
    0x00, 0x00, 0x00, 0x01, //  flags
    0x00, 0x00, 0x10, 0x00, //  page_size
    0x12, 0x34, 0x56, 0x78, //  root_device_number

    // Empty tag
    0x0, 0x0, 0x0, 0x0, // size
    0x0, 0x0, 0x0, 0x0, // tag
];
let ptr = core::ptr::NonNull::new(buffer.as_mut_ptr()).unwrap();
let tags = unsafe { Atags::new(ptr.cast()) };

for tag in tags.iter() {
    // first tag is a core tag
    match tag {
        Atag::Core(core) => {
            assert_eq!(core.flags, 1);
            assert_eq!(core.page_size, 0x1000);
            assert_eq!(core.root_device_number, 0x12345678);
        },
        // Do something with the other tags
        // In this example we only get 1 core tag and nothing else
        _ => panic!("Unknown tag {:?}", tag),
    }
}
```

# Features

## `nightly`

Will enable the nightly `strict_provenance` feature in this crate.
- [Core docs](https://doc.rust-lang.org/nightly/std/ptr/index.html#strict-provenance)
- [Tracking issue](https://github.com/rust-lang/rust/issues/95228)