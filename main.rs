extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, GtkWindowExt, WindowType, ContainerExt, WidgetExt, Fixed, FixedExt};
use gtk::{ButtonBox};
use std::cell::RefCell;

fn main() {
    let count1 = RefCell::new(0);
    let count2 = RefCell::new(0);

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Glade");
    window.set_default_size(1024, 768);
    window.set_can_focus(false);

    let button_box = ButtonBox::new(gtk::Orientation::Vertical);
    button_box.set_visible(true);

    let radio_button_1 = gtk::RadioButton::new();
    radio_button_1.set_label("1");
    let radio_button_2 = gtk::RadioButton::new();
    radio_button_2.set_label("2");
    let radio_button_3 = gtk::RadioButton::new();
    radio_button_3.set_label("3");

    button_box.add(&radio_button_1);
    button_box.add(&radio_button_2);
    button_box.add(&radio_button_3);

    let button1 = Button::new();
    button1.set_label(&"Button 1");
    button1.set_size_request(20, 35);
    button1.set_visible(true);
    button1.set_can_focus(true);
    button1.set_receives_default(true);

    let button2 = Button::new();
    button2.set_label("Button 2");
    button2.set_size_request(20, 35);
    button2.set_visible(true);
    button2.set_can_focus(true);
    button2.set_receives_default(true);

    let label = gtk::Label::new("Label Space");
    label.set_size_request(20, 35);
    label.set_visible(true);

    let label_clone_1 = label.clone();
    button1.connect_clicked(move |_| {
        let mut clicks = count1.borrow_mut();
        *clicks = *clicks + 1;
        let mytext = format!( "Button 1 has been clicked {} times", clicks);
        label_clone_1.set_text(&mytext);
    });

    let label_clone_2 = label.clone();
    button2.connect_clicked(move |_| {
        let mut clicks = count2.borrow_mut();
        *clicks = *clicks + 1;
        let mytext = format!( "Button 2 has been clicked {} times", clicks);
        label_clone_2.set_text(&mytext);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.add(&button1);
    window.add(&button2);
    window.add(&label);
    window.add(&button_box);

    window.show_all();
    gtk::main();
}