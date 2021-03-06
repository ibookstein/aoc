use aoc::aoc_input::get_input;
use aoc::intcode::*;

fn run_program(mut tape: Tape, noun: isize, verb: isize) -> isize {
    tape[1] = noun;
    tape[2] = verb;

    let mut machine = IntcodeMachine::new(tape);
    match machine.run_to_completion() {
        Ok(_) => (),
        Err(err) => panic!("IntcodeMachine error: {:?}", err),
    }

    machine.read_addr(0).unwrap()
}

fn find_preimage(original: Tape, preimage: isize) -> Option<(isize, isize)> {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(original.clone(), noun, verb) == preimage {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn main() {
    let input = get_input(2019, 2);
    let original_tape = parse_intcode_program(&input);

    println!(
        "run_program(12, 2) = {}",
        run_program(original_tape.clone(), 12, 2)
    );

    let res = find_preimage(original_tape, 19690720);
    let (noun, verb) = res.expect("Failed finding preimage");
    println!("100 * noun + verb = {}", 100 * noun + verb);
}
