#![feature(str_split_once)]
use std::{collections::HashSet, io::Read, ops::RangeInclusive};

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let mut sections = data.split("\n\n");
    let fields = sections.next().unwrap();
    let fields: Vec<Vec<RangeInclusive<usize>>> = fields
        .lines()
        .map(|line| {
            let (_name, ranges) = line.split_once(": ").unwrap();
            ranges
                .split(" or ")
                .map(|range| {
                    let (start, end) = range.split_once("-").unwrap();
                    start.parse().unwrap()..=end.parse().unwrap()
                })
                .collect()
        })
        .collect();
    let my_ticket: Vec<usize> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let tickets: Vec<Vec<usize>> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|ticket| ticket_is_valid(ticket, &fields))
        .collect();
    let mut possible_fields_per_position: Vec<HashSet<usize>> = (0..fields.len())
        .map(|position| {
            fields
                .iter()
                .enumerate()
                .filter(|(_field_id, ranges)| field_position_is_valid(position, &tickets, ranges))
                .map(|(field_id, _ranges)| field_id)
                .collect()
        })
        .collect();

    loop {
        let known_positions: Vec<(usize, usize)> = possible_fields_per_position
            .iter()
            .enumerate()
            .filter_map(|(position, possibilities)| {
                try_into_single_element(possibilities.iter()).map(|&field_id| (position, field_id))
            })
            .collect();
        for (identified_position, field_id) in known_positions {
            for (other_position, possibilities) in
                possible_fields_per_position.iter_mut().enumerate()
            {
                if other_position != identified_position {
                    possibilities.remove(&field_id);
                    if possibilities.len() == 0 {
                        panic!();
                    }
                }
            }
        }
        let placed_fields: Vec<(usize, usize)> = (0..fields.len())
            .filter_map(|field_id| {
                try_into_single_element(
                    possible_fields_per_position
                        .iter()
                        .enumerate()
                        .filter(|(_position, possibilities)| possibilities.contains(&field_id))
                        .map(|(position, _possibilities)| position),
                )
                .map(|position| (position, field_id))
            })
            .collect();

        for (position, field_id) in placed_fields {
            possible_fields_per_position[position].clear();
            possible_fields_per_position[position].insert(field_id);
        }

        if possible_fields_per_position
            .iter()
            .all(|possibilities| possibilities.len() == 1)
        {
            break;
        }
    }

    let mut unshuffled_my_ticket = vec![0; fields.len()];
    for (position, possibilities) in possible_fields_per_position.into_iter().enumerate() {
        let field_id = try_into_single_element(possibilities).unwrap();
        unshuffled_my_ticket[field_id] = my_ticket[position]
    }

    let answer: usize = unshuffled_my_ticket[..6].iter().product();
    println!("answer: {}", answer);
}

fn try_into_single_element<I: IntoIterator<Item = T>, T>(iterator: I) -> Option<T> {
    let mut iterator = iterator.into_iter();
    match iterator.next() {
        Some(v) if iterator.next().is_none() => Some(v),
        _ => None,
    }
}

fn field_position_is_valid(
    position: usize,
    tickets: &[Vec<usize>],
    ranges: &[RangeInclusive<usize>],
) -> bool {
    tickets
        .iter()
        .map(|ticket| ticket[position])
        .all(|value| value_in_ranges(value, ranges))
}

fn ticket_is_valid(ticket: &[usize], fields: &[Vec<RangeInclusive<usize>>]) -> bool {
    ticket
        .iter()
        .all(|num| fields.iter().any(|ranges| value_in_ranges(*num, ranges)))
}

fn value_in_ranges(value: usize, ranges: &[RangeInclusive<usize>]) -> bool {
    ranges.iter().any(|range| range.contains(&value))
}
