pub mod addr {
    pub fn add_one(i: u32) -> u32 {
        i + 1
    }
}


#[cfg(test)]
mod tests {
    use crate::addr::add_one;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}
