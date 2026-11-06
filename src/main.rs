use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

#[derive(Clone, Copy)]
struct FlatImageAction {
    label: &'static str,
    command: &'static str,
    tint: &'static str,
}

fn App() -> Element {
    let mut selected = use_signal(|| "alpine.flatimage".to_string());
    let status = use_signal(|| "Ready".to_string());

    let images = vec![
        ("alpine.flatimage", "Alpine Linux", "Ready"),
        ("arch.flatimage", "Arch Linux", "Ready"),
        ("blueprint.flatimage", "Blueprint", "Ready"),
    ];

    let actions = vec![
        FlatImageAction { label: "Launch", command: "run", tint: "#16a34a" },
        FlatImageAction { label: "Permissions", command: "fim-perms add xorg,network", tint: "#f59e0b" },
        FlatImageAction { label: "Desktop", command: "fim-desktop enable entry,mimetype,icon", tint: "#0ea5e9" },
        FlatImageAction { label: "Layer", command: "fim-layer commit layer", tint: "#ef4444" },
        FlatImageAction { label: "Boot", command: "fim-boot set firefox", tint: "#8b5cf6" },
    ];

    rsx! {
        div {
            class: "app",
            style: "font-family: sans-serif; background: #0b1020; color: #e5e7eb; min-height: 100vh; padding: 18px;",
            header {
                style: "display: flex; justify-content: space-between; align-items: center; background: #111827; padding: 16px 20px; border-radius: 14px; border: 1px solid #334155; margin-bottom: 18px;",
                h1 { style: "margin: 0; font-size: 1.7rem;", "FlatImage Manager" }
                button {
                    style: "padding: 10px 16px; border: 0; border-radius: 10px; background: #2563eb; color: white; font-weight: 700; cursor: pointer;",
                    onclick: move |_| status.set("Checking FlatImage files...".to_string()),
                    "Refresh"
                }
            }

            div {
                style: "display: grid; grid-template-columns: 320px minmax(0, 1fr); gap: 18px;",
                aside {
                    style: "background: #111827; border-radius: 14px; padding: 16px; border: 1px solid #334155;",
                    h2 { style: "margin-top: 0; margin-bottom: 16px;", "Images" }
                    for (name, label, state) in images {
                        button {
                            style: format!(
                                "display: block; width: 100%; text-align: left; margin-bottom: 10px; border-radius: 10px; border: 1px solid {}; background: {}; color: white; padding: 12px; cursor: pointer;",
                                if selected() == name { "#60a5fa" } else { "#374151" },
                                if selected() == name { "#1d4ed8" } else { "#1f2937" }
                            ),
                            onclick: move |_| selected.set(name.to_string()),
                            div { style: "font-weight: 700; margin-bottom: 4px;", "{label}" }
                            div { style: "font-size: 0.8rem; color: #cbd5e1;", "{state}" }
                        }
                    }
                }

                main {
                    style: "background: #111827; border-radius: 14px; padding: 18px; border: 1px solid #334155;",
                    h2 { style: "margin-top: 0; margin-bottom: 18px;", "Selected: {selected()}" }

                    div {
                        style: "display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 20px;",
                        for action in actions {
                            button {
                                style: format!("padding: 10px 16px; border: 0; border-radius: 10px; background: {}; color: white; font-weight: 700; cursor: pointer;" , action.tint),
                                onclick: move |_| {
                                    let image = selected();
                                    status.set(format!("{}: {} {}", action.label, image, action.command));
                                },
                                "{action.label}"
                            }
                        }
                    }

                    div {
                        style: "background: #0f172a; border: 1px solid #334155; border-radius: 12px; padding: 16px;",
                        h3 { style: "margin-top: 0; margin-bottom: 8px;", "Status" }
                        p { style: "margin: 0; color: #bfdbfe; white-space: pre-wrap;", "{status()}" }
                    }
                }
            }
        }
    }
}
