use crate::model::calculator_model::CalculatorModel;
use slint::SharedString;
use is_digit::IsDigit;

pub mod model;

slint::include_modules!();

fn main() {
    let mut model: CalculatorModel = Default::default();
    model.set_x(5.0);
    let main_window = MainWindow::new().unwrap();
    let weak_window = main_window.as_weak();

    // let x = model.get_x();
    // let new_value = SharedString::from(x.or(Some(0.0)).unwrap().to_string());

    // let ui = weak_window.unwrap();
    // ui.set_input_value(new_value);
    main_window.on_add_number(move |value: SharedString| {
        let temp  = value.to_string();
        
        if temp.len() <= 1 && temp.is_dec_digit() {
            let ui = weak_window.unwrap();
            let current_input = ui.get_input_value().to_string();
            let new_value = SharedString::from(format!("{}{}", current_input, value));
            ui.set_input_value(new_value);
        }
    });

    main_window.run().unwrap();
}
