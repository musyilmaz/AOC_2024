#[test]
fn test_part1() {
    let test_string = String::from("2333133121414131402");
    let result = solve_part1(&test_string);

    assert_eq!(1928, result)
}

#[test]
fn test_part2() {
    let test_string = String::from("2333133121414131402");
    let result = solve_part2(&test_string);

    assert_eq!(2858, result)
}

#[derive(Debug, PartialEq)]
enum DiskMapTypes {
    File,
    FreeSpace,
}

type Space = i32;
type FileValue = Option<i32>;
type IsMoved = bool;

type DiskMapItems = (Space, FileValue, IsMoved);

type DiskMap = Vec<isize>;
type ParsedDiskMap = Vec<String>;
type ParsedDiskMapPart2 = Vec<DiskMapItems>;

pub fn solve_part1(data: &String) -> isize {
    let mut parsed_disk_map = parse_disk_map(read_data(data));
    let compacted_disk_map = compact_disk_map(&mut parsed_disk_map);

    generate_checksum(compacted_disk_map)
}

pub fn solve_part2(data: &String) -> isize {
    let disk_map = read_data(data);
    let mut parsed_disk_map = parse_disk_map_part_2(disk_map);
    let compacted_disk_map = generate_compact_disk_map(&mut parsed_disk_map);
    let file_blocks = generate_file_blocks(compacted_disk_map);
    generate_checksum_part_2(file_blocks)
}

fn read_data(input: &String) -> DiskMap {
    input
        .trim()
        .split("")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

fn parse_disk_map_part_2(disk_map: DiskMap) -> ParsedDiskMapPart2 {
    let mut indicator: DiskMapTypes = DiskMapTypes::File;
    let mut file_id: i32 = 0;
    let mut parsed_disk_map: ParsedDiskMapPart2 = vec![];

    for item in disk_map {
        match indicator {
            DiskMapTypes::File => {
                let data = (item as i32, Some(file_id), false);
                parsed_disk_map.push(data);
                indicator = DiskMapTypes::FreeSpace;
                file_id += 1;
            }
            DiskMapTypes::FreeSpace => {
                let data = (item as i32, None, false);
                parsed_disk_map.push(data);
                indicator = DiskMapTypes::File;
            }
        }
    }

    parsed_disk_map
}

fn generate_compact_disk_map(parsed_disk_map: &mut ParsedDiskMapPart2) -> ParsedDiskMapPart2 {
    // loop over parsed_disk_map backwards
    for i in (0..parsed_disk_map.len()).rev() {
        if parsed_disk_map[i].1.is_none() || parsed_disk_map[i].2 {
            continue;
        }

        // Start from beginning and check free space
        for j in 0..i {
            // Continue looking if there is a file
            // or current scoped space is smaller than the file from back
            if parsed_disk_map[j].1.is_some() || parsed_disk_map[j].0 < parsed_disk_map[i].0 {
                continue;
            }

            let space_diff = parsed_disk_map[j].0 - parsed_disk_map[i].0;

            parsed_disk_map[j] = (parsed_disk_map[i].0, parsed_disk_map[i].1, true);
            parsed_disk_map[i].1 = None;

            if space_diff > 0 {
                parsed_disk_map.insert(j + 1, (space_diff, None, true));
            }

            break;
        }
    }

    parsed_disk_map.to_vec()
}

fn generate_file_blocks(compacted_disk_map: ParsedDiskMapPart2) -> Vec<i32> {
    let mut file_blocks: Vec<i32> = vec![];

    for disk_item in compacted_disk_map {
        if disk_item.1.is_some() {
            let mut data: Vec<i32> = vec![disk_item.1.unwrap(); disk_item.0 as usize];
            file_blocks.append(&mut data);
        } else {
            let mut data: Vec<i32> = vec![0 as i32; disk_item.0 as usize];
            file_blocks.append(&mut data);
        }
    }

    file_blocks
}

fn parse_disk_map(disk_map: DiskMap) -> ParsedDiskMap {
    let mut indicator: DiskMapTypes = DiskMapTypes::File;
    let mut file_id: isize = 0;
    let mut parsed_disk_map: ParsedDiskMap = vec![];

    for item in disk_map {
        match indicator {
            DiskMapTypes::File => {
                let mut file_data = vec![file_id.to_string(); item as usize];
                parsed_disk_map.append(&mut file_data);
                indicator = DiskMapTypes::FreeSpace
            }
            DiskMapTypes::FreeSpace => {
                let mut free_space_data = vec![".".to_string(); item as usize];
                parsed_disk_map.append(&mut free_space_data);
                indicator = DiskMapTypes::File
            }
        }

        if indicator == DiskMapTypes::File {
            file_id += 1
        }
    }

    parsed_disk_map
}

fn compact_disk_map(parsed_disk_map: &mut ParsedDiskMap) -> DiskMap {
    let mut compacted_disk_map: DiskMap = vec![];
    let mut left: usize = 0;
    let mut right: usize = parsed_disk_map.len() - 1;

    while left <= right {
        if parsed_disk_map[left] != "." {
            compacted_disk_map.push(parsed_disk_map[left].parse::<isize>().unwrap());
            left += 1
        } else if parsed_disk_map[right] == "." {
            right -= 1
        } else {
            compacted_disk_map.push(parsed_disk_map[right].parse::<isize>().unwrap());
            right -= 1;
            left += 1;
        }
    }

    compacted_disk_map
}

fn generate_checksum(disk_map: DiskMap) -> isize {
    let mut checksum = 0;
    for (i, item) in disk_map.iter().enumerate() {
        checksum += item * i as isize
    }

    checksum
}

fn generate_checksum_part_2(file_blocks: Vec<i32>) -> isize {
    let mut checksum: isize = 0;

    for (i, item) in file_blocks.iter().enumerate() {
        checksum += *item as isize * i as isize
    }

    checksum
}
