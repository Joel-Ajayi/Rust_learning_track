struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with data {}", self.data)
    }
}

fn main() {
    // Dropping is in reverse order
    let f = CustomSmartPointer {
        data: "my stuff".to_string(),
    };
    // to call drop on the func we want if we do not want the order
    drop(f);
    let a = CustomSmartPointer {
        data: "my stuff2".to_string(),
    };
}
