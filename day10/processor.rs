/// Solve day10 problem from Advent of Code 2022.

#[derive(Debug, PartialEq, Clone)]
pub enum MachineOp {
    Noop,
    Addx(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Processor {
    pub cycle: i64,
    pub reg: i64,
    pub program: Vec<MachineOp>,
    pub pc: usize,
    current_op_cycles_left: i32, // 0 at call to tick() means PC points to unstarted op, >=1 is additional cycles to spend on this op.
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            cycle: 1,
            reg: 1, 
            program: Vec::new(),
            pc: 0,
            current_op_cycles_left: 0,
        }
    }

    pub fn runnable(&self) -> bool {
        self.pc < self.program.len()
    }

    /// After a step, we have just completed cycle self.cycle, and not quite started cycle+1; self.reg is valid.
    pub fn tick(&mut self) {
        if !self.runnable() {
            return;
        }
        // Are we starting a new instruction:  then record how long it will take to finish.
        if self.current_op_cycles_left == 0 {
            self.current_op_cycles_left = match self.program[self.pc] {
                MachineOp::Noop => 1,
                MachineOp::Addx(_) => 2,
            }
        }

        // Work on the instruction for 1 cycle.
        self.current_op_cycles_left -= 1;
        self.cycle += 1;

        // Are we done?  If so, commit the result of the instruction.
        if self.current_op_cycles_left == 0 {
            match self.program[self.pc] {
                MachineOp::Noop => (),
                MachineOp::Addx(imm) => {
                    self.reg += imm as i64;
                }
            }
            self.pc += 1;
        }
    }
}

#[test]
fn test_processor() {
    let mut p = Processor::new();
    p.program.push(MachineOp::Noop);
    p.program.push(MachineOp::Addx(3));
    p.program.push(MachineOp::Addx(-5));

    assert!(p.runnable());
    assert!(p.cycle == 1);
    assert!(p.reg == 1);

    // Begin Noop.
    p.tick();
    assert!(p.cycle == 2);
    assert!(p.runnable());
    assert!(p.reg == 1);

    // Begin Addx 3.
    p.tick();
    assert!(p.cycle == 3);
    assert!(p.runnable());
    assert!(p.reg == 1);

    p.tick();
    assert!(p.cycle == 4);
    assert!(p.runnable());
    assert!(p.reg == 4);

    // running Addx -5.
    p.tick();
    assert!(p.cycle == 5);
    assert!(p.runnable());
    assert!(p.reg == 4);

    p.tick();
    assert!(p.cycle == 6);
    assert!(!p.runnable()); // PC points off end of program, so not runnable.
    assert!(p.reg == -1);
}

