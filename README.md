# flatimage-manager

A Dioxus desktop manager for FlatImage images. The goal is to make common FlatImage tasks discoverable through a graphical interface instead of requiring users to remember CLI syntax.

## Current status

This branch contains an early UI prototype. Its image entries and action buttons are placeholders: image discovery and execution of FlatImage commands are not implemented yet.

## Planned GUI workflows

- Find and select local `.flatimage` files
- Launch an image
- Configure permissions and environment variables
- Set the default boot command
- Manage layers, bindings, recipes, and desktop integration
- Show operation status and output, with confirmation before consequential changes

## Development

Install Rust, then run:

```sh
cargo run
```

Dioxus desktop also requires the platform's WebView dependencies. Consult the Dioxus desktop setup guide for the operating system in use.

## FlatImage command reference

The GUI is intended to invoke FlatImage's documented commands as child processes, without constructing shell command strings. See the [FlatImage documentation](https://flatimage.github.io/docs) for supported operations and behavior.
