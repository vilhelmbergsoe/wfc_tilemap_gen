use rand::seq::SliceRandom;
use rand::Rng;

const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 30;

fn get_possible_tiles(map: &[Option<u8>], constraints: &[&[u8]], x: usize, y: usize) -> Vec<u8> {
    let mut neighbors: [Option<u8>; 8] = [None; 8];

    // BOTTOM LEFT
    if x > 0 && y < MAP_HEIGHT - 1 {
        neighbors[0] = map[x - 1 + (y + 1) * MAP_WIDTH];
    }

    // LEFT
    if x > 0 {
        neighbors[1] = map[x - 1 + y * MAP_WIDTH];
    }

    // TOP LEFT
    if x > 0 && y > 0 {
        neighbors[3] = map[x - 1 + (y - 1) * MAP_WIDTH];
    }

    // TOP
    if y > 0 {
        neighbors[4] = map[x + (y - 1) * MAP_WIDTH];
    }

    // TOP RIGHT
    if x < MAP_WIDTH - 1 && y > 0 {
        neighbors[5] = map[x + 1 + (y - 1) * MAP_WIDTH];
    }

    // BOTTOM RIGHT
    if x < MAP_WIDTH - 1 && y < MAP_HEIGHT - 1 {
        neighbors[6] = map[x + 1 + (y + 1) * MAP_WIDTH];
    }

    // BOTTOM
    if y < MAP_HEIGHT - 1 {
        neighbors[7] = map[x + (y + 1) * MAP_WIDTH];
    }

    let mut possible_tiles: Vec<u8> = Vec::new();
    for i in 0..constraints.len() {
        possible_tiles.push(i as u8);
    }

    for neighbor in neighbors.iter() {
        if let Some(tile) = neighbor {
            // for tile in constraints[*tile as usize].iter() {
            //     if !possible_tiles.contains(tile) {
            //         possible_tiles.push(*tile);
            //     }
            // }
            let neighbor_constraints = constraints[*tile as usize];
            let mut allowed_tiles: Vec<u8> = Vec::new();

            for tile in possible_tiles.iter() {
                if neighbor_constraints.contains(tile) {
                    allowed_tiles.push(*tile);
                }
            }

            possible_tiles = allowed_tiles;
        }
    }

    possible_tiles
}

fn main() {
    let mut map: [Option<u8>; MAP_WIDTH * MAP_HEIGHT] = [None; MAP_WIDTH * MAP_HEIGHT];
    // set half of the map to grass
    // for y in 0..MAP_HEIGHT {
    //     for x in 0..MAP_WIDTH {
    //         if x < MAP_WIDTH / 2 {
    //             map[x + y * MAP_WIDTH] = Some(b'g');
    //         }
    //     }
    // }
    let constraints: &[&[u8]] = &[
        &[0, 1],
        &[0, 1, 2],
        &[1, 2]
    ];

    let mut rng = rand::thread_rng();

    for i in 0..MAP_WIDTH * MAP_HEIGHT {
        let x = i % MAP_WIDTH;
        let y = i / MAP_WIDTH;

        let possible_neighbors = get_possible_tiles(&map, constraints, x, y);

        // pick random tile from possible neighbors
        if let Some(tile) = possible_neighbors.as_slice().choose(&mut rng) {
            map[i] = Some(*tile);
        }

        // display_map(&map);
        // reset_cursor();
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }

    display_map(&map);
}

fn reset_cursor() {
    print!("\x1B[{}A\x1B[{}D", MAP_HEIGHT + 1, MAP_WIDTH);
}

fn display_map(map: &[Option<u8>]) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if let Some(tile) = map[x + y * MAP_WIDTH] {
                match tile {
                    0 => print!("\x1b[32m{}\x1b[0m", 'g'),
                    1 => print!("\x1b[33m{}\x1b[0m", 's'),
                    2 => print!("\x1b[34m{}\x1b[0m", 'w'),
                    _ => (),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}
