use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_rand(x: i32) -> i32 {
    x + rand::thread_rng().gen_range(1..=100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2))
    }
}
