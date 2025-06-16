pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}
// FIXME: continue with extracting hosting into its own file
// in section 7.5 Separating Modules into Different Files

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
