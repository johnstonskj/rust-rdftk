# How to contribute

I'm really glad you're reading this, because we need volunteer developers to help this project continue to grow and improve.

Firstly, read our [Code of Conduct](./CODE_OF_CONDUCT.md) to keep our community approachable and respectable.

You may like to pick up one of the issues marked [help wanted](../../labels/help%20wanted) or [good first issue](../../labels/good%20first%20issue) as an introduction. Alternatively, [documentation](../../labels/documentation) issues can be a great way to understand the project and help improve the developer experience.

## Submitting changes

Changes are submitted via pull requests, using the template [pull_request_template.md](./pull_request_template.md).

Always write a clear log message for your commits. One-line messages are fine for small changes, but bigger changes should look like this:

    $ git commit -m "A brief summary of the commit
    > 
    > A paragraph describing what changed and its impact."

## Coding conventions

The primary tool for coding conventions is rustfmt, and specifically `cargo fmt` is a part of the build process and will cause Actions to fail. 

DO NOT create or update any existing `rustfmt.toml` file to change the default formatting rules.

DO NOT add any feature annotations that would prohibit building on the stable channel. In some cases new crate-level features can be used to introduce an unstable feature dependency but these MUST be clearly documented and optional.
