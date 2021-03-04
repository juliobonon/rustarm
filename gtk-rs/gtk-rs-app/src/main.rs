extern crate gtk;
extern crate gio;
extern crate sys_info;

use sys_info::*;
use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Container, Label, Application, ApplicationWindow, Grid};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {

    
        let window = ApplicationWindow::new(app);
        window.set_title("Torizon Application");
        window.set_default_size(200, 200);
        let cpu_grid = Grid::new();
        let cpu_label = Label::new(None);
        let cpu_text =  cpu_num().unwrap();
        cpu_label.set_text(&cpu_text.to_string());
        let cputip = "Numbers of cores: ";
        let cputip_label = Label::new(None);
        cputip_label.set_text(&cputip);
        cpu_grid.add(&cputip_label);
        cpu_grid.add(&cpu_label);
        window.add(&cpu_grid);

        let os_grid = Grid::new();
        let os_info = Label::new(None);
        let info_text = "Running on";
        os_info.set_text(&info_text);
        let os_label = Label::new(None);
        let os_text = os_type().unwrap();
        os_label.set_text(&os_text.to_string());
        let os_release_label = Label::new(None);
        let os_release = os_release().unwrap();
        os_release_label.set_text(&os_release.to_string());
        cpu_grid.add(&os_info);
        cpu_grid.add(&os_label);
        cpu_grid.add( &os_release_label);
       

        window.show_all();  
    });

    application.run(&[]);
}