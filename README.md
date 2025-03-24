# Tinted Scheme Extractor

[![Matrix Chat](https://img.shields.io/matrix/tinted-theming:matrix.org)](https://matrix.to/#/#tinted-theming:matrix.org)
[![Crates.io](https://img.shields.io/crates/v/tinted-scheme-extractor.svg)](https://crates.io/crates/tinted-scheme-extractor)
[![LICENSE](https://img.shields.io/crates/l/tinted-scheme-extractor)](./LICENSE)

`tinted-scheme-extractor` is a [Tinted Theming] Rust library which
generates a Base16 theme based a provided image.

Note: This is early stages so the API is subject to change.

## Install

`cargo add tinted-scheme-extractor`

## Usage

```rust
use std::path::PathBuf;
use tinted_builder::{SchemeSystem, SchemeVariant};
use tinted_scheme_extractor::{create_scheme_from_image, SchemeParams};

fn main() {
    let image_path = PathBuf::from("./path/to/file.png");
    let name = "Your scheme name".to_string();
    let slug = "your-scheme-slug".to_string();
    let description = Some("Optional description".to_string());
    let variant = SchemeVariant::Dark;
    let system = SchemeSystem::Base16;
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

## Inspiration

Initially I wasn't sure if I wanted to continue [Flavours] development
or to build something new. [I brought this up with Misterio77] and he
suggested building something new from scratch. This project is a part of
rebuilding the Flavours functionality in Tinty.


[Tinted Theming]: https://github.com/tinted-theming
[Flavours]: https://github.com/Misterio77/flavours
[I brought this up with Misterio77]: https://github.com/Misterio77/flavours/issues/85
