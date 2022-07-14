fn main() {
    let mut _mutable = 8u32;
    {
        let _mutable = _mutable;
        // _mutable = 7u32; // error
    }
    _mutable = 6u32;
}
