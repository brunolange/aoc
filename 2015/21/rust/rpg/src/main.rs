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

#[derive(Debug)]
struct Player {
    hit_points: usize,
    weapon: Weapon,
    armor: Option<Armor>,
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

    let boss = Boss {
        hit_points: 12,
        damage: 7,
        armor: 2,
    };

    let most_efficient_winning_player = iproduct!(weapons, armors)
        .flat_map(|(weapon, armor)| {
            vec![
                Player {
                    hit_points: 100,
                    weapon: weapon.clone(),
                    armor: Some(armor),
                },
                Player {
                    hit_points: 100,
                    weapon: weapon,
                    armor: None,
                },
            ]
        })
        .filter(|player| can_beat(player, &boss))
        .min_by_key(|player| {
            let armor_cost = if let Some(a) = &player.armor {
                a.cost
            } else {
                0_usize
            };
            player.weapon.cost + armor_cost
        });

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

    let player_deal = std::cmp::min(1, player.weapon.damage - boss.armor);
    let boss_deal = std::cmp::min(1, boss.damage - player_armor);

    let player_required_turns = boss.hit_points / player_deal;
    let boss_required_turns = player.hit_points / boss_deal;

    player_required_turns <= boss_required_turns
}
