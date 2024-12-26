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
    let broken_outputs = input.get_broken_outputs();
    broken_outputs.join(",")
}

type Wire = String;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

    fn get_broken_outputs(&self) -> Vec<String> {
        // z00, z01, z02 are different and it was quicker to check them manually instead of writing code cases
        let mut vec = vec![];
        for bit in 3..64 {
            if let Some(broken_output) = self.get_broken_full_adder_output(bit) {
                vec.push(broken_output);
            }
        }
        vec.sort();
        vec
    }

    fn get_broken_full_adder_output(&self, bit: usize) -> Option<String> {
        let zbit = str_bit('z', bit);
        if let None = &self.connections.get(&zbit) {
            return None;
        }

        if let None = &self.wire_values.get(&str_bit('x', bit)) {
            return None; // Hopefully the erorr isn't in the last bit :P
        }

        let xor_con = &self.connections[&zbit];
        let prev_xor_con = &self.connections[&zbit];
        if xor_con.gate != Gate::XOR {
            return Some(zbit);
        }
        assert!(prev_xor_con.gate == Gate::XOR);

        let prev_xor_wires = get_sorted_froms(prev_xor_con);

        match self.check_get_or_from_first_xor(xor_con, bit) {
            Err(broken_output) => return Some(broken_output),
            Ok((or_lhs, or_rhs)) => {
                let lhs_con = &self.connections[or_lhs];
                let rhs_con = &self.connections[or_rhs];
                // Input never has both of these swapped
                // if lhs_con.gate != Gate::AND && rhs_con.gate != Gate::AND {
                //     return Some(format!("{or_lhs}&{or_rhs}"));
                // }
                if lhs_con.gate != Gate::AND {
                    return Some(or_lhs.to_string());
                }

                if rhs_con.gate != Gate::AND {
                    return Some(or_rhs.to_string());
                }

                let lhs_or_wires = get_sorted_froms(lhs_con);
                let rhs_or_wires = get_sorted_froms(rhs_con);

                let prev_inputs = (&str_bit('x', bit - 1), &str_bit('y', bit - 1));
                if lhs_or_wires == prev_xor_wires {
                    if rhs_or_wires != prev_inputs {
                        return Some(or_rhs.to_string());
                    }
                }

                if rhs_or_wires == prev_xor_wires {
                    if lhs_or_wires != prev_inputs {
                        return Some(or_lhs.to_string());
                    }
                };

                return None;
            }
        }
    }

    fn check_get_or_from_first_xor(
        &self,
        first_xor_conn: &Connection,
        bit: usize,
    ) -> Result<(&String, &String), String> {
        let (xor_lhs, xor_rhs) = get_sorted_froms(first_xor_conn);
        let lhs_conn = &self.connections[xor_lhs];
        let rhs_conn = &self.connections[xor_rhs];
        let mut or_err = "".to_string();

        let mut xor_conn = None;
        let mut or_conn = None;
        if lhs_conn.gate == Gate::XOR {
            let froms = get_sorted_froms(lhs_conn);
            if froms.0 == &str_bit('x', bit) || froms.1 == &str_bit('y', bit) {
                xor_conn = Some(lhs_conn);
                or_conn = Some(rhs_conn);
                or_err = xor_rhs.to_string();
            }
        }

        if xor_conn.is_none() {
            if rhs_conn.gate == Gate::XOR {
                let froms = get_sorted_froms(rhs_conn);
                if froms.0 == &str_bit('x', bit) || froms.1 == &str_bit('y', bit) {
                    xor_conn = Some(rhs_conn);
                    or_conn = Some(lhs_conn);
                    or_err = xor_lhs.to_string();
                }
            }
        }

        if xor_conn.is_none() {
            if lhs_conn.gate == Gate::OR {
                return Err(xor_rhs.to_string());
            }
            // Input never has both of these swapped
            /*else if rhs_conn.gate == Gate::OR*/
            return Err(xor_lhs.to_string());

            // return Err(format!("{xor_lhs}&{xor_rhs}"));
        }

        let or_conn = or_conn.unwrap();
        if or_conn.gate == Gate::OR {
            return Ok(get_sorted_froms(or_conn));
        }

        Err(or_err)
    }
}

fn get_sorted_froms(conn: &Connection) -> (&String, &String) {
    let mut froms = vec![&conn.from.0, &conn.from.1];
    froms.sort();
    (froms[0], froms[1])
}

fn str_bit(ch: char, bit: usize) -> String {
    format!("{ch}{bit:02}")
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
