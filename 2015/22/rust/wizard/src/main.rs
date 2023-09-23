use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct Effect {
    duration: usize,
    armor: usize,
    damage: usize,
    mana: usize,
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct Spell {
    name: String,
    cost: usize,
    damage: usize,
    heal: usize,
    effect: Option<Effect>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Player {
    hit_points: usize,
    mana: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Boss {
    hit_points: usize,
    damage: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Turn {
    Player,
    Boss,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    cost: usize,
    turn: Turn,
    player: Player,
    boss: Boss,
    active_spells: HashMap<Spell, usize>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let spells = vec![
        Spell {
            name: String::from("Magic Missile"),
            cost: 53,
            damage: 4,
            heal: 0,
            effect: None,
        },
        Spell {
            name: String::from("Drain"),
            cost: 73,
            damage: 2,
            heal: 2,
            effect: None,
        },
        Spell {
            name: String::from("Shield"),
            cost: 113,
            damage: 0,
            heal: 0,
            effect: Some(Effect {
                duration: 6,
                armor: 7,
                damage: 0,
                mana: 0,
            }),
        },
        Spell {
            name: String::from("Poison"),
            cost: 173,
            damage: 0,
            heal: 0,
            effect: Some(Effect {
                duration: 6,
                damage: 3,
                armor: 0,
                mana: 0,
            }),
        },
        Spell {
            name: String::from("Recharge"),
            cost: 229,
            damage: 0,
            heal: 0,
            effect: Some(Effect {
                duration: 5,
                damage: 0,
                armor: 0,
                mana: 101,
            }),
        },
    ];

    let start = Node {
        cost: 0,
        turn: Turn::Player,
        player: Player {
            hit_points: 50,
            mana: 500,
        },
        boss: Boss {
            hit_points: 71,
            damage: 10,
        },
        active_spells: HashMap::new(),
    };

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(start);

    let mut count = 0;
    let mut final_node: Option<Node> = None;
    while !priority_queue.is_empty() {
        let node = priority_queue.pop().unwrap();
        println!("reached node: {:?}", node);

        let spell_damage = node
            .active_spells
            .iter()
            .map(|(spell, _)| spell.damage)
            .sum::<usize>();

        if node.boss.hit_points <= spell_damage {
            // TODO: read this out loud. makes no sense
            println!("player wins!");
            final_node = Some(node);
            break;
        }

        if node.player.hit_points == 0 {
            // player is killed, discard this node.
            println!("player got nuked in this timeline... trying another one");
            continue;
        }

        // let mut active_spells = node.active_spells.clone();
        // active_spells.insert(
        //     spell.clone(),
        //     spell.effect.map(|effect| effect.duration).unwrap_or(0),
        // );
        let decremented_active_spells: HashMap<Spell, usize> = node
            .active_spells
            .iter()
            .filter(|(_, remaining_turns)| **remaining_turns > 1)
            .map(|(spell, remaining_turns)| (spell.clone(), remaining_turns - 1))
            .collect();

        match node.turn {
            Turn::Boss => {
                // When it's the boss' turn, there's only transition: the boss attacks, inflicting damage on the player.
                let next = Node {
                    cost: node.cost,
                    turn: Turn::Player,
                    player: Player {
                        hit_points: node.player.hit_points
                            - std::cmp::min(node.player.hit_points, node.boss.hit_points),
                        mana: node.player.mana,
                    },
                    boss: Boss {
                        hit_points: node.boss.hit_points - 0usize,
                        damage: node.boss.damage,
                    },
                    active_spells: decremented_active_spells,
                };
                priority_queue.push(next);
            }
            Turn::Player => {
                let available_spells: Vec<Spell> = spells
                    .clone()
                    .into_iter()
                    .filter(|spell| !node.active_spells.contains_key(spell))
                    .filter(|spell| spell.cost <= node.player.mana)
                    .collect();

                if available_spells.len() == 0 {
                    // player can't cast a spell, end of the line.
                    println!("player can't cast a spell... try other timeline");
                    continue;
                }

                for spell in available_spells {
                    println!("Adding transition with spell: {:?}", spell);
                    let mut next_active_spells = decremented_active_spells.clone();
                    next_active_spells.insert(
                        spell.clone(),
                        spell.effect.map(|effect| effect.duration).unwrap_or(1),
                    );
                    let next = Node {
                        cost: node.cost + spell.cost,
                        turn: Turn::Boss,
                        player: Player {
                            hit_points: node.player.hit_points,
                            mana: node.player.mana - spell.cost,
                        },
                        boss: Boss {
                            hit_points: node.boss.hit_points
                                - std::cmp::min(node.boss.hit_points, spell.damage),
                            damage: node.boss.damage,
                        },
                        active_spells: next_active_spells,
                    };
                    priority_queue.push(next);
                }
            }
        }

        count += 1;
        if count >= 100 {
            panic!("stop this madness....")
        }
    }
    // let start = Node { depth: 0 };
    // let start = Turn::P(player.clone());
    // let available_spells = spells.into_iter().filter(|spell| spell.cost <= player.mana);
    // let mut priority_queue = BinaryHeap::new();
    // for spell in available_spells {
    //     priority_queue.push(State {
    //         cost: spell.cost,
    //         turn: Turn::B(Boss {
    //             // hit_points: boss.hit_points -
    //         }),
    //     })
    // }

    // println!("spells = {:?}", spells);
    println!("final_node = {:?}", final_node);
}

// shortest path to victory?
// every node represents a turn, every edge represents a cast spell. since each spell has a cost, there's a weight
// associated to the edge.
// need to BFS that graph.

// the attacker can be determined by the depth of the node.
// perhaps in antecipation of more complicated scenarios, with either skipped turns or more players,
// it might be a good idea to just store who's currently attacking at each node.
