struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping the CustomSmartPointer with data: {}!", self.data);
    }
}

fn main() {
    let _b = CustomSmartPointer {data: String::from("my stuff")};
    let _c = CustomSmartPointer {data: String::from("other stuff")};
    drop(_c);
    println!("CustomSmartPointers Created.");
}
