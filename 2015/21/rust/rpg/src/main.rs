use itertools::iproduct;

#[derive(Clone, Debug)]
struct Weapon {
    name: String,
    cost: usize,
    damage: usize,
}

#[derive(Clone, Debug)]
struct Armor {
    name: String,
    cost: usize,
    armor: usize,
}

#[derive(Clone, Debug)]
enum Ring {
    Damage {
        name: String,
        cost: usize,
        damage: usize,
    },
    Defense {
        name: String,
        cost: usize,
        armor: usize,
    },
}

impl Ring {
    fn cost(&self) -> usize {
        match self {
            Ring::Damage {
                name: _,
                cost,
                damage: _,
            } => *cost,
            Ring::Defense {
                name: _,
                cost,
                armor: _,
            } => *cost,
        }
    }

    fn damage(&self) -> usize {
        match self {
            Ring::Damage {
                name: _,
                cost: _,
                damage,
            } => *damage,
            _ => 0,
        }
    }

    fn armor(&self) -> usize {
        match self {
            Ring::Defense {
                name: _,
                cost: _,
                armor,
            } => *armor,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug)]
enum PlayerRing {
    NoRing,
    OneRing(Ring),
    TwoRings(Ring, Ring),
}

#[derive(Debug)]
struct Player {
    hit_points: usize,
    weapon: Weapon,
    armor: Option<Armor>,
    rings: PlayerRing,
}

struct Boss {
    hit_points: usize,
    damage: usize,
    armor: usize,
}

fn main() {
    let weapons = vec![
        Weapon {
            name: "Dagger".to_owned(),
            cost: 8,
            damage: 4,
        },
        Weapon {
            name: "Shortsword".to_owned(),
            cost: 10,
            damage: 5,
        },
        Weapon {
            name: "Warhammer".to_owned(),
            cost: 25,
            damage: 6,
        },
        Weapon {
            name: "Longsword".to_owned(),
            cost: 40,
            damage: 7,
        },
        Weapon {
            name: "Greataxe".to_owned(),
            cost: 74,
            damage: 8,
        },
    ];
    let armors = vec![
        Armor {
            name: "Leather".to_owned(),
            cost: 13,
            armor: 1,
        },
        Armor {
            name: "Chainmail".to_owned(),
            cost: 31,
            armor: 2,
        },
        Armor {
            name: "Splintmail".to_owned(),
            cost: 53,
            armor: 3,
        },
        Armor {
            name: "Bandedmail".to_owned(),
            cost: 75,
            armor: 4,
        },
        Armor {
            name: "Platemail".to_owned(),
            cost: 102,
            armor: 5,
        },
    ];
    let rings = vec![
        Ring::Damage {
            name: "Damage +1".to_owned(),
            cost: 25,
            damage: 1,
        },
        Ring::Damage {
            name: "Damage +2".to_owned(),
            cost: 50,
            damage: 2,
        },
        Ring::Damage {
            name: "Damage +3".to_owned(),
            cost: 100,
            damage: 3,
        },
        Ring::Defense {
            name: "Defense +1".to_owned(),
            cost: 20,
            armor: 1,
        },
        Ring::Defense {
            name: "Defense +2".to_owned(),
            cost: 40,
            armor: 2,
        },
        Ring::Defense {
            name: "Defense +3".to_owned(),
            cost: 80,
            armor: 3,
        },
    ];

    let boss = Boss {
        hit_points: 109,
        damage: 8,
        armor: 2,
    };

    let most_efficient_winning_player: Player = iproduct!(weapons.iter(), armors.iter())
        .flat_map(|(weapon, armor)| {
            let armor_combinations = vec![None, Some(armor.clone())];

            let ring_combinations = vec![PlayerRing::NoRing]
                .into_iter()
                .chain(rings.iter().map(|r| PlayerRing::OneRing(r.clone())))
                .chain(
                    iproduct!(rings.iter().enumerate(), rings.iter().enumerate())
                        .filter(|((i, _), (j, _))| i != j)
                        .map(|((_, left), (_, right))| {
                            PlayerRing::TwoRings(left.clone(), right.clone())
                        }),
                );

            iproduct!(armor_combinations, ring_combinations).map(|(armor, rings)| Player {
                hit_points: 100,
                weapon: weapon.clone(),
                armor,
                rings,
            })
        })
        .filter(|player| can_beat(player, &boss))
        .min_by_key(|player| {
            let armor_cost = if let Some(a) = &player.armor {
                a.cost
            } else {
                0_usize
            };
            let rings_cost = match &player.rings {
                PlayerRing::NoRing => 0_usize,
                PlayerRing::OneRing(r) => r.cost(),
                PlayerRing::TwoRings(left, right) => left.cost() + right.cost(),
            };
            player.weapon.cost + armor_cost + rings_cost
        })
        .unwrap();

    println!(
        "most_efficient_winning_player = {:?}",
        most_efficient_winning_player
    );
}

fn can_beat(player: &Player, boss: &Boss) -> bool {
    let player_armor = if let Some(a) = &player.armor {
        a.armor
    } else {
        0_usize
    };

    let player_rings_damage = match &player.rings {
        PlayerRing::OneRing(ring) => ring.damage(),
        PlayerRing::TwoRings(left, right) => left.damage() + right.damage(),
        _ => 0,
    };

    let player_rings_armor = match &player.rings {
        PlayerRing::OneRing(ring) => ring.armor(),
        PlayerRing::TwoRings(left, right) => left.armor() + right.armor(),
        _ => 0,
    };

    let player_deal = std::cmp::max(
        1,
        player.weapon.damage as i64 + player_rings_damage as i64 - boss.armor as i64,
    ) as usize;
    let boss_deal = std::cmp::max(
        1,
        boss.damage as i64 - player_armor as i64 - player_rings_armor as i64,
    ) as usize;

    let player_required_turns = boss.hit_points / player_deal;
    let boss_required_turns = player.hit_points / boss_deal;

    player_required_turns <= boss_required_turns
}
