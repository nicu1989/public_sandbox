# crates-io-demo-lib-nicu1989

A minimal Rust library used to demonstrate publishing to crates.io.

## How to publish to crates.io

1. Create a crates.io account and generate an API token (Account Settings).
2. Authenticate locally:
   - `cargo login <YOUR_API_TOKEN>`
3. Update `Cargo.toml`:
   - Ensure `name` is unique and owned by you.
   - Update `description`, `repository`, `keywords`, etc.
   - Keep `license = "MIT OR Apache-2.0"` or set your own.
5. Dry run packaging:
   - `cargo publish --dry-run`
6. Publish:
   - `cargo publish`

### Releasing updates

- Bump the version in `Cargo.toml` following SemVer.
- Run `cargo publish --dry-run`, then `cargo publish`.

### Notes

- You must choose a unique crate name. `crates-io-demo-lib` is an example.
- This crate is dual-licensed under MIT or Apache-2.0 just for demo purpose.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
crates-io-demo-lib-nicu1989 = "0.1"
```

Then call the function:

```rust
use crates_io_demo_lib::greet;

fn main() {
    println!("{}", greet("world"));
}
```
