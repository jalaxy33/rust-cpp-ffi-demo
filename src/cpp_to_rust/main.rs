use rust_ffi::ffi;


fn main() {
    ffi::new_cpp_class();
    let result = ffi::c_add(5, 7);
    println!("Result of addition: {}", result);
}
