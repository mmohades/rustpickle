# RustPickle 

### Usage example

```rust
use rustpickle::RustPickle;

pub fn main() {
    let students = ["Daniel", "John", "Julia", "Grace"];
    let mut rs = RustPickle::new("grades".to_owned());

    // Add pairs of string keys and values
    for student in students {
        rs.add(String::from(student), "90".to_owned());
    }

    // get value
    assert_eq!(rs.get("Grace".to_owned()), Some("90".to_owned()));

    // updating the value
    rs.add(String::from("Grace"), "70".to_owned());
    assert_eq!(rs.get("Grace".to_owned()), Some("70".to_owned()));

    //dump and wait for completion
    rs.dump()
        .expect("Expected result write success")
        .recv()
        .expect("Expected writing to file to be completed.");

    let mut new_rs = RustPickle::new("grades".to_owned());
    new_rs.read().expect("Failed to read the db file.");
    assert_eq!(rs.get("Grace".to_owned()), Some("70".to_owned()));

    for student in students {
        println!(
            "Name: {}, Grade: {}",
            student,
            rs.get(String::from(student))
                .expect("Failed to get student's grade")
        );
    }
}
```

