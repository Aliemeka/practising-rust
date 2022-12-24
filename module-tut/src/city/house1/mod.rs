pub fn is_house_sold() -> bool {
    false
}

pub fn is_house_available() -> bool {
    !is_house_sold()
}

pub fn is_next_house_for_sale() -> bool {
    use super::house2;
    house2::is_house_available()
}
