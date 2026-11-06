use async_process::Command as AsyncCommand;
use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, PartialEq)]
struct ImageEntry {
    path: String,
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
struct PendingAction {
    description: String,
    args: Vec<String>,
}

fn main() {
    dioxus_desktop::launch(App);
}

fn scan_images() -> Vec<ImageEntry> {
    let mut roots = vec![std::env::current_dir().unwrap_or_default()];
    if let Some(home) = std::env::var_os("HOME") {
        let home = PathBuf::from(home);
        roots.extend([home.clone(), home.join("Downloads"), home.join("Desktop")]);
    }

    let mut found = Vec::new();
    for root in roots {
        let Ok(entries) = std::fs::read_dir(root) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "flatimage") {
                found.push(ImageEntry {
                    name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                    path: path.to_string_lossy().into_owned(),
                });
            }
        }
    }
    found.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    found.dedup_by(|a, b| a.path == b.path);
    found
}

fn request_action(
    description: &str,
    args: Vec<String>,
    needs_confirmation: bool,
    selected: String,
    pending: Signal<Option<PendingAction>>,
    status: Signal<String>,
    logs: Signal<Vec<String>>,
    busy: Signal<bool>,
) {
    if selected.is_empty() {
        status.set("Choose a FlatImage first.".to_string());
        return;
    }
    if !Path::new(&selected).is_file() {
        status.set("The selected image no longer exists. Scan again or add its path.".to_string());
        return;
    }
    if needs_confirmation {
        pending.set(Some(PendingAction { description: description.to_string(), args }));
    } else {
        run_flatimage(selected, args, status, logs, busy);
    }
}

fn run_flatimage(
    image: String,
    args: Vec<String>,
    status: Signal<String>,
    logs: Signal<Vec<String>>,
    busy: Signal<bool>,
) {
    let label = if args.is_empty() { "Launch image".to_string() } else { args.join(" ") };
    status.set(format!("Running: {label}"));
    busy.set(true);
    spawn(async move {
        let result = if args.is_empty() {
            Command::new(&image)
                .spawn()
                .map(|_| format!("Started {}. The app is running outside this manager.", image))
                .map_err(|error| format!("Could not launch image: {error}"))
        } else {
            match AsyncCommand::new(&image).args(&args).output().await {
                Ok(output) => {
                    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
                    text.push_str(&String::from_utf8_lossy(&output.stderr));
                    if output.status.success() {
                        Ok(if text.trim().is_empty() { "Completed successfully.".to_string() } else { text })
                    } else {
                        Err(format!("Command exited with {}.\n{}", output.status, text))
                    }
                }
                Err(error) => Err(format!("Could not run FlatImage command: {error}")),
            }
        };
        match result {
            Ok(message) => {
                status.set(message.clone());
                logs.write().push(format!("✓ {label}\n{message}"));
            }
            Err(message) => {
                status.set(message.clone());
                logs.write().push(format!("✗ {label}\n{message}"));
            }
        }
        busy.set(false);
    });
}

fn App() -> Element {
    let mut images = use_signal(scan_images);
    let mut selected = use_signal(String::new);
    let mut manual_path = use_signal(String::new);
    let mut permissions = use_signal(String::new);
    let mut env_vars = use_signal(String::new);
    let mut boot_program = use_signal(String::new);
    let mut recipe = use_signal(String::new);
    let mut bind_source = use_signal(String::new);
    let mut bind_target = use_signal(String::new);
    let mut bind_type = use_signal(|| "ro".to_string());
    let mut layer_path = use_signal(String::new);
    let mut remote_url = use_signal(String::new);
    let mut overlay = use_signal(|| "OVERLAYFS".to_string());
    let pending = use_signal(|| None::<PendingAction>);
    let mut status = use_signal(|| "Choose a .flatimage file to get started.".to_string());
    let mut logs = use_signal(|| vec!["FlatImage Manager ready.".to_string()]);
    let busy = use_signal(|| false);

    let selected_path = selected();
    let selected_name = images().iter()
        .find(|image| image.path == selected_path)
        .map(|image| image.name.clone())
        .unwrap_or_else(|| if selected_path.is_empty() { "No image selected".into() } else { selected_path.clone() });

    let request = move |description: &'static str, args: Vec<String>, confirm: bool| {
        request_action(description, args, confirm, selected(), pending, status, logs, busy);
    };

    rsx! {
        style { {APP_CSS} }
        div { class: "shell",
            header { class: "topbar",
                div {
                    div { class: "brand-mark", "F" }
                    div {
                        h1 { "FlatImage Manager" }
                        p { "Manage portable Linux apps without memorizing commands" }
                    }
                }
                button {
                    class: "secondary",
                    disabled: busy(),
                    onclick: move |_| {
                        let found = scan_images();
                        status.set(format!("Found {} FlatImage file(s).", found.len()));
                        images.set(found);
                    },
                    "↻  Scan for images"
                }
            }

            div { class: "layout",
                aside { class: "sidebar card",
                    div { class: "section-heading",
                        div { class: "eyebrow", "YOUR LIBRARY" }
                        h2 { "FlatImages" }
                    }
                    if images().is_empty() {
                        div { class: "empty",
                            div { class: "empty-icon", "＋" }
                            strong { "No images found yet" }
                            p { "Scan common folders or add a .flatimage by path." }
                        }
                    }
                    for image in images() {
                        {
                            let path = image.path.clone();
                            let is_selected = selected() == image.path;
                            rsx! {
                                button {
                                    class: if is_selected { "image-row selected" } else { "image-row" },
                                    onclick: move |_| selected.set(path.clone()),
                                    span { class: "image-icon", "▧" }
                                    span { class: "image-copy",
                                        strong { "{image.name}" }
                                        small { "{image.path}" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "add-image",
                        label { "Add an image by path" }
                        input {
                            placeholder: "/path/to/app.flatimage",
                            value: "{manual_path}",
                            oninput: move |event| manual_path.set(event.value()),
                        }
                        button {
                            class: "secondary full",
                            onclick: move |_| {
                                let path = manual_path().trim().to_string();
                                let candidate = Path::new(&path);
                                if candidate.is_file() && candidate.extension().is_some_and(|ext| ext == "flatimage") {
                                    let entry = ImageEntry {
                                        name: candidate.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                                        path: candidate.to_string_lossy().into_owned(),
                                    };
                                    if !images().iter().any(|item| item.path == entry.path) {
                                        images.write().push(entry.clone());
                                    }
                                    selected.set(entry.path);
                                    manual_path.set(String::new());
                                    status.set("Image added and selected.".to_string());
                                } else {
                                    status.set("That path is not a .flatimage file. Check the path and try again.".to_string());
                                }
                            },
                            "Add image"
                        }
                    }
                }

                main { class: "content",
                    section { class: "welcome card",
                        div {
                            div { class: "eyebrow", "SELECTED IMAGE" }
                            h2 { "{selected_name}" }
                            p { "Actions run FlatImage's documented fim-* commands directly as child processes. Results appear below; no shell is used." }
                        }
                        button {
                            class: "primary launch",
                            disabled: busy() || selected().is_empty(),
                            onclick: move |_| request("Launch image", Vec::new(), false),
                            if busy() { "Working…" } else { "▶  Launch" }
                        }
                    }

                    div { class: "feature-grid",
                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon amber", "◈" } div { h3 { "Permissions" } p { "Choose what this image can access." } } }
                            div { class: "button-row",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show permissions", vec!["fim-perms".into(), "list".into()], false), "Show current" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Allow network access", vec!["fim-perms".into(), "add".into(), "network".into()], false), "Allow network" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Remove network access", vec!["fim-perms".into(), "del".into(), "network".into()], false), "Remove network" }
                                button { class: "secondary danger", disabled: busy() || selected().is_empty(), onclick: move |_| request("Clear every permission and restore the most isolated setting", vec!["fim-perms".into(), "clear".into()], true), "Clear all" }
                            }
                            div { class: "inline-form",
                                input { placeholder: "Permission names, comma-separated", value: "{permissions}", oninput: move |event| permissions.set(event.value()) }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = permissions().trim().to_string();
                                    if !value.is_empty() { request("Add permissions", vec!["fim-perms".into(), "add".into(), value], false); }
                                }, "Add" }
                            }
                            p { class: "hint", "Known options include home, media, audio, wayland, xorg, dbus_user, usb, input, gpu, network, shm, and dev. Grant only what the app needs." }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon violet", "⌘" } div { h3 { "Default startup" } p { "Choose what opens when you launch." } } }
                            div { class: "button-row",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show startup command", vec!["fim-boot".into(), "show".into()], false), "Show current" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Reset startup command", vec!["fim-boot".into(), "clear".into()], true), "Reset to default" }
                            }
                            div { class: "inline-form",
                                input { placeholder: "Program, for example firefox", value: "{boot_program}", oninput: move |event| boot_program.set(event.value()) }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = boot_program().trim().to_string();
                                    if !value.is_empty() { request("Set default startup program", vec!["fim-boot".into(), "set".into(), value], false); }
                                }, "Save" }
                            }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon blue", "▤" } div { h3 { "Environment variables" } p { "Provide app settings inside the image." } } }
                            div { class: "button-row",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show environment variables", vec!["fim-env".into(), "list".into()], false), "Show current" }
                                button { class: "secondary danger", disabled: busy() || selected().is_empty(), onclick: move |_| request("Remove all saved environment variables", vec!["fim-env".into(), "clear".into()], true), "Clear all" }
                            }
                            div { class: "inline-form",
                                input { placeholder: "NAME=value", value: "{env_vars}", oninput: move |event| env_vars.set(event.value()) }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = env_vars().trim().to_string();
                                    if value.contains('=') { request("Add environment variable", vec!["fim-env".into(), "add".into(), value], false); }
                                    else { status.set("Enter a variable as NAME=value.".to_string()); }
                                }, "Add" }
                            }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon green", "▱" } div { h3 { "Layers" } p { "Save changes or add reusable layers." } } }
                            div { class: "button-row",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("List layers", vec!["fim-layer".into(), "list".into()], false), "List layers" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Commit changes to a new managed layer", vec!["fim-layer".into(), "commit".into(), "layer".into()], true), "Save as layer" }
                                button { class: "secondary danger", disabled: busy() || selected().is_empty(), onclick: move |_| request("Commit changes into the FlatImage executable", vec!["fim-layer".into(), "commit".into(), "binary".into()], true), "Save into image" }
                            }
                            div { class: "inline-form",
                                input { placeholder: "Layer file path to add", value: "{layer_path}", oninput: move |event| layer_path.set(event.value()) }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = layer_path().trim().to_string();
                                    if !value.is_empty() { request("Add layer file", vec!["fim-layer".into(), "add".into(), value], true); }
                                }, "Add layer" }
                            }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon cyan", "⇄" } div { h3 { "Folder bindings" } p { "Share a host folder with this image." } } }
                            button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("List folder bindings", vec!["fim-bind".into(), "list".into()], false), "Show current bindings" }
                            div { class: "field-row",
                                input { placeholder: "Host folder path", value: "{bind_source}", oninput: move |event| bind_source.set(event.value()) }
                                input { placeholder: "Path inside image", value: "{bind_target}", oninput: move |event| bind_target.set(event.value()) }
                            }
                            div { class: "button-row",
                                select { value: "{bind_type}", onchange: move |event| bind_type.set(event.value()),
                                    option { value: "ro", "Read only (safer)" }
                                    option { value: "rw", "Read and write" }
                                    option { value: "dev", "Device" }
                                }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let source = bind_source().trim().to_string();
                                    let target = bind_target().trim().to_string();
                                    if source.is_empty() || target.is_empty() { status.set("Enter both folder paths first.".to_string()); }
                                    else { request("Add host folder binding", vec!["fim-bind".into(), "add".into(), bind_type(), source, target], true); }
                                }, "Add binding" }
                            }
                            p { class: "hint", "Read-only is recommended unless the image needs to edit files in the shared folder." }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon pink", "✦" } div { h3 { "Package recipes" } p { "Look up and install curated software sets." } } }
                            div { class: "inline-form",
                                input { placeholder: "Recipe name, for example gpu", value: "{recipe}", oninput: move |event| recipe.set(event.value()) }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = recipe().trim().to_string();
                                    if !value.is_empty() { request("Get recipe information", vec!["fim-recipe".into(), "info".into(), value], false); }
                                }, "Recipe info" }
                            }
                            div { class: "button-row",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = recipe().trim().to_string();
                                    if !value.is_empty() { request("Download recipe", vec!["fim-recipe".into(), "fetch".into(), value], true); }
                                }, "Fetch" }
                                button { class: "primary small", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = recipe().trim().to_string();
                                    if !value.is_empty() { request("Install recipe and its dependencies", vec!["fim-recipe".into(), "install".into(), value], true); }
                                }, "Install recipe" }
                            }
                            p { class: "hint", "Recipes may download and install software into this image." }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon blue", "▣" } div { h3 { "Desktop integration" } p { "Add or remove app menu and file-type integration." } } }
                            div { class: "button-row wrap",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Enable menu entry, file type, and icon", vec!["fim-desktop".into(), "enable".into(), "entry,mimetype,icon".into()], false), "Enable all" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Disable desktop integration", vec!["fim-desktop".into(), "enable".into(), "none".into()], false), "Disable" }
                                button { class: "secondary danger", disabled: busy() || selected().is_empty(), onclick: move |_| request("Remove generated desktop integration files", vec!["fim-desktop".into(), "clean".into()], true), "Clean files" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Preview desktop menu entry", vec!["fim-desktop".into(), "dump".into(), "entry".into()], false), "Preview entry" }
                            }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon amber", "⚙" } div { h3 { "Privacy & filesystem" } p { "Review isolation and filesystem settings." } } }
                            div { class: "button-row wrap",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Enable case-insensitive paths", vec!["fim-casefold".into(), "on".into()], false), "Enable casefold" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Disable case-insensitive paths", vec!["fim-casefold".into(), "off".into()], false), "Disable casefold" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show filesystem backend", vec!["fim-overlay".into(), "show".into()], false), "Show backend" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show isolation options", vec!["fim-unshare".into(), "list".into()], false), "Show isolation" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show running instances", vec!["fim-instance".into(), "list".into()], false), "Instances" }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show image version information", vec!["fim-version".into(), "full".into()], false), "Version info" }
                            }
                            div { class: "field-row",
                                select { value: "{overlay}", onchange: move |event| overlay.set(event.value()),
                                    option { value: "BWRAP", "BWRAP (native)" }
                                    option { value: "OVERLAYFS", "OverlayFS (fast)" }
                                    option { value: "UNIONFS", "UnionFS (compatible)" }
                                }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Change filesystem backend", vec!["fim-overlay".into(), "set".into(), overlay().to_lowercase()], true), "Apply backend" }
                            }
                        }

                        section { class: "card feature",
                            div { class: "feature-title", span { class: "feature-icon violet", "↗" } div { h3 { "Recipe remote" } p { "Configure where this image fetches recipes." } } }
                            div { class: "button-row",
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| request("Show configured recipe remote", vec!["fim-remote".into(), "show".into()], false), "Show remote" }
                                button { class: "secondary danger", disabled: busy() || selected().is_empty(), onclick: move |_| request("Clear configured recipe remote", vec!["fim-remote".into(), "clear".into()], true), "Clear remote" }
                            }
                            div { class: "inline-form",
                                input { placeholder: "https://example.com/recipes", value: "{remote_url}", oninput: move |event| remote_url.set(event.value()) }
                                button { class: "secondary", disabled: busy() || selected().is_empty(), onclick: move |_| {
                                    let value = remote_url().trim().to_string();
                                    if value.starts_with("https://") || value.starts_with("http://") {
                                        request("Set recipe remote URL", vec!["fim-remote".into(), "set".into(), value], true);
                                    } else { status.set("Enter a valid http:// or https:// remote URL.".to_string()); }
                                }, "Save remote" }
                            }
                        }
                    }

                    section { class: "card activity",
                        div { class: "activity-header",
                            div { h3 { "Activity" } p { "Command output and helpful errors appear here." } }
                            span { class: if busy() { "status-pill running" } else { "status-pill" }, if busy() { "Running" } else { "Ready" } }
                        }
                        p { class: "current-status", "{status()}" }
                        div { class: "log-list",
                            for entry in logs().iter().rev().take(6).rev() {
                                pre { "{entry}" }
                            }
                        }
                    }
                }
            }
            footer { "FlatImage Manager • Commands are passed as process arguments, never through a shell." }

            if let Some(action) = pending() {
                div { class: "modal-backdrop",
                    section { class: "confirm card",
                        div { class: "eyebrow", "PLEASE CONFIRM" }
                        h2 { "{action.description}?" }
                        p { "This action may change the selected FlatImage or download/install software. Review the action before continuing." }
                        div { class: "command-preview", "{action.args.join(" ")}" }
                        div { class: "button-row modal-buttons",
                            button { class: "secondary", onclick: move |_| pending.set(None), "Cancel" }
                            button { class: "primary", disabled: busy(), onclick: move |_| {
                                let selected_image = selected();
                                pending.set(None);
                                run_flatimage(selected_image, action.args.clone(), status, logs, busy);
                            }, "Confirm and continue" }
                        }
                    }
                }
            }
        }
    }
}

const APP_CSS: &str = r#"
* { box-sizing: border-box; }
body { margin: 0; background: #0b1020; color: #e5e7eb; font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }
button, input, select { font: inherit; }
.shell { max-width: 1500px; margin: 0 auto; padding: 22px; }
.topbar { display:flex; align-items:center; justify-content:space-between; gap:18px; padding:18px 22px; margin-bottom:18px; border:1px solid #26334c; border-radius:16px; background:linear-gradient(110deg,#141f35,#111827); }
.topbar > div { display:flex; align-items:center; gap:14px; }
.brand-mark { width:46px; height:46px; display:grid; place-items:center; border-radius:13px; background:#2563eb; color:white; font-size:26px; font-weight:800; }
h1,h2,h3,p { margin-top:0; } h1 { margin-bottom:3px; font-size:22px; } .topbar p,.welcome p,.feature-title p,.activity-header p { margin:0; color:#94a3b8; font-size:13px; }
.layout { display:grid; grid-template-columns:310px minmax(0,1fr); gap:18px; align-items:start; }
.card { background:#111827; border:1px solid #26334c; border-radius:14px; }
.sidebar { padding:17px; position:sticky; top:14px; } .section-heading { margin-bottom:14px; } .section-heading h2 { margin:4px 0 0; font-size:18px; } .eyebrow { color:#60a5fa; font-weight:800; font-size:10px; letter-spacing:1.2px; }
.image-row { display:flex; width:100%; align-items:center; gap:10px; padding:10px; margin:0 0 8px; border:1px solid #26334c; border-radius:10px; background:#151f31; color:#e5e7eb; text-align:left; cursor:pointer; }
.image-row:hover,.image-row.selected { border-color:#3b82f6; background:#172a49; } .image-icon { display:grid; place-items:center; width:34px; height:34px; border-radius:9px; color:#93c5fd; background:#1e3a5f; font-size:20px; flex:none; }
.image-copy { display:grid; gap:3px; min-width:0; } .image-copy strong { font-size:12px; } .image-copy small { color:#94a3b8; font-size:10px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; max-width:220px; }
.empty { text-align:center; padding:18px 10px; color:#94a3b8; } .empty-icon { margin:0 auto 10px; width:38px; height:38px; display:grid; place-items:center; border:1px dashed #475569; border-radius:12px; font-size:24px; } .empty strong { color:#e2e8f0; font-size:13px; } .empty p { font-size:11px; line-height:1.5; }
.add-image { border-top:1px solid #26334c; margin-top:15px; padding-top:14px; display:grid; gap:8px; } .add-image label { font-size:11px; font-weight:700; color:#cbd5e1; }
input,select { width:100%; min-width:0; padding:9px 10px; border:1px solid #344258; border-radius:8px; background:#0b1220; color:#e5e7eb; outline:none; font-size:12px; } input:focus,select:focus { border-color:#3b82f6; }
button { border:0; } button:disabled { opacity:.5; cursor:not-allowed!important; } .secondary,.primary { border-radius:8px; padding:9px 11px; cursor:pointer; font-size:11px; font-weight:700; transition:.15s ease; } .secondary { color:#dbeafe; background:#202c40; border:1px solid #344258; } .secondary:hover:not(:disabled) { background:#2b3a53; } .primary { color:white; background:#2563eb; } .primary:hover:not(:disabled) { background:#1d4ed8; } .full { width:100%; } .launch { padding:13px 20px; font-size:13px; white-space:nowrap; }
.content { display:grid; gap:16px; min-width:0; } .welcome { display:flex; align-items:center; justify-content:space-between; gap:20px; padding:20px 22px; } .welcome h2 { margin:5px 0 7px; font-size:20px; overflow-wrap:anywhere; } .welcome p { max-width:700px; line-height:1.5; }
.feature-grid { display:grid; grid-template-columns:repeat(2,minmax(0,1fr)); gap:14px; } .feature { padding:16px; min-width:0; } .feature-title { display:flex; align-items:center; gap:10px; margin-bottom:14px; } .feature-title h3 { margin:0 0 4px; font-size:14px; } .feature-title p { font-size:11px; line-height:1.4; }
.feature-icon { width:35px; height:35px; display:grid; place-items:center; border-radius:10px; font-size:17px; flex:none; } .amber { color:#fcd34d; background:#3a2e16; } .violet { color:#c4b5fd; background:#30244b; } .blue { color:#93c5fd; background:#1e3453; } .green { color:#86efac; background:#193b2b; } .cyan { color:#67e8f9; background:#153a43; } .pink { color:#f9a8d4; background:#42233a; }
.button-row { display:flex; align-items:center; gap:7px; flex-wrap:wrap; margin:8px 0; } .button-row select { width:auto; flex:1; } .inline-form { display:flex; gap:7px; margin-top:9px; } .inline-form button { flex:none; } .field-row { display:grid; grid-template-columns:1fr 1fr; gap:7px; margin-top:10px; } .hint { margin:10px 0 0; color:#94a3b8; font-size:10px; line-height:1.5; } .danger { color:#fecaca; }
.small { padding:8px 12px; } .activity { padding:16px; } .activity-header { display:flex; align-items:center; justify-content:space-between; gap:12px; } .activity-header h3 { margin:0 0 4px; font-size:14px; } .status-pill { padding:5px 9px; border-radius:999px; color:#86efac; background:#173626; font-size:10px; font-weight:800; } .status-pill.running { color:#fde68a; background:#3b2e13; } .current-status { margin:13px 0 8px; color:#bfdbfe; font-size:12px; } .log-list { max-height:190px; overflow:auto; } .log-list pre { margin:7px 0 0; padding:9px; white-space:pre-wrap; overflow-wrap:anywhere; border-radius:8px; background:#0b1220; color:#cbd5e1; font:11px/1.5 ui-monospace,SFMono-Regular,Consolas,monospace; }
.modal-backdrop { position:fixed; z-index:20; inset:0; display:grid; place-items:center; padding:20px; background:#020617cc; } .confirm { width:min(480px,100%); padding:22px; box-shadow:0 24px 80px #0009; } .confirm h2 { margin:7px 0; font-size:19px; } .confirm p { color:#94a3b8; font-size:12px; line-height:1.6; } .command-preview { padding:10px; border-radius:8px; background:#0b1220; color:#cbd5e1; font:11px ui-monospace,SFMono-Regular,Consolas,monospace; overflow-wrap:anywhere; } .modal-buttons { justify-content:flex-end; margin-top:16px; }
footer { padding:16px 3px 4px; color:#64748b; font-size:10px; text-align:center; }
@media (max-width:950px) { .layout { grid-template-columns:1fr; } .sidebar { position:static; } .feature-grid { grid-template-columns:1fr; } }
@media (max-width:600px) { .shell { padding:10px; } .topbar,.welcome { align-items:flex-start; flex-direction:column; } .field-row { grid-template-columns:1fr; } }
"#;
