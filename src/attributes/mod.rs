/// Chapter 4.27 - Attributes
/// [Chapter 4.27] https://doc.rust-lang.org/book/attributes.html

/// #[test] - attributes for item
/// #![test] - attributes for block
pub mod attributes {
    #[test]
    fn check() {
        assert_eq!(2, 1 + 1);
    }
}