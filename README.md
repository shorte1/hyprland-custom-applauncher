# hyprland-custom-applauncher
app launcher for personal use

currently just launches the dolphin, duckstation, and pcsx2 emulators

- Images are managed in CSS
- Applications to launch are hard coded in rust
- Easily expandable but must be rebuilt if applications are changed, added, or removed
- I've set shortcuts for launching in hyprland config

How to set up shortcut:
- In hyprland.conf make a shortcut with the format:
    bind = /<key-inputs-of-choice/>, exec, pgrep launcher >/dev/null 2>&1 && killall launcher || /<path-to/>/launcher/target/release/launcher

How to rebuild:
- Go to launcher folder and run "cargo build --release"
