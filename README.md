# flatimage-manager

A Dioxus desktop GUI for managing FlatImage files without requiring users to memorize CLI syntax.

## Current features

- Scans the current directory, home directory, Downloads, and Desktop for `.flatimage` files; manual path entry is also available.
- Launches a selected FlatImage.
- Provides GUI workflows for permissions, environment variables, boot command, layers, folder bindings, recipes, desktop integration, casefold, overlay backend, instances, version information, and recipe remotes.
- Runs FlatImage commands as child processes with separate arguments (never through a shell) and displays stdout/stderr in the activity panel.
- Shows confirmation dialogs for potentially consequential operations, including layer commits, recipe changes, and clearing settings.

## Run

Install Rust and the system WebView dependencies required by Dioxus Desktop, then run:

```sh
cargo run
```

## Notes

The app invokes the selected FlatImage executable using its documented `fim-*` command interface. Some operations change configuration embedded in the executable or install software. Review confirmations and output before proceeding. See the [FlatImage documentation](https://flatimage.github.io/docs) for details.
