fn main() {
    const GRID_SIZE: i32 = 3;
    let mut rust_plate: [i32; (GRID_SIZE * GRID_SIZE) as usize] =
        [0; (GRID_SIZE * GRID_SIZE) as usize];

    for x in 0..rust_plate.len() {
        rust_plate[x] = 1;
    }

    for y in 0..3 {
        let mut num_neighbours_arr: [i32; (GRID_SIZE * GRID_SIZE) as usize] =
            [0; (GRID_SIZE * GRID_SIZE) as usize];

        for x in 0..rust_plate.len() {
            num_neighbours_arr[x] = find_num_neighbours(&rust_plate, x, GRID_SIZE);
        }

        let mut cell_change: [i32; (GRID_SIZE * GRID_SIZE) as usize] =
            [0; (GRID_SIZE * GRID_SIZE) as usize];

        for x in 0..cell_change.len() {
            cell_change[x] = get_cell_change(&num_neighbours_arr, x, rust_plate[x]);
        }

        println!("============");
        print_arr(&rust_plate, GRID_SIZE);
        for x in 0..rust_plate.len() {
            rust_plate[x] += cell_change[x];
        }
        print_arr(&rust_plate, GRID_SIZE);
        println!("============");
    }
}

fn print_arr(arr: &[i32], grid_size: i32) {
    let mut print_str = String::from("");
    for x in 0..arr.len() {
        print_str += &arr[x].to_string();
        print_str += ",";
        if (x as i32 + 1) % grid_size == 0 {
            print_str += "\n";
        }
    }

    println!("{}", print_str);
}

fn get_cell_change(num_neighbours_arr: &[i32], index: usize, alive: i32) -> i32 {
    let mut change = 0;
    if alive == 1 {
        if num_neighbours_arr[index] < 2 {
            change = -1
        }

        if num_neighbours_arr[index] > 1 && num_neighbours_arr[index] < 4 {
            change = 0
        }

        if num_neighbours_arr[index] > 3 {
            change = -1
        }
    } else {
        if num_neighbours_arr[index] == 3 {
            change = 1
        }
    }

    return change;
}

fn find_num_neighbours(rust_plate: &[i32], index: usize, grid_size: i32) -> i32 {
    let mut num_neighbours: i32 = 0;

    // top left

    let cell_above_left_index = (index as i32 - grid_size) - 1;
    if cell_above_left_index >= 0 && (cell_above_left_index as i32 + 1) % grid_size != 0 {
        if rust_plate[cell_above_left_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    // top

    let cell_above_index = index as i32 - grid_size;
    if cell_above_index >= 0 {
        if rust_plate[cell_above_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    // top right

    let cell_above_right_index = (index as i32 - grid_size) + 1;
    if cell_above_right_index >= 0 && (index as i32 + 1) % grid_size != 0 {
        if rust_plate[cell_above_right_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    // right

    let cell_right_index = index as i32 + 1;
    if cell_right_index % grid_size != 0 {
        if rust_plate[cell_right_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    // bottom right

    let cell_below_right_index = (index as i32 + grid_size) + 1;
    if cell_below_right_index < rust_plate.len() as i32 && (index as i32 + 1) % grid_size != 0 {
        if rust_plate[cell_below_right_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    // bottom

    let cell_below_index = index as i32 + grid_size;
    if cell_below_index < rust_plate.len() as i32 {
        if rust_plate[cell_below_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    // bottom left

    let cell_below_left_index = (index as i32 + grid_size) - 1;
    if cell_below_left_index < rust_plate.len() as i32 && index as i32 % grid_size != 0 {
        if rust_plate[cell_below_left_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    // left

    let cell_left_index = index as i32 - 1;
    if (cell_left_index + 1) % grid_size != 0 {
        if rust_plate[cell_left_index as usize] == 1 {
            num_neighbours += 1;
        }
    }

    return num_neighbours;
}
