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
        let current_placeholder = ui.get_placeholder_value().to_string();

        match temp {
            "=" => {
                if current_input != "" {
                    let result = CalculatorModel::string_executer(current_input);
                    ui.set_placeholder_value(result.clone().into());
                    ui.set_input_value(result.into());
                }
            }
            "C" => {
                ui.set_input_value("".into());
                ui.set_placeholder_value("".into());
            }
            "<-" => {
                current_input.pop();
                ui.set_input_value(current_input.into());
                ui.set_placeholder_value("".into());
            }
            "*" => set_values(ui, current_input, current_placeholder, value),
            "/" => set_values(ui, current_input, current_placeholder, value),
            "+" => set_values(ui, current_input, current_placeholder, value),
            "-" => set_values(ui, current_input, current_placeholder, value),
            _ => {
                println!("{}{}", current_input, current_placeholder);
                if current_input == current_placeholder {
                    ui.set_input_value(value);
                    ui.set_placeholder_value("".into());
                } else {
                    let new_value = SharedString::from(format!("{}{}", current_input, value));
                    ui.set_input_value(new_value);
                    ui.set_placeholder_value("".into());
                }
            }
        }
    });

    main_window.run().unwrap();
}

fn set_values(
    ui: MainWindow,
    current_input: String,
    current_placeholder: String,
    value: SharedString,
) {
    if !current_placeholder.is_empty() && current_placeholder.parse::<f64>().is_ok() {
        let new_value = SharedString::from(format!("{}{}", current_placeholder, value));
        ui.set_input_value(new_value);
        ui.set_placeholder_value("".into());
    } else {
        let new_value = SharedString::from(format!("{}{}", current_input, value));
        ui.set_input_value(new_value);
        ui.set_placeholder_value("".into());
    }
}
