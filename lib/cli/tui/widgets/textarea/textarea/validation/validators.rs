pub fn required_validator(input: &str) -> Result<(), String> {
    if input.is_empty() {
        Err(format!("This field is required"))
    } else {
        Ok(())
    }
}
