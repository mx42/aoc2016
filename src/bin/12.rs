advent_of_code::solution!(12);

// Could do with just a `char` but less place for confusion that way
#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

// Well... clearly it would be easier to just use usize... Buuuut. Less confusing...
impl Register {
    fn idx(self) -> usize {
        match self {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::D => 3,
        }
    }
}

impl TryFrom<&&str> for Register {
    type Error = ();
    fn try_from(value: &&str) -> Result<Self, Self::Error> {
        match *value {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum Value {
    Reg(Register),
    Const(i8),
}

impl TryFrom<&&str> for Value {
    type Error = ();
    fn try_from(value: &&str) -> Result<Self, Self::Error> {
        Register::try_from(value)
            .map(Value::Reg)
            .or_else(|_| value.parse::<i8>().map(Value::Const).map_err(|_| ()))
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Value::Reg(r) => write!(f, "[{:?}]", r)?,
            Value::Const(c) => write!(f, "{:>3?}", c)?,
        }
        Ok(())
    }
}

#[derive(Clone)]
enum Instr {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Value, Value),
}

impl TryFrom<&str> for Instr {
    type Error = ();
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let token: Vec<&str> = line.split_whitespace().collect();
        match token.as_slice() {
            ["cpy", src, dst] => Ok(Instr::Copy(src.try_into()?, dst.try_into()?)),
            ["inc", reg] => Ok(Instr::Increment(reg.try_into()?)),
            ["dec", reg] => Ok(Instr::Decrement(reg.try_into()?)),
            ["jnz", val, offset] => Ok(Instr::JumpNotZero(val.try_into()?, offset.try_into()?)),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instr::Copy(val, reg) => write!(f, "CPY {:>3?} -> [{:?}]", val, reg)?,
            Instr::Increment(reg) => write!(f, "INC [{:?}]", reg)?,
            Instr::Decrement(reg) => write!(f, "DEC [{:?}]", reg)?,
            Instr::JumpNotZero(Value::Const(0), offset) => {
                write!(f, "JNZ   0 off: {:?} -> NOOP", offset)?
            }
            Instr::JumpNotZero(cond, offset) => write!(f, "JNZ {:3?} off: {:?}", cond, offset)?,
        }
        Ok(())
    }
}

struct State {
    rs: [i64; 4],
    ins: Vec<Instr>,
    ins_ptr: i8,
}

impl State {
    fn init(ins: Vec<Instr>) -> Self {
        Self {
            rs: [0; 4],
            ins,
            ins_ptr: 0,
        }
    }

    fn step(&mut self) -> bool {
        if self.ins_ptr as usize >= self.ins.len() || self.ins_ptr < 0 {
            return false;
        }
        match &self.ins[self.ins_ptr as usize] {
            Instr::Copy(Value::Const(c), dst) => self.rs[dst.idx()] = *c as i64,
            Instr::Copy(Value::Reg(src), dst) => self.rs[dst.idx()] = self.rs[src.idx()],
            Instr::Increment(reg) => self.rs[reg.idx()] += 1,
            Instr::Decrement(reg) => self.rs[reg.idx()] -= 1,
            Instr::JumpNotZero(Value::Const(0), _) => (),
            Instr::JumpNotZero(Value::Const(_), Value::Const(c)) => self.ins_ptr += c - 1,
            Instr::JumpNotZero(Value::Const(_), Value::Reg(reg)) => {
                self.ins_ptr += (self.rs[reg.idx()] - 1) as i8
            }
            Instr::JumpNotZero(Value::Reg(r), offset) => {
                if self.rs[r.idx()] != 0 {
                    self.ins_ptr += match offset {
                        Value::Const(c) => c - 1,
                        Value::Reg(r_off) => (self.rs[r_off.idx()] - 1) as i8,
                    }
                }
            }
        }
        self.ins_ptr += 1;
        true
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Reg:  [ A ] [ B ] [ C ] [ D ]")?;
        write!(f, "     ")?;
        for r in self.rs {
            write!(f, " [{:^3?}]", r)?;
        }
        writeln!(f, "\nProgram:")?;
        for (n, ins) in self.ins.clone().into_iter().enumerate() {
            if n as i8 == self.ins_ptr {
                write!(f, "-> ")?;
            } else {
                write!(f, "   ")?;
            }
            writeln!(f, "{:>3}: {:?}", n, ins)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let instructions: Vec<Instr> = input.lines().flat_map(Instr::try_from).collect();
    let mut state = State::init(instructions);
    // println!("{:?}", state);
    while state.step() {
        // println!("{:?}", state);
    }
    Some(state.rs[0])
}

pub fn part_two(input: &str) -> Option<i64> {
    let instructions: Vec<Instr> = input.lines().flat_map(Instr::try_from).collect();
    let mut state = State::init(instructions);
    state.rs[Register::C.idx()] = 1;
    // println!("{:?}", state);
    while state.step() {
        // println!("{:?}", state);
    }
    Some(state.rs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
