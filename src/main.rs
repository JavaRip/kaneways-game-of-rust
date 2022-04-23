fn main() {
    const GRID_SIZE: i32 = 3;
    let mut rust_plate: [i32; (GRID_SIZE * GRID_SIZE) as usize] =
        [0; (GRID_SIZE * GRID_SIZE) as usize];

    let mut board = String::from("");
    let mut change_board = String::from("");

    for x in 0..rust_plate.len() {
        rust_plate[x] = 1;
        board += &rust_plate[x].to_string();

        board += ",";

        if x as i32 % GRID_SIZE == GRID_SIZE - 1 {
            board += "\n"
        }
    }

    let mut rust_change: [i32; (GRID_SIZE * GRID_SIZE) as usize] =
        [0; (GRID_SIZE * GRID_SIZE) as usize];

    let mut game_loop_count = 0;
    while game_loop_count < 1 {
        game_loop_count += 1;

        for x in 0..rust_plate.len() {
            let mut num_neighbours = 0;

            // top left

            let cell_above_left_index = (x as i32 - GRID_SIZE) - 1;
            if cell_above_left_index >= 0 && (cell_above_left_index as i32 + 1) % GRID_SIZE != 0 {
                if rust_plate[cell_above_left_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            // top

            let cell_above_index = x as i32 - GRID_SIZE;
            if cell_above_index >= 0 {
                if rust_plate[cell_above_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            // top right

            let cell_above_right_index = (x as i32 - GRID_SIZE) + 1;
            if cell_above_right_index >= 0 && (x as i32 + 1) % GRID_SIZE != 0 {
                if rust_plate[cell_above_right_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            // right

            let cell_right_index = x as i32 + 1;
            if cell_right_index % GRID_SIZE != 0 {
                if rust_plate[cell_right_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            // bottom right

            let cell_below_right_index = (x as i32 + GRID_SIZE) + 1;
            if cell_below_right_index < rust_plate.len() as i32 && (x as i32 + 1) % GRID_SIZE != 0 {
                if rust_plate[cell_below_right_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            // bottom

            let cell_below_index = x as i32 + GRID_SIZE;
            if cell_below_index < rust_plate.len() as i32 {
                if rust_plate[cell_below_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            // bottom left

            let cell_below_left_index = (x as i32 + GRID_SIZE) - 1;
            if cell_below_left_index < rust_plate.len() as i32 && x as i32 % GRID_SIZE != 0 {
                if rust_plate[cell_below_left_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            // left

            let cell_left_index = x as i32 - 1;
            if (cell_left_index + 1) % GRID_SIZE != 0 {
                if rust_plate[cell_left_index as usize] == 1 {
                    num_neighbours += 1;
                }
            }

            println!("-----------------------------------");
            println!("cell {} has {} neighbours", x, num_neighbours);
            if rust_plate[x] == 1 {
                if num_neighbours < 2 {
                    println!("cell {} dies", x);
                    change_board += "-1,";
                    rust_change[x] = -1;
                }

                if num_neighbours > 1 && num_neighbours < 4 {
                    println!("cell {} lives", x);
                    change_board += "0,";
                    rust_change[x] = 0;
                }

                if num_neighbours > 3 {
                    println!("cell {} dies", x);
                    change_board += "-1,";
                    rust_change[x] = -1;
                }
            } else {
                if num_neighbours == 3 {
                    println!("cell {} becomes alive", x);
                    change_board += "+1,";
                    rust_change[x] = 1;
                }
            }

            if (x as i32 + 1) % GRID_SIZE == 0 {
                change_board += "\n";
            }
        }
    }

    println!("{}", board);
    println!("{}", change_board);
    for x in 0..rust_plate.len() {
        rust_plate[x] = rust_plate[x] + rust_change[x];
    }

    let mut new_board = String::from("");
    for x in 0..rust_plate.len() {
        new_board += &rust_plate[x].to_string();
        new_board += ",";
        if (x + 1) as i32 % GRID_SIZE == 0 {
            new_board += "\n";
        }
    }

    println!("{}", new_board);
}
