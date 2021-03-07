///
pub trait Encoder {
    fn get_speed(&self) -> f32;
    fn get_position_in_revolution(&self) -> f32;
    fn get_position() -> f32;

    /// A function to
    fn sample();
}

// #[cfg(test)]
mod tests {
    use super::*;

    struct MockEncoder {}

    impl Encoder for MockEncoder {
        fn get_speed(&self) -> f32 {
            unimplemented!()
        }

        fn get_position_in_revolution(&self) -> f32 {
            unimplemented!()
        }

        fn get_position() -> f32 {
            unimplemented!()
        }

        fn sample() {
            unimplemented!()
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
