use crate::card::Card;

// Check if the given 3 cards form a set
pub fn is_set(card1: &Card, card2: &Card, card3: &Card) -> bool {
    // Check all 4 properties
    if !all_same_or_all_different(&card1.shape, &card2.shape, &card3.shape) {
        return false;
    }

    if !all_same_or_all_different(&card1.color, &card2.color, &card3.color) {
        return false;
    }

    if !all_same_or_all_different(&card1.count, &card2.count, &card3.count) {
        return false;
    }

    if !all_same_or_all_different(&card1.fill, &card2.fill, &card3.fill) {
        return false;
    }

    println!("SET:\n{:?},\n{:?},\n{:?}", card1, card2, card3);

    true
}

fn all_same_or_all_different<T: PartialEq>(a: &T, b: &T, c: &T) -> bool {
    (a != b && b != c && c != a) || (a == b && b == c && c == a)
}
