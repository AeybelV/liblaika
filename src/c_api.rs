#[no_mangle]
pub extern "C" fn liblaika_hello() {
    crate::liblaika::liblaika_hello();
}
