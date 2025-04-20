use rand::Rng;

use crate::constants::{PIN_RANGE, TEST_PIN};

pub fn generate_pin() -> String {
    #[cfg(test)]
    let pin = TEST_PIN;

    #[cfg(not(test))]
    let pin = rand::rng().random_range(PIN_RANGE);

    pin.to_string()
}
