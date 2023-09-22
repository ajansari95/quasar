#[cfg(test)]
mod tests {
    use crate::math::tick::tick_to_price;

    #[test]
    fn t2p() {
        let upper = -14300000;
        let lower = -14650000;
        println!("LOWER_TICK: {} -> {}", lower, tick_to_price(lower).unwrap());
        println!("UPPER_TICK: {} -> {}", upper, tick_to_price(upper).unwrap());
    }
}
