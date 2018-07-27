// add this to lib in case we export later, but put it in main so we can use it here.
#![feature(try_from)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
