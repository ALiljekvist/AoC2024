use std::{fs::read_to_string, iter::zip};

fn make_compact(blocks: &Vec<(i64, usize, usize)>) -> Vec<(i64, usize, usize)> {
    let mut new_blocks: Vec<(i64, usize, usize)> = vec![blocks[0].clone()];
    let mut front = 1;
    let mut back = blocks.len() - 1;
    let mut block = blocks[back].clone();
    while front < back {
        let mut slot = get_slot(front, blocks);
        while slot.1 > 0 {
            if block.2 <= slot.1 {
                // entire block fits, add all to the new memory layout
                new_blocks.push((block.0, slot.0, block.2));
                // update slot values and get next block
                slot.0 += block.2;
                slot.1 -= block.2;
                back -= 1;
                block = blocks[back].clone();
                continue;
            }
            // fit what you can into the slot and update the remaining block before 
            new_blocks.push((block.0, slot.0, slot.1));
            block.2 -= slot.1;
            slot.0 += slot.1;
            slot.1 = 0;
        }
        new_blocks.push(blocks[front]);
        front += 1;
    }
    if block.2 > 0 && block.1 == blocks[front].1 {
        let last_block = new_blocks[new_blocks.len()-1];
        new_blocks.push((block.0, last_block.1 + last_block.2, block.2))
    }
    new_blocks
}

fn get_slot(ind: usize, blocks: &Vec<(i64, usize, usize)>) -> (usize, usize) {
    let slot_start = blocks[ind-1].1 + blocks[ind-1].2;
    return (slot_start, blocks[ind].1 - slot_start);
}

fn make_compact_blockwise(blocks: &mut Vec<(i64, usize, usize)>) {
    let mut slots: Vec<(usize, usize)> = zip(&blocks[0..], &blocks[1..]).into_iter()
        .map(|(b1, b2)| (b1.1+b1.2, b2.1-b1.1-b1.2))
        .filter(|x| x.1 != 0)
        .collect();
    let mut i = blocks.len()-1;
    while i > 0 {
        let mut moved = false;
        for slot in slots.iter_mut() {
            if blocks[i].1 < slot.0 {
                break
            }
            if blocks[i].2 <= slot.1 {
                // take the block
                let mut block = blocks.remove(i);
                block.1 = slot.0;
                // Update the refilled slot
                slot.0 += block.2;
                slot.1 -= block.2;
                let mut new_ind = 0;
                while new_ind < blocks.len() && blocks[new_ind].1 < block.1 {
                    new_ind += 1
                }
                blocks.insert(new_ind, block);
                moved = true;
                break
            }
        }
        if !moved {
            i -= 1;
        }
    }
}

fn check_sum(blocks: &Vec<(i64, usize, usize)>) -> i64 {
    return blocks.iter()
        .map(|x| (x.1..x.1+x.2).into_iter().map(|d| d as i64).sum::<i64>() * x.0)
        .sum::<i64>()
}

fn main() {
    let nums: Vec<i64> = read_to_string("input.txt").unwrap()
        .trim()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let mut blocks: Vec<(i64, usize, usize)> = Vec::new();
    let mut mem_i = 0;
    for (i, num) in nums.iter().enumerate() {
        if i % 2 == 0 {
            blocks.push(((i as i64)/2, mem_i, *num as usize))
        }
        mem_i += *num as usize;
    }
    let p1_blocks = make_compact(&blocks);
    println!("part1: {}", check_sum(&p1_blocks));
    make_compact_blockwise(&mut blocks);
    println!("part2: {}", check_sum(&blocks));
}
