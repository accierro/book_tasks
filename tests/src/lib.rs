#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!("Hello", "Hello")
    }

    #[test]
    fn it_fails() {
        panic!("Any test function which panics will be marked as failed. This test will fail.")
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            height: 8,
        };
        let smaller = Rectangle {
            width: 1,
            height: 7,
        };

        assert!(larger.can_hold(&smaller));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
