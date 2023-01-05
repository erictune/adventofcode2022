/// Solve day10 problem from Advent of Code 2022.
mod processor;
mod crt;

use crate::processor::Processor;
use crate::processor::MachineOp;
use crate::crt::Crt;

pub fn do_day10(input: &str, mode: i32) -> String {
    let _ = match mode {
        1 | 2 => (),
        _ => panic!("Unknown mode"),
    };
    // Load the program.
    let mut processor: Processor = Processor::new();

    let mut lines = input.split("\n");
    for line in lines.by_ref().take_while(|&l| l.len() > 0) {
        if line == "noop" {
            processor.program.push(MachineOp::Noop);
            continue;
        }
        let lineparts = line.split(" ").collect::<Vec<&str>>();
        if lineparts.len() == 2 && lineparts[0] == "addx" {
            let imm = lineparts[1]
                .parse::<i32>()
                .expect("Invalid number after 'addx'.");
            processor.program.push(MachineOp::Addx(imm));
        } else {
            panic!("Invalid instruction.");
        }
    }
    if let Some(_) = lines.next() {
        panic!("Unexpected non-blank line after blank line.");
    }

    // Run the program.
    if mode == 1 {
        let mut signal_strength = 0;
        while processor.runnable() {
            processor.tick();
            if processor.cycle % 40 == 20  && processor.cycle <= 220 {
                signal_strength += processor.cycle * processor.reg;
            }
        }
        return String::from(format!("{}", signal_strength));
    } else {
        let mut crt = Crt::new();
        while processor.runnable() {
            // crt draws using latched sprite posn.
            crt.tick();
            // meanwhile, processor computes this cycles value.
            processor.tick();
            crt.set_sprite(processor.reg);
        }
        let image = crt.as_multiline_string();
        return String::from(format!("{}", image));
    }
}

#[cfg(test)]
const AOC_TEST_INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";


#[test]
fn test_do_day10_prob1_test_input() {
    let expected = format!("{}", 13140);
    let actual = do_day10(AOC_TEST_INPUT, 1);
    assert_eq!(String::from(expected), actual);
}

#[cfg(test)]
const AOC_TEST_PROB2_OUTPUT: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

#[test]
fn test_do_day10_prob2_test_input() {
    let actual = do_day10(AOC_TEST_INPUT, 2);
    let expected = String::from(AOC_TEST_PROB2_OUTPUT);
    println!("actual:\n{}", actual);
    println!("expected:\n{}", expected);

    assert_eq!(expected, actual);
}

