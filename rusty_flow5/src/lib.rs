#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rusty_flow5/cpp/wrapper.h");

        type Foil;

        fn foo();
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;

    #[test]
    fn test_foo() {
        // Call `foo()` to ensure the C++ binding links and runs.
        ffi::foo();
    }
}