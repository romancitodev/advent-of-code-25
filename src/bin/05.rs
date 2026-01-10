advent_of_code::solution!(5);

const SPECIAL_WHITESPACE: &str = "\r\n\r\n";

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges_str, numbers_str) = input.split_once(SPECIAL_WHITESPACE)?;

    let mut ranges = ranges_str
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            let start = start.parse::<u64>().ok()?;
            let end = end.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|&(start, _)| start);
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());

    // Here we are merging overlapping ranges to optimize the search later
    // For example, if we have ranges (1-5), (4-10), (12-15)
    // We will merge (1-5) and (4-10) into (1-10)
    // But how?
    // We iterate over the sorted ranges and for each range we check if it overlaps with the last range in the merged list
    // If it does, we merge them by updating the end of the last range in the merged list
    // If it doesn't, we simply add the new range to the merged list
    // This way we ensure that we have the minimum number of ranges to check against later
    for (start, end) in ranges {
        // "if we have at least one range merged already and the start of the current range is less than or equal to the end of the last merged range plus one (to account for contiguous ranges)"
        // then we merge the ranges
        if let Some(range) = merged.last_mut()
            && start <= range.1 + 1
        {
            // We update the end of the last merged range to be the maximum of the current end and the last merged end
            // So, for example if we have (1-5) and (4-10) being `start = 4` and `end = 10` and `range = (1, 5)`
            // We will update `range.1` to be `10`, resulting in `range = (1, 10)`
            range.1 = range.1.max(end);
            continue;
        }
        merged.push((start, end));
    }

    let count = numbers_str
        .lines()
        .filter_map(|line| line.parse::<u64>().ok())
        .filter(|&num| {
            merged
                .binary_search_by(|&(start, end)| {
                    if num < start {
                        std::cmp::Ordering::Greater
                    } else if num > end {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_str, ..) = input.split_once(SPECIAL_WHITESPACE)?;

    let mut ranges = ranges_str
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            let start = start.parse::<u64>().ok()?;
            let end = end.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|&(start, _)| start);
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());

    // Here we are merging overlapping ranges to optimize the search later
    // For example, if we have ranges (1-5), (4-10), (12-15)
    // We will merge (1-5) and (4-10) into (1-10)
    // But how?
    // We iterate over the sorted ranges and for each range we check if it overlaps with the last range in the merged list
    // If it does, we merge them by updating the end of the last range in the merged list
    // If it doesn't, we simply add the new range to the merged list
    // This way we ensure that we have the minimum number of ranges to check against later
    for (start, end) in ranges {
        // "if we have at least one range merged already and the start of the current range is less than or equal to the end of the last merged range plus one (to account for contiguous ranges)"
        // then we merge the ranges
        if let Some(range) = merged.last_mut()
            && start <= range.1 + 1
        {
            // We update the end of the last merged range to be the maximum of the current end and the last merged end
            // So, for example if we have (1-5) and (4-10) being `start = 4` and `end = 10` and `range = (1, 5)`
            // We will update `range.1` to be `10`, resulting in `range = (1, 10)`
            range.1 = range.1.max(end);
            continue;
        }
        merged.push((start, end));
    }

    let count = merged
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum::<u64>();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
