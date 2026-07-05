use meval::eval_str;

#[derive(Default)]
pub struct CalculatorModel {}

impl CalculatorModel {
    pub fn string_executer(operation: String) -> String {
        let operation_result = eval_str(operation);

        let result = match operation_result {
            Ok(v) if v.is_infinite() => "Cannot divide by zero".to_string(),
            Ok(v) if v.is_nan() => "Invalid operation".to_string(),
            Ok(v) => v.to_string(),
            Err(_) => "Invalid operation".to_string(),
        };

        return result;
    }
}
