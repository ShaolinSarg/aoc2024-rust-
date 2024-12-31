use std::collections::{BTreeSet, HashMap};

use support::read_input_file_as_lines;

fn split_pairs(input: &str) -> Vec<i32> {
    input.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
}

fn _split_list_of_pairs(list: Vec<String>) -> (BTreeSet<i32>, BTreeSet<i32>) {
    list.iter().fold((BTreeSet::new(), BTreeSet::new()), |mut acc, x| {
        let v = split_pairs(x);
        acc.0.insert(v[0]);
        acc.1.insert(v[1]);
        acc
    })
}

fn split_list_of_pairs_dups(list: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right) = list.iter().fold((Vec::new(), Vec::new()), |mut acc, x| {
        let v = split_pairs(x);
        acc.0.push(v[0]);
        acc.1.push(v[1]);
        acc
    });

    left.sort();
    right.sort();
    (left, right)
}

fn diff_pairs<I1>(t1: I1, t2: I1) -> Vec<u32> 
where
    I1: IntoIterator<Item = i32> {
    t1.into_iter().zip(t2.into_iter())
        .map(|(x, y)| y.abs_diff(x))
        .collect()
}

fn similarity_scores<I1>(t1: I1, t2: I1) -> Vec<i32> 
where
    I1: IntoIterator<Item = i32> {
        let occurences = t2.into_iter().fold(HashMap::new(), |mut acc, x| {
            acc.insert(x, acc.get(&x).unwrap_or(&0) + 1);
            acc
        });

        t1.into_iter().map( | x | x * *occurences.get(&x).unwrap_or(&0) as i32).collect()
}


pub fn day01_part1_answer(path_to_input: &str) -> u32 {
    let input = read_input_file_as_lines(path_to_input);
    let (left, right) = split_list_of_pairs_dups(input);
    let v = diff_pairs(left, right);
    v.iter().sum()
}

pub fn day01_part2_answer(path_to_input: &str) -> i32 {
    let input = read_input_file_as_lines(path_to_input);
    let (left, right) = split_list_of_pairs_dups(input);
    let v = similarity_scores(left, right);
    v.iter().sum()
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_split_pairs() {
        let i = "3   4".to_string();
        let v = split_pairs(&i);

        assert_eq!(vec![3, 4], v);
    }

    #[test]
    fn test_split_list_of_pairs() {
        let l = vec!["3   4".to_string(), "5   6".to_string(), "7   8".to_string()];
        let l2 = vec!["7   8".to_string(), "5   4".to_string(), "3   6".to_string()];
        let (l_sorted, r_sorted) = _split_list_of_pairs(l);
        let (l_unsorted, r_unsorted)= _split_list_of_pairs(l2);
        
        assert_eq!(BTreeSet::from([3, 5, 7]), l_sorted);
        assert_eq!(BTreeSet::from([4, 6, 8]), r_sorted);

        assert_eq!(BTreeSet::from([3, 5, 7]), l_unsorted);
        assert_eq!(BTreeSet::from([4, 6, 8]), r_unsorted);
    }

    #[test]
    fn test_diff_pairs() {
        let t1 = BTreeSet::from([3, 5, 7]);
        let t2 = BTreeSet::from([4, 6, 8]);
        let v = diff_pairs(t1, t2);

        assert_eq!(vec![1, 1, 1], v);
    }

    #[test]
    fn test_simple_example() {
        let input = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string()];

    let (left, right) = split_list_of_pairs_dups(input);

    let v = diff_pairs(left, right);
    
    assert_eq!(vec![2,1,0,1,2,5], v);

    assert_eq!(11u32, v.iter().sum());
    
    }

}

#[test]
fn test_similarity_example() {
    let input = vec![
        "3   4".to_string(),
        "4   3".to_string(),
        "2   5".to_string(),
        "1   3".to_string(),
        "3   9".to_string(),
        "3   3".to_string()];

    let (left, right) = split_list_of_pairs_dups(input);

    let v = similarity_scores(left, right);

    assert_eq!(31i32, v.iter().sum());

}
