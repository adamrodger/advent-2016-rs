use aoc_runner_derive::aoc;

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    calculate_safe_tiles(input, 40)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    calculate_safe_tiles(input, 400_000)
}

fn calculate_safe_tiles(input: &str, iterations: usize) -> usize {
    let mut current = input.trim().chars().map(|c| c == '^').collect::<Vec<_>>();
    let mut safe = current.iter().filter(|&&p| !p).count();

    for _ in 0..iterations - 1 {
        current = transform(&current);
        safe += current.iter().filter(|&&p| !p).count();
    }

    safe
}

/// Transform the current row into the next row
fn transform(input: &[bool]) -> Vec<bool> {
    (0..input.len()).map(|i| is_trap(i, input)).collect()
}

/// Check if the next row at the given index should be a trap or not
///
/// The rules equate to:
///     true    true    false - left and centre
///     true    false   false - left only
///     false   true    true  - centre and right
///     false   false   true  - right only
///
/// so the centre value doesn't actually matter, it's just the same as left != right
fn is_trap(i: usize, input: &[bool]) -> bool {
    let left = if i > 0 {
        input.get(i - 1).unwrap()
    } else {
        &false
    };

    // manual bounds checking here results in ~5% performance improvement over unwrap_or
    let right = if i < input.len() - 1 {
        input.get(i + 1).unwrap()
    } else {
        &false
    };

    left != right
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day18.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2005);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 20_008_491);
    }
}
