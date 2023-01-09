use std::collections::VecDeque;

type ItemVal = isize;

type MonkeyId = usize;

type ItemOp = Box<dyn Fn(ItemVal) -> ItemVal>;

type RouteOp = Box<dyn Fn(ItemVal) -> MonkeyId>;

struct Monkey {
    id: usize,
    items: VecDeque<ItemVal>,
    item_op: ItemOp,
    route_op: RouteOp,
    inspections: u32,
}

impl Monkey {
    pub fn new(id: usize, items: VecDeque<ItemVal>, item_op: ItemOp, route_op: RouteOp) -> Self {
        Monkey {
            id: id,
            items: items,
            item_op: item_op,
            route_op: route_op,
            inspections: 0,
        }
    }

    pub fn new_from_lines(lineblockstr: &str) -> Monkey {
        let lines = lineblockstr.split("\n").collect::<Vec<&str>>();
        dbg!(&lines);
        assert!(lines.len() == 6 || (lines.len() == 7 && lines[6] == ""));
        // "Monkey 0:"
        let id = lines[0].chars().skip(7).take_while(|&c| c != ':').collect::<String>()
                        .parse::<MonkeyId>().expect(format!("Should parse id from {}", lines[0]).as_str());
        // "  Starting items: 79, 98"
        let items = lines[1].chars().skip_while(|&c| !c.is_digit(10)).collect::<String>()
                            .split(", ")
                            .map(|s| s.parse::<ItemVal>().expect("item value not u32"))
                            .collect::<VecDeque<ItemVal>>();
        // "  Operation: new = old * 19"
        let expr_str = lines[2]
            .chars()
            .skip(19)
            .collect::<String>();

        let mut expr = expr_str.split(" ");
        let _ = expr.next().unwrap();
        let operator = expr.next()
        .unwrap()
        .chars().next().unwrap();
        let operand2 = expr.next().unwrap().parse::<ItemVal>();
        let item_op = match (operator, operand2) {
            ('+', Ok(n)) => item_plus(n),
            ('*', Err(_)) => item_squared(),
            ('*', Ok(n)) => item_times(n),
            _ => panic!("bad expr")
            };
        // TODO: this could use find() and split_at() and perhaps be more idiomatic.
        // "  Test: divisible by 23"
        let divisor = lines[3].chars().skip_while(|&c| !c.is_digit(10)).collect::<String>().parse::<ItemVal>().expect("Should parse divisor");
        // "    If true: throw to monkey 2"
        let mtrue = lines[4].chars().skip_while(|&c| !c.is_digit(10)).collect::<String>().parse::<MonkeyId>().expect("Should parse true monkey");
        // "    If false: throw to monkey 3"
        let mfalse = lines[5].chars().skip_while(|&c| !c.is_digit(10)).collect::<String>().parse::<MonkeyId>().expect("Should parse false monkey");
        let route_op = divisible_router(divisor, mtrue, mfalse);
        Monkey::new(id, items, item_op, route_op)
    }
}

pub struct Monkeys {
    completed_round: usize,
    monkeys: Vec<Monkey>, // Priority queue maybe?
}

impl Monkeys {
    pub fn new() -> Self {
        Monkeys {
            completed_round: 0,
            monkeys: vec![],
        }
    }

    pub fn do_round(&mut self) {
        // During a round, monkeys take a turn in order id 0 to id n.
        // On a turn, a monkey handles each item in order.
        // When handling an item the monkey does:
        // 1. inspect (causes worry level == item val to change according to fn item_op)
        // 2. inspection ends (causes worry level == item val to be divided by three and rounded down to the nearest integer)
        // 3. monkey throws item to another monkey (according to route_fn) - it goes to end of recipient's list - throw of item and reciept of item happen atomically.
        let n_monkeys = self.monkeys.len();
        for i in 0..n_monkeys {
            // Device to allow two mutable references two different items in a slice, chosen at different times.
            //let mid_idx = i;
            //let (low_monkeys, rest) = self.monkeys.split_at_mut(i);
            //let (this_monkey, hi_monkeys) = rest.split_at_mut(1);
            //fn try_get_second_mut(second_idx: usize) -> &mut Monkey {
            //    match second_idx {
            //        0..mid_idx => &mut low_monkeys[second_idx],
            //        mid_idx => panic!("Can't borrow first item a second_time"),
            //        mid_idx + 1..n_monkeys => &mut hi_monkeys[second_idx-mid_idx-1],
            //    }
            //}
            let mut in_air: Vec<(usize, ItemVal)> = vec![];
            // First we process everything in the active monkey's queue, placing the thrown items in a temporary queue (in_air).
            // Later we put the in-air things into the target monkeys queues ("catch").
            // This isn't intended to create a delay or change ordering; in the problem statement, things spend no time in the air.
            // Rather, it is hard to convince rust to let you mutate two different vector elements (monkeys) in
            // the same scope.  Buffering the items in "in_air" allows separating the mutates into different scopes.
            {
                let this_monkey = &mut self.monkeys[i];
                assert!(this_monkey.id == i);
                for itemval in &this_monkey.items {
                    // Inspect
                    let newitemval = (this_monkey.item_op)(*itemval);
                    this_monkey.inspections += 1;
                    // End Inspect
                    let newnewitemval = newitemval / 3;
                    // Throw
                    let target_id = (this_monkey.route_op)(newnewitemval);
                    println!(
                        "monkey {} done-inspecting-itemval {} target {}",
                        i, newnewitemval, target_id
                    );

                    assert!(target_id < n_monkeys);
                    assert!(target_id != i);
                    in_air.push((target_id, newnewitemval));
                }
            }
            // Everything is thrown to a different monkey now.
            self.monkeys[i].items.clear();
            {
                for (idx, val) in in_air {
                    self.monkeys[idx].items.push_back(val);
                }
            }
        }
        self.completed_round += 1;
    }

    pub fn pretty(&self) -> String {
        let mut s: String;
        s = format!(
            "After round {}, the monkeys are holding items with these worry levels:\n",
            self.completed_round
        );
        for m in &self.monkeys {
            let item_str = m
                .items
                .iter()
                .map(|&i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            s.push_str(format!("Monkey {}: {}\n", m.id, item_str).as_str());
        }
        s
    }

    pub fn monkey_business(&self) -> u32 {
        let mut insp = self
            .monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<u32>>();
        insp.sort();
        insp.reverse();
        insp[0] * insp[1]
    }

    pub fn new_from_file(filelines: &str) -> Self {
        let mut m = Monkeys::new();
        let blocks: Vec<&str> = filelines.split("\n\n").collect();
        for block in blocks {
            let mm = Monkey::new_from_lines(block);
            m.monkeys.push(mm);
        }
        m
    }
}

fn item_times(n: isize) -> ItemOp {
    Box::new(move |i: ItemVal| -> ItemVal { i.checked_mul(n).unwrap() })
}

fn item_plus(n: isize) -> ItemOp {
    Box::new(move |i| -> ItemVal { i.checked_add(n).unwrap() })
}

fn item_squared() -> ItemOp {
    Box::new(move |i: ItemVal| -> ItemVal { i.checked_mul(i).unwrap() })
}

fn divisible_router(divisor: ItemVal, t: MonkeyId, f: MonkeyId) -> RouteOp {
    Box::new(move |i: ItemVal| -> MonkeyId {
        match i % divisor == 0 {
            true => t,
            false => f,
        }
    })
}

// Test the example from AoC Day 11 with explicitly constructed monkeys.
#[test]
fn test_monkeys_aoc_example_using_new() {
    let mut m = Monkeys::new();
    m.monkeys.push(Monkey::new(
        0,
        VecDeque::from([79, 98]),
        item_times(19),
        divisible_router(23, 2, 3),
    ));
    m.monkeys.push(Monkey::new(
        1,
        VecDeque::from([54, 65, 75, 74]),
        item_plus(6),
        divisible_router(19, 2, 0),
    ));
    m.monkeys.push(Monkey::new(
        2,
        VecDeque::from([79, 60, 97]),
        item_squared(),
        divisible_router(13, 1, 3),
    ));
    m.monkeys.push(Monkey::new(
        3,
        VecDeque::from([74]),
        item_plus(3),
        divisible_router(17, 0, 1),
    ));
    for _ in 1..=20 {
        print!("{}", m.pretty());
        m.do_round();
    }
    print!("{}", m.pretty());
    assert_eq!(m.completed_round, 20);
    assert_eq!(m.monkeys.len(), 4);
    assert_eq!(m.monkeys[0].items, [10, 12, 14, 26, 34]);
    assert_eq!(m.monkeys[1].items, [245, 93, 53, 199, 115]);
    assert_eq!(m.monkeys[2].items, []);
    assert_eq!(m.monkeys[3].items, []);
    assert_eq!(m.monkeys[0].items, [10, 12, 14, 26, 34]);
    assert_eq!(m.monkeys[1].items, [245, 93, 53, 199, 115]);
    assert_eq!(m.monkeys[2].items, []);
    assert_eq!(m.monkeys[3].items, []);
    assert_eq!(m.monkeys[0].inspections, 101);
    assert_eq!(m.monkeys[1].inspections, 95);
    assert_eq!(m.monkeys[2].inspections, 7);
    assert_eq!(m.monkeys[3].inspections, 105);
    assert_eq!(m.monkey_business(), 10605);
}

#[cfg(test)]
const MONKEY_STR: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";
#[test]
fn test_new_from_lines() {
    let mm = Monkey::new_from_lines(MONKEY_STR);
    assert_eq!(mm.id, 0);
    assert_eq!(mm.items, VecDeque::from([79, 98]));
}

#[cfg(test)]
const AOC_TEST_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";


// Test parsing a file of monkeys.
#[test]
fn test_monkeys_new_from_file() {
    let m = Monkeys::new_from_file(AOC_TEST_INPUT);

    assert_eq!(m.monkeys.len(), 4);
}

// Test the example from AoC Day 11 with parsed monkeys.
#[test]
fn test_aoc_prob1() {
    let mut m = Monkeys::new_from_file(AOC_TEST_INPUT);

    for _ in 1..=20 {
        print!("{}", m.pretty());
        m.do_round();
    }
    assert_eq!(m.monkey_business(), 10605);
}
