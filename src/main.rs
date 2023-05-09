use rand::distributions::{Distribution, WeightedIndex};

const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 30;

fn get_possible_tiles(map: &[Option<u8>], constraints: &[(u8, &[u8])], x: usize, y: usize) -> Vec<u8> {
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
            let neighbor_constraints = constraints[*tile as usize].1;
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
    let mut map: Vec<Option<u8>> = vec![None; MAP_WIDTH * MAP_HEIGHT];
    let constraints: &[(u8, &[u8])] = &[
        (12, &[0, 1, 3]),
        (5, &[0, 1, 2]),
        (10, &[1, 2]),
        (1, &[0]),
    ];

    let mut rng = rand::thread_rng();

    for i in 0..MAP_WIDTH * MAP_HEIGHT {
        let x = i % MAP_WIDTH;
        let y = i / MAP_WIDTH;

        let possible_neighbors = get_possible_tiles(&map, constraints, x, y);

        if possible_neighbors.len() == 0 {
            println!("No solution found");
            return;
        }

        let weights: Vec<_> = possible_neighbors
            .iter()
            .map(|tile| constraints[*tile as usize].0)
            .collect();
        let dist = WeightedIndex::new(&weights).unwrap();

        // randomly choose a tile from the possible neighbors based on the weights
        map[i] = Some(possible_neighbors[dist.sample(&mut rng)]);
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
                    3 => print!("\x1b[31m{}\x1b[0m", 'h'),
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
