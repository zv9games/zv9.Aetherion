use aetherion_shared::AetherionData;
use regex::Regex;

pub fn validate_data_id(data: &AetherionData) -> bool {
    // Placeholder regex check
    let re = Regex::new(r"^\d+$").unwrap();
    re.is_match(&data.id.to_string())
}