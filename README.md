# `bevy_docs_extension_demo`

A crate for testing [rustdoc] extensions and configurations for the [Bevy game engine].

## Building Locally

To apply documentation extensions to local builds, you must provide `cargo doc`
with the relevant arguments for `RUSTDOCFLAGS` to configure the output of rustdoc.
See the [`rustdoc` documentation] for a list of available arguments.

For example, building the docs with the `trait-tags` extension and `--no-deps` can be done
with the following command:

```bash
RUSTDOCFLAGS="--html-after-content docs-rs/trait-tags.html" cargo doc --no-deps
```

Note that if documentation is also built for other dependencies,
you must provide absolute paths instead of relative paths for files:

```bash
RUSTDOCFLAGS="--html-after-content path/to/docs-rs/trait-tags.html" cargo doc
```

## Building on Docs.rs

Extensions can be applied to [docs.rs] builds by providing the relevant arguments
for `rustdoc-args` in your `Cargo.toml`. See the [`rustdoc` documentation]
for a list of available arguments.

For example, building the docs with the `trait-tags` extension can be done
with the following configuration:

```toml
[package.metadata.docs.rs]
rustdoc-args = ["--html-after-content", "docs-rs/trait-tags.html"]
```

See [`Cargo.toml`](./Cargo.toml) for the configuration passed to [docs.rs] for building this crate.

[docs.rs]: https://docs.rs
[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[`rustdoc` documentation]: https://doc.rust-lang.org/rustdoc/command-line-arguments.html
[Bevy game engine]: https://bevyengine.org

## Trait Tags

This extension adds tags on types to indicate usage based on Bevy traits such as `Component`, `Event`, etc.

For trait tags to show in listings, metadata needs to be embedded within the source files. You likely do not want to commit this to version control on a branch that receives new manual changes.
Run the `embed-trait-info` tool by pointing it at your project workspace (using `--root-dir` if it doesn't match the working directors) and give it the names of the packages to modify, e.g.

```bash
cargo run -p embed-trait-info bevy_docs_extension_demo
```

before building the documentation as above.

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.
