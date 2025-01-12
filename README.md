# `bevy_docs_extension_demo`

A crate for testing [rustdoc] extensions and configurations for the [Bevy game engine].

See [`./docs-rs`](./docs-rs) for implemented extensions and how to use them,
and [`Cargo.toml`](./Cargo.toml) for the configuration passed to [docs.rs] for building the crate.

## Building Locally

To apply the documentation extensions in this directory, you must provide `cargo doc`
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

[docs.rs]: https://docs.rs
[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[`rustdoc` documentation]: https://doc.rust-lang.org/rustdoc/command-line-arguments.html
[Bevy game engine]: https://bevyengine.org

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.
