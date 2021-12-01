
# Debugging Toql
Toql generates a lot of code. Mostly from the `Toql` derive, but also from various macros, such as `query!`.

To debug Toql generated code, follow these steps:

1. If you have a lot of modules move the affected `mod` at the end of the mod list. (So generated code will appear in the terminal last).
2. Run `cargo` with the logger enabled and a single job:
```rust
 RUST_LOG=DEBUG cargo check --jobs=1
```
3. Copy all the logged code from the derived struct and paste it into the source file.
4. Remove the log headers by regex replacing `\[2.*` with an empty string. There should be 13 occurences.
5. Copy your derived struct.
6. Comment out your derived struct.
7. On the copied struct remove all references to Toql.
8. Format your document and debug!

## Support
If you have issues with Toql you can post them on [GitHub](https://github.com/roy-ganz/toql/issues).

