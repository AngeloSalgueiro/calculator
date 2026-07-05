use crate::model::calculator_model::CalculatorModel;
use slint::SharedString;

pub mod model;

slint::include_modules!();

fn main() {
    // View
    let main_window = MainWindow::new().unwrap();
    let weak_window = main_window.as_weak();

    main_window.on_send_carac(move |value: SharedString| {
        let temp = value.as_str();
        let ui = weak_window.unwrap();
        let mut current_input = ui.get_input_value().to_string();

        match temp {
            "=" => {
                let result = CalculatorModel::string_executer(current_input);
                ui.set_placeholder_value(result.into());
                ui.set_input_value("".into());
            }
            "C" => {
                ui.set_input_value("".into());
            }
            "<-" => {
                current_input.pop();
                ui.set_input_value(current_input.into())
            }
            _ => {
                let new_value = SharedString::from(format!("{}{}", current_input, value));
                ui.set_input_value(new_value);
                ui.set_placeholder_value("".into());
            }
        }
    });

    main_window.run().unwrap();
}
