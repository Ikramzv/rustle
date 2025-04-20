use rand::Rng;

use crate::constants::PIN_RANGE;

pub fn generate_pin() -> String {
    let pin = rand::rng().random_range(PIN_RANGE);

    pin.to_string()
}
