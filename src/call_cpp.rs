#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cpp_to_rust.h");

        pub type CppClass;

        pub fn new_cpp_class() -> UniquePtr<CppClass>;

        pub fn c_add(a: i32, b: i32) -> i32;
    }
}
