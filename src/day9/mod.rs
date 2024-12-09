use std::collections::HashMap;

use nom::InputIter;

use crate::common::string_utils::read_file_to_string;

pub fn run() {
    let input = read_file_to_string("input/day9.txt");
    let mut memory = Vec::new();
    let mut file_mode = true;
    let mut next_id = 0;
    for c in input.as_str().iter_elements() {
        let digit = c.to_digit(10).unwrap();
        if file_mode {
            //insert digits of id
            for _i in 0..digit {
                memory.push(Some(next_id));
            }
            next_id = next_id + 1;
        } else {
            for _i in 0..digit {
                memory.push(None);
            }
        }
        file_mode = !file_mode;
    }
    // println!("{:?}", memory);

    // compact it
    let mut lower = 0;
    let mut higher = memory.len() - 1;

    while lower < higher {
        if memory[lower].is_none() {
            while higher > lower {
                if memory[higher].is_some() {
                    let higher_elem = memory[higher].take().unwrap();
                    memory[lower].replace(higher_elem);
                    break;
                }
                higher = higher - 1;
            }
        }
        lower = lower + 1;
    }

    // println!("{:?}", memory);

    // checksum it
    let mut checksum = 0;
    for (i, value) in memory.iter().enumerate() {
        checksum = checksum + (i * value.unwrap_or(0));
    }

    println!("checksum: {}", checksum);
}

pub fn run2() {
    let input = read_file_to_string("input/day9.txt");
    let mut memory = Vec::new();
    let mut file_mode = true;
    let mut next_id = 0;
    let mut file_map = HashMap::new();
    for c in input.as_str().iter_elements() {
        let digit = c.to_digit(10).unwrap();
        if file_mode {
            //insert digits of id recording where we inserted it
            file_map.insert((next_id), (memory.len(), digit));
            for _i in 0..digit {
                memory.push(Some(next_id));
            }
            next_id = next_id + 1;
        } else {
            for _i in 0..digit {
                memory.push(None);
            }
        }
        file_mode = !file_mode;
    }
    println!("{:?}", file_map);
    println!("{:?}", memory);

    // compact it
    // starting with last used next_id (-1)
    next_id = next_id - 1;
    while next_id >= 0 {
        let (file_index, file_size) = file_map.get(&next_id).unwrap();

        println!(
            "id: {}, starting at index: {}, size: {}",
            next_id, file_index, file_size
        );

        // find a gap of that size then pull it in
        let mut search_index = 0;
        while search_index < *file_index {
            if memory[search_index].is_none() {
                if memory[search_index..search_index + *file_size as usize]
                    .into_iter()
                    .all(|m| m.is_none())
                {
                    // found a big enough slice
                    // println!(
                    //     "found a none slice at {}->{} for id {}",
                    //     search_index,
                    //     search_index + *file_size as usize,
                    //     next_id
                    // );

                    let mut move_counter: usize = 0;
                    while move_counter < *file_size as usize {
                        let higher_elem = memory[file_index + move_counter].take().unwrap();
                        memory[search_index + move_counter].replace(higher_elem);

                        move_counter = move_counter + 1;
                    }

                    // println!("{:?}", memory);

                    break;
                }
            }

            search_index = search_index + 1;
        }

        if (next_id == 0) {
            break;
        }
        next_id = next_id - 1;
    }

    println!("{:?}", memory);

    // checksum it
    let mut checksum = 0;
    for (i, value) in memory.iter().enumerate() {
        checksum = checksum + (i * value.unwrap_or(0));
    }

    println!("checksum: {}", checksum);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use nom::InputIter;

    const TEST_DATA: &str = r"2333133121414131402";

    #[test]
    pub fn test() {
        let mut memory = Vec::new();
        let mut file_mode = true;
        let mut next_id = 0;
        for c in TEST_DATA.iter_elements() {
            let digit = c.to_digit(10).unwrap();
            if file_mode {
                //insert digits of id
                for _i in 0..digit {
                    memory.push(Some(next_id));
                }
                next_id = next_id + 1;
            } else {
                for _i in 0..digit {
                    memory.push(None);
                }
            }
            file_mode = !file_mode;
        }
        println!("{:?}", memory);

        // compact it
        let mut lower = 0;
        let mut higher = memory.len() - 1;

        while lower < higher {
            if memory[lower].is_none() {
                while higher > lower {
                    if memory[higher].is_some() {
                        let higher_elem = memory[higher].take().unwrap();
                        memory[lower].replace(higher_elem);
                        break;
                    }
                    higher = higher - 1;
                }
            }
            lower = lower + 1;
        }

        println!("{:?}", memory);

        // checksum it
        let mut checksum = 0;
        for (i, value) in memory.iter().enumerate() {
            checksum = checksum + (i * value.unwrap_or(0));
        }

        println!("checksum: {}", checksum);
    }

    #[test]
    pub fn test2() {
        let mut memory = Vec::new();
        let mut file_mode = true;
        let mut next_id = 0;
        let mut file_map = HashMap::new();
        for c in TEST_DATA.iter_elements() {
            let digit = c.to_digit(10).unwrap();
            if file_mode {
                //insert digits of id recording where we inserted it
                file_map.insert((next_id), (memory.len(), digit));
                for _i in 0..digit {
                    memory.push(Some(next_id));
                }
                next_id = next_id + 1;
            } else {
                for _i in 0..digit {
                    memory.push(None);
                }
            }
            file_mode = !file_mode;
        }
        println!("{:?}", file_map);
        println!("{:?}", memory);

        // compact it
        // starting with last used next_id (-1)
        next_id = next_id - 1;
        while next_id >= 0 {
            let (file_index, file_size) = file_map.get(&next_id).unwrap();

            println!(
                "id: {}, starting at index: {}, size: {}",
                next_id, file_index, file_size
            );

            // find a gap of that size then pull it in
            let mut search_index = 0;
            while search_index < *file_index {
                if memory[search_index].is_none() {
                    if memory[search_index..search_index + *file_size as usize]
                        .into_iter()
                        .all(|m| m.is_none())
                    {
                        // found a big enough slice
                        // println!(
                        //     "found a none slice at {}->{} for id {}",
                        //     search_index,
                        //     search_index + *file_size as usize,
                        //     next_id
                        // );

                        let mut move_counter: usize = 0;
                        while move_counter < *file_size as usize {
                            let higher_elem = memory[file_index + move_counter].take().unwrap();
                            memory[search_index + move_counter].replace(higher_elem);

                            move_counter = move_counter + 1;
                        }

                        // println!("{:?}", memory);

                        break;
                    }
                }

                search_index = search_index + 1;
            }

            if (next_id == 0) {
                break;
            }
            next_id = next_id - 1;
        }

        println!("{:?}", memory);

        // checksum it
        let mut checksum = 0;
        for (i, value) in memory.iter().enumerate() {
            checksum = checksum + (i * value.unwrap_or(0));
        }

        println!("checksum: {}", checksum);
    }
}
