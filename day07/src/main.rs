use std::env;
use std::fs;

#[derive(Clone,PartialEq,Copy,Debug)]
enum Operator {
    Add,
    Mul
}
const NUM_OPS: u32 = 2;

#[derive(Clone,PartialEq,Copy,Debug)]
enum OperatorP2 {
    Add,
    Mul,
    Concat
}
const NUM_OPS_P2: u32 = 3;

#[derive(Debug)]
struct Equation {
    result_val: u64,
    values: Vec<u64>,
}
impl Equation {
    fn is_solvable(&self) -> bool {
        if self.values.len() == 1 {
            println!("is_solvable values size == 1");
            return false;
        }
        
        let eq_num_ops = self.values.len()-1;
        let num_combos: u32 = NUM_OPS.pow(eq_num_ops.try_into().unwrap());

        let mut all_combos: Vec<Vec<Operator>> = vec![vec![]; num_combos.try_into().unwrap()];
        for i in 0..all_combos.len() {
            all_combos[i].resize(eq_num_ops, Operator::Add);
        }
        let mut num_till_switch: u32 = 1;
        for i in (0..eq_num_ops).rev() {
            let mut cur_op: Operator = Operator::Add;
            for j in 0..num_combos {
                all_combos[j as usize][i] = cur_op;
                if (j+1)%num_till_switch == 0 {
                    match cur_op {
                        Operator::Add => cur_op = Operator::Mul,
                        Operator::Mul => cur_op = Operator::Add
                    }
                }
            }
            num_till_switch *= NUM_OPS;
        }

        for ops in all_combos {
            if self.evaluate(&ops) == self.result_val {
                return true;
            }
        }

        return false;
    }
    fn evaluate(&self, ops: &Vec<Operator>) -> u64 {
        if ops.len() != self.values.len()-1 {
            println!("Unexpected ops len");
            return 0;
        }
        let mut cur_result: u64 = self.values[0];
        for i in 1..self.values.len() {
            match ops[i-1] {
                Operator::Add => cur_result = cur_result + self.values[i],
                Operator::Mul => cur_result = cur_result * self.values[i],
            }
        }

        cur_result
    }

    fn is_solvable_p2(&self) -> bool {
        if self.values.len() == 1 {
            println!("is_solvable values size == 1");
            return false;
        }
        
        let eq_num_ops = self.values.len()-1;
        let num_combos: u32 = NUM_OPS_P2.pow(eq_num_ops.try_into().unwrap());

        let mut all_combos: Vec<Vec<OperatorP2>> = vec![vec![]; num_combos.try_into().unwrap()];
        for i in 0..all_combos.len() {
            all_combos[i].resize(eq_num_ops, OperatorP2::Add);
        }
        let mut num_till_switch: u32 = 1;
        for i in (0..eq_num_ops).rev() {
            let mut cur_op: OperatorP2 = OperatorP2::Add;
            for j in 0..num_combos {
                all_combos[j as usize][i] = cur_op;
                if (j+1)%num_till_switch == 0 {
                    match cur_op {
                        OperatorP2::Add => cur_op = OperatorP2::Mul,
                        OperatorP2::Mul => cur_op = OperatorP2::Concat,
                        OperatorP2::Concat => cur_op = OperatorP2::Add
                    }
                }
            }
            num_till_switch *= NUM_OPS_P2;
        }

        for ops in all_combos {
            if self.evaluate_p2(&ops) == self.result_val {
                return true;
            }
        }

        return false;
    }
    fn evaluate_p2(&self, ops: &Vec<OperatorP2>) -> u64 {
        if ops.len() != self.values.len()-1 {
            println!("Unexpected ops len");
            return 0;
        }
        let mut cur_result: u64 = self.values[0];
        for i in 1..self.values.len() {
            match ops[i-1] {
                OperatorP2::Add => cur_result = cur_result + self.values[i],
                OperatorP2::Mul => cur_result = cur_result * self.values[i],
                OperatorP2::Concat => {
                    let str1: String = cur_result.to_string();
                    let str2: String = self.values[i].to_string();
                    let str3: String = str1 + &str2;
                    let new_val: u64 = str3.parse().unwrap();
                    cur_result = new_val;
                }
            }
        }

        cur_result
    }
}

fn build_equation(in_str: &String) -> Equation {
    let mut tokens = in_str.split(':');
    let result_val = sscanf::sscanf!(tokens.next().unwrap(), "{}", u64).unwrap();

    let values: Vec<u64> = tokens.next().unwrap().split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    Equation {
        result_val,
        values
    }
}

fn read_input_eqs(file_name: &String) -> Vec<Equation> {
    return fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| build_equation(&s))
        .collect();
}

fn part1(eqs: &Vec<Equation>) {
    let mut vals_sum: u64 = 0;
    for eq in eqs {
        if eq.is_solvable() {
            vals_sum += eq.result_val;
        }
    }
    println!("Part1 sum: {}", vals_sum);
}

fn part2(eqs: &Vec<Equation>) {
    let mut vals_sum: u64 = 0;
    for eq in eqs {
        if eq.is_solvable_p2() {
            vals_sum += eq.result_val;
        }
    }
    println!("Part2 sum: {}", vals_sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let eqs = read_input_eqs(&args[1]);
    
    part1(&eqs);
    part2(&eqs);
}

