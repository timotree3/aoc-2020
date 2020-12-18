use std::{collections::HashSet, io::Read};

fn main() {
    let mut scratch = read_starting_state();
    let mut state = HashSet::new();
    for _t in 0..6 {
        state.clone_from(&scratch);
        for &(x, y, z, w) in &state {
            // Populate inactive cells with 3 neighbors
            for (neighbor_x, neighbor_y, neighbor_z, neighbor_w) in neighbors(x, y, z, w) {
                if !scratch.contains(&(neighbor_x, neighbor_y, neighbor_z, neighbor_w))
                    && count_active_neighbors_bounded_by_4(
                        neighbor_x, neighbor_y, neighbor_z, neighbor_w, &state,
                    ) == 3
                {
                    scratch.insert((neighbor_x, neighbor_y, neighbor_z, neighbor_w));
                }
            }

            // Kill active cells without 2 or 3 active neighbors
            let active_neighbors_bounded = count_active_neighbors_bounded_by_4(x, y, z, w, &state);
            if active_neighbors_bounded != 2 && active_neighbors_bounded != 3 {
                scratch.remove(&(x, y, z, w));
            }
        }
    }

    println!("answer: {}", scratch.len());
}

fn read_starting_state() -> HashSet<(usize, usize, usize, usize)> {
    let offset = usize::MAX / 2;
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, chr)| {
                if chr == '#' {
                    Some((offset + x, offset + y, offset, offset))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn count_active_neighbors_bounded_by_4(
    x: usize,
    y: usize,
    z: usize,
    w: usize,
    state: &HashSet<(usize, usize, usize, usize)>,
) -> usize {
    neighbors(x, y, z, w)
        .filter(|coords| state.contains(coords))
        .take(4)
        .count()
}

fn neighbors(
    x: usize,
    y: usize,
    z: usize,
    w: usize,
) -> impl Iterator<Item = (usize, usize, usize, usize)> {
    (x - 1..=x + 1).flat_map(move |neighbor_x| {
        (y - 1..=y + 1).flat_map(move |neighbor_y| {
            (z - 1..=z + 1).flat_map(move |neighbor_z| {
                (w - 1..=w + 1).filter_map(move |neighbor_w| {
                    if neighbor_x == x && neighbor_y == y && neighbor_z == z && neighbor_w == w {
                        None
                    } else {
                        Some((neighbor_x, neighbor_y, neighbor_z, neighbor_w))
                    }
                })
            })
        })
    })
}
