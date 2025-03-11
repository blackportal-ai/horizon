pub mod api;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub use api::state::HorizonState as HorizonCliState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
