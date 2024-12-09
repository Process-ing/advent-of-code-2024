use utils::read_all;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Block {
    start: usize,
    id: i32,
    size: u8
}

impl Block {
    fn new(id: i32, start: usize, size: u8) -> Self {
        Self {
            id,
            start,
            size,
        }
    }
}

fn to_disk_map(line: &[u8]) -> Vec<u8> {
    return line.iter().map(|&byte| byte - b'0').collect()
}

fn compact<'a>(disk_map: &'a [u8]) -> impl Iterator<Item = Block> + 'a {
    let mut disk_map: Vec<u8> = disk_map.iter().map(|c| *c).collect();

    let mut i = 0;
    while disk_map[i] == 0 {
        i += 1;
    }

    let mut j = disk_map.len() - 1;
    while i <= j && disk_map[j] == 0 {
        j -= 2;
    }

    let mut pos = 0;
    return std::iter::from_fn(move || {
        if i <= j {
            if i % 2 == 0 {
                let next = Some(Block::new(i as i32 / 2, pos, disk_map[i]));
                pos += disk_map[i] as usize;

                i += 1;
                while i <= j && disk_map[i] == 0 {
                    i += 1;
                }

                return next;

            } else if disk_map[j] < disk_map[i] {
                let next = Some(Block::new(j as i32 / 2, pos, disk_map[j]));
                pos += disk_map[j] as usize;

                disk_map[i] -= disk_map[j];
                j -= 2;
                while i <= j && disk_map[j] == 0 {
                    j -= 2;
                }

                return next;

            } else {
                let next = Some(Block::new(j as i32 / 2, pos, disk_map[i]));
                pos += disk_map[i] as usize;

                disk_map[j] -= disk_map[i];
                i += 1;
                while i <= j && disk_map[i] == 0 {
                    i += 1;
                }

                while i <= j && disk_map[j] == 0 {
                    j -= 2;
                }

                return next;
            }
        } else {
            return None;
        }
    });
}

fn to_blocks<'a>(disk_map: &'a [u8]) -> impl Iterator<Item = Block> + 'a {
    let mut pos = 0;

    return disk_map.iter()
        .enumerate()
        .map(move |(i, &size)| {
            let next = Block::new(if i % 2 == 0 { i as i32 / 2 } else { -1 }, pos, size);
            pos += size as usize;
            return next;
        });
}

fn compact_whole<'a>(disk_map: &'a [u8]) -> Vec<Block> {
    let mut block_disk: Vec<Block> = to_blocks(disk_map).collect();

    let mut j = block_disk.len() - 1;

    loop {
        for i in 0..j {
            if block_disk[i].id == -1 && block_disk[i].size >= block_disk[j].size {
                let new_block1 = Block::new(block_disk[j].id, block_disk[i].start, block_disk[j].size);

                if block_disk[i].size > block_disk[j].size {
                    let new_block2 = Block::new(-1, block_disk[i].start + block_disk[j].size as usize, block_disk[i].size - block_disk[j].size);
                    block_disk.splice(i..i + 1, [new_block1, new_block2]);
                    j += 1;
                } else {
                    block_disk[i] = new_block1;
                }
                block_disk[j].id = -1;
                break;
            }
        }

        if j == 0 {
            break;
        }
        j -= 1;
        while block_disk[j].id == -1 {
            j -= 1;
        }
    }

    return block_disk;
}

fn get_checksum<'a, I: Iterator<Item = Block>>(compact_disk: I) -> usize {
    return compact_disk.fold(0, |checksum, Block { id, start, size}| {
        if id > 0 { checksum + id as usize * (2 * start + size as usize - 1) * size as usize / 2 } else { checksum }
    });
}

fn main() {
    let disk_map = to_disk_map(&read_all().into_bytes());
    let compact_disk = compact(&disk_map);
    let compact_whole_disk = compact_whole(&disk_map);

    let result1 = get_checksum(compact_disk);
    let result2 = get_checksum(compact_whole_disk.into_iter());

    println!("Part 1 result: {result1}");
    println!("Part 2 result: {result2}");
}
