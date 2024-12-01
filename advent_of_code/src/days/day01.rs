use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../inputs/day01.txt");
    let (list1, list2) = parse_two_lists(input);
    let result = compute_sorted_abs_difference_sum(&list1, &list2);
    println!("Sum of sorted element-wise differences: {}", result);

    let frequency = compute_frequency(&list2);
    let result2 = compute_similarity_score(&list1, &frequency);
    println!("Similarity distance: {}", result2);
}

fn parse_two_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok());
        if let (Some(a), Some(b)) = (numbers.next(), numbers.next()) {
            list1.push(a);
            list2.push(b);
        }
    }

    (list1, list2)
}

fn compute_sorted_abs_difference_sum(list1: &[i32], list2: &[i32]) -> i32 {
    let mut sorted_list1 = list1.to_vec();
    let mut sorted_list2 = list2.to_vec();

    sorted_list1.sort();
    sorted_list2.sort();

    sorted_list1.iter()
        .zip(sorted_list2.iter())
        .map(|(a, b)| (a - b).abs()) // Take absolute difference
        .sum()
}

fn compute_frequency(list: &[i32]) -> HashMap<i32, usize> {
    let mut frequency = HashMap::new();

    for &item in list {
        *frequency.entry(item).or_insert(0) += 1;
    }

    frequency
}

fn compute_similarity_score(list1: &[i32], frequency: &HashMap<i32, usize>) -> i32 {
    list1
        .iter()
        .map(|&item| {
        let freq = *frequency.get(&item).unwrap_or(&0); // Dereference to get usize
        freq as i32 * item
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_two_lists() {
        let input = "1 2\n3 4\n5 6";
        let (list1, list2) = parse_two_lists(input);
        assert_eq!(list1, vec![1, 3, 5]);
        assert_eq!(list2, vec![2, 4, 6]);
    }

    #[test]
    fn test_compute_sorted_abs_difference_sum() {
        let list1 = vec![1, 3, 5];
        let list2 = vec![2, 4, 6];
        let result = compute_sorted_abs_difference_sum(&list1, &list2);
        assert_eq!(result, 3); // Explanation: |1-2| + |3-4| + |5-6| = 1 + 1 + 1 = 3
    }
}

