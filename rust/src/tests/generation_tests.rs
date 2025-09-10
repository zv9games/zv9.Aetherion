// generation_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aetherion::codegen::emitter::{generate_code};
	use crate::aetherion::codegen::config::Config;


    #[test]
    fn test_basic_generation() {
        let config = Config::default();
        let input = "struct MyStruct;";
        let output = generate_code(input, &config);
        assert!(output.contains("impl MyStruct"));
    }

    #[test]
    fn test_generation_with_custom_config() {
        let config = Config {
            derive_debug: true,
            ..Default::default()
        };
        let input = "struct CustomStruct;";
        let output = generate_code(input, &config);
        assert!(output.contains("#[derive(Debug)]"));
    }

    #[test]
    fn test_empty_input() {
        let config = Config::default();
        let input = "";
        let output = generate_code(input, &config);
        assert_eq!(output, "");
    }

    #[test]
    fn test_invalid_input() {
        let config = Config::default();
        let input = "not a struct";
        let result = std::panic::catch_unwind(|| generate_code(input, &config));
        assert!(result.is_err());
    }
}
