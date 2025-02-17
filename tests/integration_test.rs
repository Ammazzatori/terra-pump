#[cfg(test)]
mod tests {
    #[test]
    fn test_buy_price() {
        assert_eq!(super::calculate_buy_price(100), 0);
    }
}
