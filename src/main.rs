use rust_ffi::call_cpp;

fn main() {
    call_cpp::ffi::new_cpp_class();
    let result = call_cpp::ffi::c_add(5, 7);
    println!("Result of addition: {}", result);
}
