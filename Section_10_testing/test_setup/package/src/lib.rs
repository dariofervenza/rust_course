pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]  // cfg is configuration, so we are telling rust to only use what it needs for a test module
mod tests {
    use super::*; // this will include all the functions from the outside, so it will include the simple_add()

    #[test]  // this tells rust compiler that the following is a test --> run it with cargo test
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    #[ignore]  // this will ignore this test
    fn it_fails(){
        panic!("Test failed");
    }
    // we can use assertations to test things

    // assert to true
    #[test]
    fn call_simple_add(){
        assert!(simple_add());
    }

    // assert not equal
    #[test]
    fn assert_not_eq() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }

    // assert it panics
    #[test]
    #[should_panic]
    fn it_fails_panic(){
        panic!("Test failed");
    }

    // assert that an error is caught
    // cargo test -- --test-threads=4  // if you have expensibe tests, to make them faster

    // when you have hundreds or thousands of test you may want to only run part of them

    // cargo test assert_not_eq  // only rins that test

    // run a subset

    // cargo test it  // it will run the test functions that starts with it like it_fails_panic or it_fails
}


fn simple_add() -> bool {
    if 2 +2 == 4 {
        true
    } else {
        false
    }
}