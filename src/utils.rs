use byte_unit::{Byte, UnitType};

pub fn format_size(bytes: u64) -> String {
    let byte = Byte::from_u64(bytes);
    byte.get_appropriate_unit(UnitType::Binary).to_string()
}