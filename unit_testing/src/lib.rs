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

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_two_servants() {
        let hero = Hero { id: 1, name: "安倍晴明".to_string(), is_contracted_servant: true};
        let villain = Villain { id: 1, name: "芦屋道満".to_string(), is_contracted_servant: false };
    
        let character1 = Character::Hero(hero);
        let character2 = Character::Villain(villain);
    
        assert_eq!(Character::identify_hero(&character1), true);
        assert_eq!(Character::identify_hero(&character2), false);
    }
}