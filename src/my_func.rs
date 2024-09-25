pub fn add_five(num: u32) -> u32 {
    num + 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test() {
        let x: u32 = 100;
        let y: u32 = add_five(x);
        println!("test results x = {x}, y = {y}");
        assert_eq!(y, 105);
    }
}
