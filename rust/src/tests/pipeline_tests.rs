// pipeline_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::{run_pipeline, PipelineConfig, PipelineError};

    #[test]
    fn test_pipeline_with_valid_input() {
        let config = PipelineConfig::default();
        let input = "name: Alice\nage: 30";
        let result = run_pipeline(input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.name, "Alice");
        assert_eq!(output.age, 30);
    }

    #[test]
    fn test_pipeline_with_missing_field() {
        let config = PipelineConfig::default();
        let input = "name: Bob";
        let result = run_pipeline(input, &config);
        assert!(matches!(result, Err(PipelineError::MissingField(_))));
    }

    #[test]
    fn test_pipeline_with_invalid_format() {
        let config = PipelineConfig::default();
        let input = "this is not valid";
        let result = run_pipeline(input, &config);
        assert!(matches!(result, Err(PipelineError::ParseError(_))));
    }

    #[test]
    fn test_pipeline_with_custom_config() {
        let config = PipelineConfig {
            strict_mode: true,
            ..Default::default()
        };
        let input = "name: Charlie\nage: thirty";
        let result = run_pipeline(input, &config);
        assert!(matches!(result, Err(PipelineError::ValidationError(_))));
    }
}
