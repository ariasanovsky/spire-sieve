use crate::character::Character;

pub mod card;
pub mod card_pool;

const IRONCLAD: u8 = Character::Ironclad as u8;
const SILENT: u8 = Character::Silent as u8;
const DEFECT: u8 = Character::Defect as u8;
const WATCHER: u8 = Character::Watcher as u8;

#[derive(Debug, Eq, PartialEq)]
pub struct AnonymousCard<const CHARACTER: u8>(usize);

impl<const CHARACTER: u8> AnonymousCard<CHARACTER> {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum AnonymouslyRareCard<const CHARACTER: u8> {
    Common(AnonymousCard<CHARACTER>),
    Uncommon(AnonymousCard<CHARACTER>),
    Rare(AnonymousCard<CHARACTER>),
}

#[derive(Debug, Eq, PartialEq)]
enum Card<const CHARACTER: u8> {}

// #[derive(Debug, Eq, PartialEq)]
// struct CardPool<const CHARACTER: u8> {
//     commons: &'static 
// }

#[test]
fn byte_sized_enums() {
    assert_eq!(std::mem::size_of::<Character>(), 1);
    assert_eq!(
        std::mem::size_of::<AnonymousCard<IRONCLAD>>(),
        std::mem::size_of::<usize>()
    );
    assert_eq!(
        std::mem::size_of::<AnonymouslyRareCard<IRONCLAD>>(),
        8 + std::mem::size_of::<AnonymousCard<IRONCLAD>>()
    );

    let foo = Character::Ironclad as usize;
}