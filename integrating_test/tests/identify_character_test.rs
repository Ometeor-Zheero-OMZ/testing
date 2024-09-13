// Integrating test

use integrating_test::{Character, CharacterTrait, Hero, Villain};

#[test]
fn identify_hero_test() {
    let hero = Hero { id: 1, name: "安倍晴明".to_string(), is_contracted_servant: true};
    let villain = Villain { id: 1, name: "芦屋道満".to_string(), is_contracted_servant: false };

    let character1 = Character::Hero(hero);
    let character2 = Character::Villain(villain);

    assert_eq!(Character::identify_hero(&character1), true);
    assert_eq!(Character::identify_hero(&character2), false);
}