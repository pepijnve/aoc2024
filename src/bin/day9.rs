enum BlockSequence {
    Used { content: u32, length: usize },
    Free { length: usize },
}

fn print_block_seq(block_sequence: &[BlockSequence]) {
    for block in block_sequence {
        match *block {
            BlockSequence::Used { content, length } => {
                for _ in 0..length {
                    print!("{}", content);
                }
            }
            BlockSequence::Free { length } => {
                for _ in 0..length {
                    print!(".");
                }
            }
        }
    }
    println!("");
}

fn parse_input(input: &str) -> Vec<BlockSequence> {
    let mut blocks = Vec::new();
    let mut file_index = 0;
    let mut used = true;
    for c in input.chars() {
        let length = c.to_digit(10).unwrap() as usize;
        if used {
            blocks.push(BlockSequence::Used {
                content: file_index,
                length,
            });
            file_index += 1;
            used = false;
        } else {
            blocks.push(BlockSequence::Free { length });
            used = true;
        }
    }
    blocks
}

fn checksum(compacted: &Vec<BlockSequence>) -> usize {
    let mut checksum = 0;
    let mut file_index = 0;
    for block in compacted {
        match *block {
            BlockSequence::Used { content, length } => {
                for _ in 0..length {
                    checksum += file_index * content as usize;
                    file_index += 1;
                }
            }
            BlockSequence::Free { length } => {
                file_index += length;
            }
        }
    }

    checksum
}

fn calculate_answer1(input: &str) -> usize {
    let blocks = parse_input(input);

    let mut compacted = Vec::new();
    let mut end_index = blocks.len() - 1;
    let mut start_index = 0;
    while start_index <= end_index {
        match blocks[start_index] {
            BlockSequence::Used { content, length } => {
                compacted.push(BlockSequence::Used { content, length });
            }
            BlockSequence::Free { length } => {
                let mut free_length = length;
                while free_length > 0 && end_index > 0 && end_index > start_index {
                    match blocks[end_index] {
                        BlockSequence::Used { content, length } => {
                            if length <= free_length {
                                compacted.push(BlockSequence::Used { content, length });
                                free_length -= length;
                                end_index -= 1;
                            } else {
                                compacted.push(BlockSequence::Used {
                                    content,
                                    length: free_length,
                                });
                                blocks[end_index] = BlockSequence::Used {
                                    content,
                                    length: length - free_length,
                                };
                                free_length = 0;
                            }
                        }
                        _ => {
                            end_index -= 1;
                        }
                    }
                }
            }
        }
        start_index += 1;
    }

    checksum(&compacted)
}

fn calculate_answer2(_: &str) -> usize {
    0
}

fn main() {
    let input = aoc2024::io::read_input();

    println!("result 1: {}", calculate_answer1(&input));
    println!("result 2: {}", calculate_answer2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let sample = "2333133121414131402";

        assert_eq!(calculate_answer1(sample), 1928);
        assert_eq!(calculate_answer2(sample), 2858);
    }
}
