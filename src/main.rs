use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Grid, Label, Button};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use gtk::gdk;
use gtk::*;
fn main() {
    let app = Application::builder()
        .application_id("com.example.launcher")
        .build();

    app.connect_activate(|app| {
        // Create a GTK4 window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Launcher")
            .default_width(300)
            .default_height(100)
            .build();

        // Enable layer-shell support
        window.init_layer_shell();

        // Make it an overlay window
        window.set_layer(Layer::Overlay);

        // Position at top center
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Left, false);
        window.set_anchor(Edge::Right, false);

        // No exclusive zone, so it doesn’t push tiles around
        window.set_exclusive_zone(0);

        // Setting grid for button interface
        let grid = Grid::new();
        Grid::set_column_spacing(&grid,10);
        Grid::set_row_spacing(&grid, 30);
        grid.set_halign(Align::Center);
        grid.set_valign(Align::Center);
        let dolphin = Label::new(Some(""));
        let duck = Label::new(Some(""));
        let pcsx2 = Label::new(Some(""));

        dolphin.set_widget_name("dolphin");
        duck.set_widget_name("duck");
        pcsx2.set_widget_name("pcsx2");

        let button1 = Button::new();
        button1.set_child(Some(&dolphin));
        let button2 = Button::new();
        button2.set_child(Some(&duck));
        let button3 = Button::new();
        button3.set_child(Some(&pcsx2));

        // Grid::attach(&grid, &label, 1, 0, 3, 1);
        Grid::attach(&grid, &button1, 0, 0, 1, 1);
        Grid::attach(&grid, &button2, 1, 0, 1, 1);
        Grid::attach(&grid, &button3, 2, 0, 1, 1);
        
        window.set_child(Some(&grid));

        button1.connect_clicked(|_| {
            std::process::Command::new("dolphin-emu")
                .spawn()
                .expect("Failed to launch Dolphin Emulator");
            std::process::Command::new("sh")
                .arg("-c")
                .arg("sleep 0.2 && killall launcher")
                .spawn()
                .expect("Failed to close launcher");
        });

        button2.connect_clicked(|_| {
            std::process::Command::new("duckstation")
                .spawn()
                .expect("Failed to launch DuckStation");
            std::process::Command::new("sh")
                .arg("-c")
                .arg("sleep 0.2 && killall launcher")
                .spawn()
                .expect("Failed to close launcher");
        });

        button3.connect_clicked(|_| {
            std::process::Command::new("pcsx2")
                .spawn()
                .expect("Failed to launch PCSX2");
            std::process::Command::new("sh")
                .arg("-c")
                .arg("sleep 0.2 && killall launcher")
                .spawn()
                .expect("Failed to close launcher");
        });

        // Load CSS
        let provider = gtk::CssProvider::new();
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|p| p.to_path_buf()))
            .expect("Failed to get executable directory");
        let css_path = exe_dir.join("../../src/style.css");
        println!("Loading CSS from: {:?}", css_path);
        provider
            .load_from_path(css_path);

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.show();

        // TODO:
        // Force Focus
        // Arrow key navigability
        
    });

    app.run();
}
