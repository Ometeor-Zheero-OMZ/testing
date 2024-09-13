pub struct Hero {
    pub id: u16,
    pub name: String,
    pub is_contracted_servant: bool
}
pub struct Villain {
    pub id: u16,
    pub name: String,
    pub is_contracted_servant: bool
}

pub enum Character {
    Hero(Hero),
    Villain(Villain)
}

pub trait CharacterTrait {
    fn identify_hero(character: &Character) -> bool;
}

impl CharacterTrait for Character {
    fn identify_hero(character: &Character) -> bool {
        match character {
            Character::Hero(hero) => {
                if hero.is_contracted_servant {
                    true
                } else {
                    false
                }
            }
            Character::Villain(_) => false
        }
    }
}