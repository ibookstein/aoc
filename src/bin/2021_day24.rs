use aoc::{aoc_input::get_input, parse::iter_consume_exact};
use enum_as_inner::EnumAsInner;
use itertools::Itertools;
use num_integer::Integer;
use petgraph::{
    dot::Dot,
    graph::NodeIndex,
    stable_graph::StableGraph,
    visit::{Dfs, EdgeRef, IntoNodeReferences, Reversed, Topo},
    Direction,
};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
    process::Command,
};
use strum_macros::{Display, IntoStaticStr};

#[derive(Debug, Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl From<Register> for usize {
    fn from(r: Register) -> Self {
        match r {
            Register::W => 0,
            Register::X => 1,
            Register::Y => 2,
            Register::Z => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumAsInner)]
enum Operand {
    Reg(Register),
    Imm(i64),
}

#[derive(Debug, Clone, Copy, IntoStaticStr, Display)]
enum Opcode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl Opcode {
    fn is_commutative(&self) -> bool {
        match self {
            Opcode::Inp => false,
            Opcode::Add => true,
            Opcode::Mul => true,
            Opcode::Div => false,
            Opcode::Mod => false,
            Opcode::Eql => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opc: Opcode,
    ops: Vec<Operand>,
}

#[derive(Debug, Clone, Copy, EnumAsInner)]
enum NodeType {
    Compute(Opcode),
    Immediate(i64),
    Source(usize),
    Sink,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Compute(opc) => write!(f, "{}", opc),
            NodeType::Immediate(imm) => write!(f, "{}", imm),
            NodeType::Source(i) => write!(f, "Input{}", i),
            NodeType::Sink => write!(f, "Sink"),
        }
    }
}

#[derive(Debug, Clone)]
struct DataflowGraph {
    g: StableGraph<NodeType, u8>,
}

impl DataflowGraph {
    fn new(g: StableGraph<NodeType, u8>) -> Self {
        Self { g }
    }

    fn rauw(&mut self, from: NodeIndex, to: NodeIndex) {
        let edges: Vec<_> = self
            .g
            .edges_directed(from, Direction::Outgoing)
            .map(|e| (*e.weight(), e.id(), e.target()))
            .collect();
        for (w, id, target) in edges {
            self.g.remove_edge(id);
            self.g.add_edge(to, target, w);
        }
    }

    fn constant_fold(&mut self, ni: NodeIndex, lhs_imm: i64, rhs_imm: i64) {
        let opc = self.g[ni].into_compute().unwrap();
        let res_imm = match opc {
            Opcode::Add => lhs_imm + rhs_imm,
            Opcode::Mul => lhs_imm * rhs_imm,
            Opcode::Div => lhs_imm / rhs_imm,
            Opcode::Mod => lhs_imm % rhs_imm,
            Opcode::Eql => (lhs_imm == rhs_imm) as i64,
            _ => panic!("Invalid compute"),
        };

        let res_node = self.g.add_node(NodeType::Immediate(res_imm));
        self.rauw(ni, res_node);
    }

    fn combine_compute(&mut self, ni: NodeIndex) -> bool {
        let iter = self
            .g
            .edges_directed(ni, Direction::Incoming)
            .sorted_by_key(|e| e.weight())
            .map(|e| (e.source(), e.id()));
        let mut ops: [_; 2] = iter_consume_exact(iter).unwrap();

        let mut lhs_imm = self.g[ops[0].0].as_immediate().copied();
        let mut rhs_imm = self.g[ops[1].0].as_immediate().copied();

        if let (Some(lhs_imm), Some(rhs_imm)) = (lhs_imm, rhs_imm) {
            self.constant_fold(ni, lhs_imm, rhs_imm);
            return true;
        }

        let opc = self.g[ni].into_compute().unwrap();
        // Right-justify immediates when commutative
        if let (true, Some(_), None) = (opc.is_commutative(), lhs_imm, rhs_imm) {
            ops.swap(0, 1);
            self.g[ops[0].1] = 0;
            self.g[ops[1].1] = 1;
            std::mem::swap(&mut lhs_imm, &mut rhs_imm);
        }

        match opc {
            Opcode::Add => match rhs_imm {
                Some(0) => {
                    self.rauw(ni, ops[0].0);
                    true
                }
                _ => false,
            },
            Opcode::Mul => match rhs_imm {
                Some(0) => {
                    self.rauw(ni, ops[1].0);
                    true
                }
                Some(1) => {
                    self.rauw(ni, ops[0].0);
                    true
                }
                _ => false,
            },
            Opcode::Div => match (lhs_imm, rhs_imm) {
                (Some(0), _) => {
                    self.rauw(ni, ops[0].0);
                    true
                }
                (_, Some(1)) => {
                    self.rauw(ni, ops[0].0);
                    true
                }
                _ => false,
            },
            Opcode::Mod => match (lhs_imm, rhs_imm) {
                (Some(0), _) => {
                    self.rauw(ni, ops[0].0);
                    true
                }
                (_, Some(1)) => {
                    let imm0 = self.g.add_node(NodeType::Immediate(0));
                    self.rauw(ni, imm0);
                    true
                }
                _ => false,
            },
            Opcode::Eql => false,
            _ => panic!("Invalid compute"),
        }
    }

    fn combine(&mut self) -> bool {
        let mut topo = Topo::new(&self.g);
        let mut any_change = false;
        while let Some(ni) = topo.next(&self.g) {
            match self.g[ni] {
                NodeType::Compute(_) => {
                    any_change |= self.combine_compute(ni);
                }
                _ => {}
            }
        }
        any_change
    }

    fn dce(&mut self) -> bool {
        let mut sinks = Vec::new();
        let mut all = HashSet::new();
        for ni in self.g.node_indices() {
            match self.g[ni] {
                NodeType::Sink => sinks.push(ni),
                _ => {
                    all.insert(ni);
                }
            }
        }

        let reversed = Reversed(&self.g);
        let mut dfs = Dfs::empty(&reversed);
        dfs.stack = sinks;

        while let Some(ni) = dfs.next(&reversed) {
            all.remove(&ni);
        }

        let any_change = !all.is_empty();
        for ni in all {
            self.g.remove_node(ni);
        }
        any_change
    }

    fn optimize(&mut self) {
        loop {
            let mut any_change = false;

            any_change |= self.combine();
            any_change |= self.dce();

            if !any_change {
                break;
            }
        }
    }

    fn partially_evaluate(&mut self, values: HashMap<usize, i64>) {
        let inputs: Vec<_> = self
            .g
            .node_references()
            .filter_map(|nr| nr.1.as_source().map(|src| (*src, nr.0)))
            .collect();

        for (i, ni) in inputs {
            if let Some(imm) = values.get(&i) {
                let immnode = self.g.add_node(NodeType::Immediate(*imm));
                self.rauw(ni, immnode);
            }
        }

        self.optimize();
    }
}

fn parse_opcode(s: &str) -> (Opcode, usize) {
    match s {
        "inp" => (Opcode::Inp, 1),
        "add" => (Opcode::Add, 2),
        "mul" => (Opcode::Mul, 2),
        "div" => (Opcode::Div, 2),
        "mod" => (Opcode::Mod, 2),
        "eql" => (Opcode::Eql, 2),
        _ => panic!("Invalid opcode"),
    }
}

fn parse_operand(s: &str) -> Operand {
    match s {
        "w" => Operand::Reg(Register::W),
        "x" => Operand::Reg(Register::X),
        "y" => Operand::Reg(Register::Y),
        "z" => Operand::Reg(Register::Z),
        s => Operand::Imm(s.parse().unwrap()),
    }
}

fn parse_operands(s: &str) -> Vec<Operand> {
    s.split(' ').map(parse_operand).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let [opc, ops] = iter_consume_exact(line.splitn(2, ' ')).unwrap();
    let (opc, expected_operands) = parse_opcode(opc);
    let ops = parse_operands(ops);

    assert_eq!(expected_operands, ops.len());
    assert!(if let Operand::Reg(_) = ops[0] {
        true
    } else {
        false
    });

    Instruction { opc, ops }
}

fn parse_dataflow_graph(input: &str) -> DataflowGraph {
    let mut graph = StableGraph::new();

    let zero = graph.add_node(NodeType::Immediate(0));
    let mut inputs = 0usize;
    let mut regmap = [zero; 4];
    for insn in input.lines().map(parse_instruction) {
        match insn.opc {
            Opcode::Inp => {
                let source = graph.add_node(NodeType::Source(inputs));
                inputs += 1;
                let regidx: usize = insn.ops[0].into_reg().unwrap().into();
                regmap[regidx] = source;
            }
            Opcode::Add | Opcode::Mul | Opcode::Div | Opcode::Mod | Opcode::Eql => {
                let lhsidx: usize = insn.ops[0].into_reg().unwrap().into();
                let lhs = regmap[lhsidx];
                let rhs = match insn.ops[1] {
                    Operand::Reg(r) => regmap[usize::from(r)],
                    Operand::Imm(imm) => graph.add_node(NodeType::Immediate(imm)),
                };

                let node = graph.add_node(NodeType::Compute(insn.opc));
                graph.add_edge(lhs, node, 0);
                graph.add_edge(rhs, node, 1);
                regmap[lhsidx] = node;
            }
        }
    }

    let output_node = graph.add_node(NodeType::Sink);
    let res_node = regmap[usize::from(Register::Z)];
    graph.add_edge(res_node, output_node, 0);

    DataflowGraph::new(graph)
}

fn main() {
    let input = get_input(2021, 24);

    let mut dfg = parse_dataflow_graph(&input);
    dfg.optimize();

    let dot = format!("{}", Dot::with_config(&dfg.g, &[]));
    let mut f = File::create("day24.dot").unwrap();
    f.write_all(&dot.as_bytes()).unwrap();

    if let Some(x) = std::env::args().nth(1) {
        let mut values = HashMap::new();
        for (i, c) in x.chars().enumerate() {
            values.insert(i, c as i64 - '0' as i64);
        }

        dfg.partially_evaluate(values);
        let dot = format!("{}", Dot::with_config(&dfg.g, &[]));
        let mut f = File::create("day24_partial.dot").unwrap();
        f.write_all(&dot.as_bytes()).unwrap();

        Command::new(r"C:\Program Files (x86)\Graphviz2.38\bin\dot.exe")
            .args(["-Tsvg", "day24_partial.dot", "-o", "day24_partial.svg"])
            .spawn()
            .expect("Failed converting to SVG");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
}
