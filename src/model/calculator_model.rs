use fasteval;

#[derive(Default)]
pub struct CalculatorModel {}

impl CalculatorModel {
    pub fn string_executer(operation: &str) -> String {
        let mut ns: fasteval::EmptyNamespace = fasteval::EmptyNamespace;
        let operation_result = fasteval::ez_eval(operation, &mut ns);

        let result = match operation_result {
            Ok(v) if v.is_infinite() => "Cannot divide by zero".to_string(),
            Ok(v) if v.is_nan() => "Invalid operation".to_string(),
            Ok(v) => v.to_string(),
            Err(_) => "Invalid operation".to_string(),
        };

        return result;
    }
}
