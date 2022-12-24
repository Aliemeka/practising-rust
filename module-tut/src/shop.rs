pub const STORE_NUMBER: u32 = 20;
pub mod store {
    pub fn is_store_open() -> bool {
        false
    }

    pub fn is_neighbours_open() -> bool {
        use super::store2;
        store2::is_store_open()
    }
}
pub mod store2 {
    pub fn is_store_open() -> bool {
        true
    }
}
