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

    assert_eq!(0, result)
}

#[derive(Debug, PartialEq)]
enum DiskMapTypes {
    File,
    FreeSpace,
}

type DiskMap = Vec<isize>;
type ParsedDiskMap = Vec<String>;

pub fn solve_part1(data: &String) -> isize {
    let mut parsed_disk_map = parse_disk_map(read_data(data));
    let compacted_disk_map = compact_disk_map(&mut parsed_disk_map);

    generate_checksum(compacted_disk_map)
}

pub fn solve_part2(data: &String) -> i32 {
    0
}

fn read_data(input: &String) -> DiskMap {
    input
        .trim()
        .split("")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
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
