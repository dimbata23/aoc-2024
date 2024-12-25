use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    println!("Part one result: {res_part1}");

    let res_part2 = calculate_part2(&input);
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &State) -> u64 {
    input.clone().calc_wires_starting_with("z")
}

fn calculate_part2(input: &State) -> String {
    "".to_string()
}

type Wire = String;

#[derive(Debug, Copy, Clone)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Connection {
    from: (Wire, Wire),
    gate: Gate,
}

#[derive(Debug, Clone)]
struct State {
    wire_values: HashMap<Wire, bool>,
    connections: HashMap<Wire, Connection>,
}

impl State {
    fn calc_wires_starting_with(&mut self, starting_with: &str) -> u64 {
        let mut wires = self
            .connections
            .keys()
            .filter(|wire| wire.starts_with(starting_with))
            .cloned()
            .collect::<Vec<_>>();
        wires.sort();
        let mut val = 0;
        for z_wire in wires.iter().rev() {
            val <<= 1;
            val |= self.calc_wire_value(z_wire) as u64;
        }
        val
    }

    fn calc_wire_value(&mut self, wire: &Wire) -> bool {
        let mut vals = self.wire_values.clone();
        let res = self.calc_wire_val_internal(&mut vals, wire);
        self.wire_values = vals;
        res
    }

    fn calc_wire_val_internal(&self, wire_values: &mut HashMap<Wire, bool>, wire: &Wire) -> bool {
        if let Some(&val) = wire_values.get(wire) {
            return val;
        }

        match self.connections.get(wire) {
            None => {
                unreachable!("Cannot calculate wire {wire} value. It doesn't have required inputs.")
            }
            Some(connection) => {
                let lhs_val = self.calc_wire_val_internal(wire_values, &connection.from.0);
                let rhs_val = self.calc_wire_val_internal(wire_values, &connection.from.1);
                let res = connection.gate.calc(lhs_val, rhs_val);
                wire_values.insert(wire.clone(), res);
                res
            }
        }
    }
}

impl Gate {
    fn from_str(gate_str: &str) -> Gate {
        match gate_str {
            "AND" => Gate::AND,
            "OR" => Gate::OR,
            "XOR" => Gate::XOR,
            _ => unreachable!("Invalid gate type"),
        }
    }

    fn calc(self, lhs: bool, rhs: bool) -> bool {
        match self {
            Gate::AND => lhs && rhs,
            Gate::OR => lhs || rhs,
            Gate::XOR => lhs ^ rhs,
        }
    }
}

fn parse_file(file_path: &str) -> io::Result<State> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut wire_values = HashMap::new();
    let mut connections = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        if line.contains(":") {
            // Parse wire value: "<wire>: <bool>"
            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            let wire = parts[0].to_string();
            let value = parts[1] == "1";
            wire_values.insert(wire, value);
        } else if line.contains("->") {
            // Parse connection: "<from_lhs> <gate> <from_rhs> -> <result>"
            let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
            let lhs = parts[0];
            let result_wire = parts[1].to_string();

            let tokens: Vec<&str> = lhs.split_whitespace().collect();
            let from_wire1 = tokens[0].to_string();
            let gate = Gate::from_str(tokens[1]);
            let from_wire2 = tokens[2].to_string();

            assert!(connections
                .insert(
                    result_wire,
                    Connection {
                        from: (from_wire1, from_wire2),
                        gate,
                    },
                )
                .is_none());
        }
    }

    Ok(State {
        wire_values,
        connections,
    })
}
