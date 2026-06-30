use slint::SharedString;

slint::include_modules!();

fn main(){
    let main_window = MainWindow::new().unwrap();
    let weak_window = main_window.as_weak();

    main_window.on_add_number(move |value: SharedString| {
        let ui = weak_window.unwrap();
        let text = ui.get_input_value().to_string();
        let new_value = SharedString::from(format!("{}{}", text, value));
        ui.set_input_value(new_value);
    });

    main_window.run().unwrap();
}