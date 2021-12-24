#[cfg(test)]
mod tests {
    use rustpickle::RustPickle;

    #[test]
    fn test_add_get() {
        let mut rs = RustPickle::new("test".to_owned());
        let test_key = "language".to_owned();
        let test_value = "rust".to_owned();
        rs.add(test_key.clone(), test_value.clone());
        let result = rs.get(test_key.clone());
        assert_eq!(result, Some(test_value));
    }

    #[test]
    fn test_dump_read() {
        let mut rs = RustPickle::new("thread".to_owned());
        let test_key = "type".to_owned();
        let test_value = "async".to_owned();
        rs.add(test_key.clone(), test_value.clone());
        rs.dump()
            .expect("Expected result write success")
            .recv()
            .expect("Write must have sent a signal.");
        let _ = rs.read();
        let result = rs.get(test_key.clone());
        assert_eq!(Some(test_value), result);
    }

    #[test]
    fn test_add_get_twice() {
        let mut rs = RustPickle::new("test".to_owned());
        let test_key = "school".to_owned();
        let test_value = "umd".to_owned();
        rs.add(test_key.clone(), test_value.clone());
        let result = rs.get(test_key.clone());
        assert_eq!(result, Some(test_value));

        let test_key = "name".to_owned();
        let test_value = "mark".to_owned();
        rs.add(test_key.clone(), test_value.clone());
        let result = rs.get(test_key.clone());
        assert_eq!(result, Some(test_value));
    }
    #[test]
    fn submit_students_grade() {
        let students = ["Mark", "John", "Julia", "Grace"];

        // initialize a pickledb instance
        let mut rs = RustPickle::new("330_grades".to_owned());

        // add pairs of string keys and values
        for student in students {
            rs.add(String::from(student), "90".to_owned());
        }

        // getting the value with key should work
        assert_eq!(rs.get("Mark".to_owned()), Some("90".to_owned()));

        // replacing the value
        rs.add(String::from("Mark"), "70".to_owned());
        assert_eq!(rs.get("Mark".to_owned()), Some("70".to_owned()));

        //save the changes for the future use
        //wait for it to complete
        rs.dump()
            .expect("Expected result write success")
            .recv()
            .expect("Waiting for write to be done.");

        let mut new_rs = RustPickle::new("330_grades".to_owned());
        let _ = new_rs.read();
        for student in students {
            assert_eq!(
                rs.get(String::from(student)),
                Some((if student == "Mark" { "70" } else { "90" }).to_owned())
            );
        }
    }
}
