# Guide for Toql 0.4

This is the user guide for [Toql](https://docs.rs/toql/0.4/toql/), an ORM with support for async databases.

The guide is published on a [GitHub Page]( https://roy-ganz.github.io/toql_guide/).

## Build

This guide uses [mdBook](https://github.com/rust-lang/mdBook).

To build the `/src` folder and put the guide into `/docs` run:

```bash
mdbook build
```

If you want to see the output after the build call with `--open` option.

## Test

The examples in the guide can be tested with

```bash
cargo test --doc
```


## License
Toql guide is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).