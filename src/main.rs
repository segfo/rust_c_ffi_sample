extern crate libc;

extern {
    fn test_c();
    fn test_c2();
}


fn main() {
    unsafe{
        test_c();
        test_c2();
    }
}
