# flatimage-manager

A Dioxus desktop manager for FlatImage images, designed to make common tasks accessible without memorizing CLI syntax.

## Status

Early UI prototype. Local image discovery and real FlatImage command execution are not implemented yet; action buttons currently display status text only.

## Development

Install Rust and the system WebView dependencies required by Dioxus Desktop, then run:

```sh
cargo run
```

The planned GUI workflows include image discovery and launch, permissions, environment variables, boot commands, desktop integration, layers, bindings, recipes, and clear operation feedback. See the [FlatImage documentation](https://flatimage.github.io/docs) for supported operations.
