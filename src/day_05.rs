use std::time::Instant;

use crate::Timing;

fn parse_crane(input: String) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    input.lines().take(8).for_each(|line| {
        for (idx, char) in line.char_indices() {
            if (idx - 1) % 4 == 0 {
                if char.is_whitespace() {
                    continue;
                }

                let stack_id = ((idx + 3) / 4) - 1;
                stacks.get_mut(stack_id).unwrap().push(char);
            }
        }
    });

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    stacks
}

fn parse_commands(input: String) -> Vec<(usize, usize, usize)> {
    let mut commands = Vec::new();

    for line in input.lines() {
        let mut splitted = line.split(" ");
        let amount: usize = splitted.nth(1).unwrap().parse().unwrap();
        let source: usize = splitted.nth(1).unwrap().parse().unwrap();
        let dest: usize = splitted.nth(1).unwrap().parse().unwrap();
        commands.push((amount, source, dest));
    }

    commands
}

fn get_top(crane: Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for stack in crane {
        result.push(*stack.last().unwrap());
    }

    result
}

fn execute_command(crane: &mut Vec<Vec<char>>, command: (usize, usize, usize)) {
    let amount = command.0;
    let source = command.1 - 1;
    let dest = command.2 - 1;

    for _ in 0..amount {
        let item = crane.get_mut(source).unwrap().pop().unwrap();
        crane.get_mut(dest).unwrap().push(item);
    }
}

fn execute_command_part2(crane: &mut Vec<Vec<char>>, command: (usize, usize, usize)) {
    let amount = command.0;
    let source = command.1 - 1;
    let dest = command.2 - 1;
    let mut foo = Vec::new();

    for _ in 0..amount {
        let item = crane.get_mut(source).unwrap().pop().unwrap();
        foo.insert(0, item);
    }
    crane.get_mut(dest).unwrap().extend(foo);
}

pub fn run() -> std::io::Result<Timing> {
    let buffer = include_str!("../input/05/input.txt");

    let start = Instant::now();
    let (crane, commands) = buffer.split_once("\n\n").unwrap();

    let mut crane = parse_crane(crane.to_string());
    let commands = parse_commands(commands.to_string());

    for command in commands {
        execute_command(&mut crane, command);
    }

    let top = get_top(crane);
    let elapsed_part1 = start.elapsed();
    println!("Top: {}", top);

    let start = Instant::now();
    let (crane, commands) = buffer.split_once("\n\n").unwrap();

    let mut crane = parse_crane(crane.to_string());
    let commands = parse_commands(commands.to_string());

    for command in commands {
        execute_command_part2(&mut crane, command);
    }

    let top = get_top(crane);
    let elapsed_part2 = start.elapsed();
    println!("Top part2: {}", top);

    Ok(Timing(elapsed_part1, elapsed_part2))
}
