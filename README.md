# flatimage-manager

A Dioxus desktop GUI for managing FlatImage files. The app aims to make common tasks approachable without requiring users to type commands.

## Current implementation

- Scans the current folder, home folder, Downloads, and Desktop for `.flatimage` files; you can also add a file by path.
- Launches a selected image directly.
- Runs supported `fim-*` management commands as child processes and displays their output in the activity panel. Commands are passed as argument arrays (not through a shell).
- Includes UI workflows for permissions, environment variables, boot command, layers, folder bindings, recipes, desktop integration, casefold, overlay, instances, and version info.

## Run

Install Rust and the system WebView dependencies needed by Dioxus Desktop, then run:

```sh
cargo run
```

## Notes

The GUI invokes the FlatImage executable itself with its documented `fim-*` arguments. Some operations can change image configuration or install software; review the action and its output. FlatImage documentation: <https://flatimage.github.io/docs>.
