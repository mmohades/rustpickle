#[cfg(test)]
mod tests {
    use rustpickle::RustPickle;

    #[test]
    fn test_add_get() {
        let mut rs = RustPickle::new("test".to_owned());
        let test_key = "t1".to_owned();
        let test_value = "95".to_owned();
        rs.add(test_key.clone(), test_value.clone());
        let result = rs.get(test_key.clone());
        assert_eq!(result, Some(test_value));
    }
}
