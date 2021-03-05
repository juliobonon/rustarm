use gtk::*;
use gtk::prelude::*;

fn main(){

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("torizon.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("window1").unwrap();
    let fixed: gtk::Fixed = builder.get_object("fixed1").unwrap();
    let image: gtk::Image = builder.get_object("image1").unwrap();
    let label : gtk::Label = builder.get_object("label1").unwrap();

    window.set_title("Torizon Application");
    window.set_default_size(420,250);
    window.show_all();
    
    gtk::main();
    
}

