use std::fs::read_to_string;

fn make_compact(memory: &mut Vec<i64>) {
    let (mut i, mut j) = (0, memory.len()-1);
    while i < j {
        if memory[i] != -1 {
            i += 1;
            continue;
        }
        if memory[j] == -1 {
            j -= 1;
            continue;
        }
        (memory[i], memory[j]) = (memory[j], memory[i])
    }
}

fn make_compact_blockwise(
    memory: &mut Vec<i64>,
    block: &(i64, usize, usize)) {
    let mut slots: Vec<(usize, usize)> = Vec::new();
    let mut i = (0, 0);
    while i.0 < memory.len() {
        let next = find_next_empty_block(i, memory);
        slots.push(next);
        i.0 = next.0 + next.1
    }
    // find all slots below the current block index and move the block if you can
    for slot in slots.iter() {
        if block.1 < slot.0 {
            continue;
        }
        if block.2 <= slot.1 {
            // Swap memory
            for i in 0..block.2 {
                memory[slot.0+i] = block.0;
                memory[block.1+i] = -1;
            }
            return
        }
    }
}

fn find_next_empty_block(pos: (usize, usize), memory: &Vec<i64>) -> (usize, usize) {
    let mut start = pos.0;
    while start < memory.len() && memory[start] != -1 {
        start += 1
    }
    let mut end = start+1;
    while end < memory.len() && memory[end] == -1 {
        end += 1
    }
    (start, end-start)
}

fn check_sum(memory: &Vec<i64>) -> i64 {
    return memory.iter()
        .enumerate()
        .filter(|x| *x.1 >= 0)
        .map(|x| (x.0 as i64) * x.1)
        .sum::<i64>()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let nums: Vec<usize> = input.trim()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let mut memory = vec![-1; nums.iter().sum()];
    let mut blocks: Vec<(i64, usize, usize)> = Vec::new();
    let mut mem_i = 0;
    for (i, num) in nums.iter().enumerate() {
        if i % 2 == 0 {
            for j in mem_i..mem_i+num {
                memory[j] = (i as i64)/2;
            }
            blocks.push(((i as i64)/2, mem_i, *num))
        }
        mem_i += num;
    }
    let mut p1_memory = memory.clone();
    make_compact(&mut p1_memory);
    println!("part1: {}", check_sum(&p1_memory));
    for block in blocks.iter().rev() {
        make_compact_blockwise(&mut memory, block);
    }
    println!("part2: {}", check_sum(&memory));
}
