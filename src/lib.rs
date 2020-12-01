pub struct Device {
    pub registers: [i64; 6],
    ip: IP,
    program: Vec<(i64, i64, i64, i64)>,
}

#[derive(Debug, Clone, Copy)]
enum IP {
    Immed(usize),
    Shadow(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct TestCase {
    pub before: [i64; 4],
    pub opcode: [i64; 4],
    pub after: [i64; 4],
}

impl TestCase {
    fn before(&self) -> [i64; 6] {
        [self.before[0], self.before[1], self.before[2], self.before[3], 0, 0]
    }

    fn after(&self) -> [i64; 6] {
        [self.after[0], self.after[1], self.after[2], self.after[3], 0, 0]
    }
}

impl Device {
    pub fn new() -> Self {
        Device { registers: [0; 6], ip: IP::Immed(0), program: Vec::new() }
    }

    pub fn reset(&mut self) {
        self.registers = [0; 6];
        self.ip = IP::Immed(0);
    }

    pub fn test_case(&mut self, t: &TestCase) -> i64 {
        let mut matches = 0;

        self.registers = t.before();
        self.addr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.addi(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.mulr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.muli(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.banr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.bani(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.borr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.bori(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.setr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.seti(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.gtir(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.gtri(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.gtrr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.eqir(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.eqri(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        self.registers = t.before();
        self.eqrr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after() { 1 } else { 0 };

        matches
    }

    pub fn exec(&mut self, o: i64, a: i64, b: i64, c: i64) {
        match o {
            0  => self.banr(a, b, c),
            1  => self.eqrr(a, b, c),
            2  => self.setr(a, b, c),
            3  => self.eqir(a, b, c),
            4  => self.bori(a, b, c),
            5  => self.muli(a, b, c),
            6  => self.bani(a, b, c),
            7  => self.borr(a, b, c),
            8  => self.gtir(a, b, c),
            9  => self.gtrr(a, b, c),
            10 => self.addi(a, b, c),
            11 => self.gtri(a, b, c),
            12 => self.eqri(a, b, c),
            13 => self.addr(a, b, c),
            14 => self.mulr(a, b, c),
            15 => self.seti(a, b, c),
            _ => panic!("Unrecognized opcode {}", o),
        }
    }

    fn map_opcode(op: &str) -> i64 {
        match op {
            "banr" => 0,
            "eqrr" => 1,
            "setr" => 2,
            "eqir" => 3,
            "bori" => 4,
            "muli" => 5,
            "bani" => 6,
            "borr" => 7,
            "gtir" => 8,
            "gtrr" => 9,
            "addi" => 10,
            "gtri" => 11,
            "eqri" => 12,
            "addr" => 13,
            "mulr" => 14,
            "seti" => 15,
            _ => panic!("Unrecognized opcode {}", op),
        }
    }

    fn opcode_as_str(op: i64) -> &'static str {
        match op {
            0 => "banr",
            1 => "eqrr",
            2 => "setr",
            3 => "eqir",
            4 => "bori",
            5 => "muli",
            6 => "bani",
            7 => "borr",
            8 => "gtir",
            9 => "gtrr",
            10 => "addi",
            11 => "gtri",
            12 => "eqri",
            13 => "addr",
            14 => "mulr",
            15 => "seti",
            _ => panic!("Unrecognized opcode {}", op),
        }
    }

    pub fn shadow_ip(&mut self, n: usize) {
        self.ip = IP::Shadow(n);
    }

    pub fn push(&mut self, op_line: &str) {
        let mut parts = op_line.split(" ");
        let op = parts.next().unwrap();
        let op = Device::map_opcode(op);
        let a = parts.next().unwrap().parse::<i64>().unwrap();
        let b = parts.next().unwrap().parse::<i64>().unwrap();
        let c = parts.next().unwrap().parse::<i64>().unwrap();
        self.program.push((op, a, b, c));
    }

    pub fn run_all(&mut self) {
        loop {
            let i = self.ip();

            if i >= self.program.len() {
                break;
            }

            let line = self.program[i];

            self.exec(line.0, line.1, line.2, line.3);
            self.inc_ip();
        }
    }

    fn inc_ip(&mut self) {
        match self.ip {
            IP::Immed(n) => self.ip = IP::Immed(n + 1),
            IP::Shadow(n) => self.registers[n] += 1,
        }
    }

    fn ip(&self) -> usize {
        match self.ip {
            IP::Immed(n) => n as usize,
            IP::Shadow(n) => self.registers[n] as usize,
        }
    }

    fn addr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va + vb;
    }

    fn addi(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va + vb;
    }

    fn mulr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va * vb;
    }

    fn muli(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va * vb;
    }

    fn banr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va & vb;
    }

    fn bani(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va & vb;
    }

    fn borr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va | vb;
    }

    fn bori(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va | vb;
    }

    fn setr(&mut self, a: i64, _b: i64, c: i64) {
        let va = self.registers[a as usize];
        self.registers[c as usize] = va;
    }

    fn seti(&mut self, a: i64, _b: i64, c: i64) {
        let va = a;
        self.registers[c as usize] = va;
    }

    fn gtir(&mut self, a: i64, b: i64, c: i64) {
        let va = a;
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va > vb { 1 } else { 0 };
    }

    fn gtri(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = if va > vb { 1 } else { 0 };
    }

    fn gtrr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va > vb { 1 } else { 0 };
    }

    fn eqir(&mut self, a: i64, b: i64, c: i64) {
        let va = a;
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va == vb { 1 } else { 0 };
    }

    fn eqri(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = if va == vb { 1 } else { 0 };
    }

    fn eqrr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va == vb { 1 } else { 0 };
    }
}

