use enum_iterator::{all, All};

use crate::card::card::*;

#[test]
fn foo() {
    let cards: All<Card> = all::<Card>();
    /*
        const CARDS: All<Card> = all::<Card>();
        does not compile :(
    */
}