# Tinted Scheme Extractor

`tinted-scheme-extractor` is a Rust library which generates a Base16
theme based an image provided.

Note: This is early stages so the API is subject to change.

## Install

`cargo add tinted-scheme-extractor`

## Usage

```rust
use std::path::PathBuf;
use tinted_scheme_extractor::{create_scheme_from_image, SchemeParams, System, Variant};

fn main() {
    let image_path = PathBuf::from("./path/to/file.png");
    let name = "Your scheme name".to_string();
    let slug = "your-scheme-slug".to_string();
    let description = Some("Optional description".to_string());
    let variant = Variant::Dark;
    let system = System::Base16;
    let verbose = false;
    let author = "Your name".to_string();

    let scheme = create_scheme_from_image(SchemeParams {
        image_path,
        author,
        description,
        name,
        slug,
        system,
        verbose,
        variant,
    }).unwrap();

    println!("{}", &scheme);
}
```

## Changes to come

- `System::Base16` is currently only supported, `System::Base24` to come
- `Variant::Dark` is currently only supported, `Variant::Light` to come
- I'll probably change the `image_path` input to accept data and not a
  path

## Inspiration

Initially I wasn't sure if I wanted to continue [Flavours] development
or to build something new. [I brought this up with Misterio77] and he
suggested building something new from scratch. This project is a part of
rebuilding the Flavours functionality in Tinty.


[Flavours]: https://github.com/Misterio77/flavours
[I brought this up with Misterio77]: https://github.com/Misterio77/flavours/issues/85
