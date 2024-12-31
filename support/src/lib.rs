use std::{collections::HashSet, fs};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: i32,
    pub col: i32
}

impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FieldItem {
    item: char,
    point: Point
}
impl FieldItem {
    fn new(c: char, point: Point) -> Self {
        FieldItem { item: c, point: point }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn new(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => panic!("Could not create direction from [{}]", c),
        }
    }
}

pub fn read_input_file_as_string(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn read_input_file_as_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_input_to_typed_groups<T>(path: &str, op: fn(&str) -> T) -> Vec<Vec<T>> {
    read_input_file_as_lines(path)
        .split(String::is_empty)
        .map(Vec::from)
        .map(|i| i.iter().map(|s| op(s)).collect())
        .collect()
}

pub fn parse_strings_to_field_item(input: Vec<String>, ignore: char) -> HashSet<FieldItem> {
    input
        .iter()
        .enumerate()
        .fold(HashSet::new(), |mut points, (row, line)| {
            for (col, c) in line.char_indices() {
                if c != ignore {
                    points.insert(FieldItem::new(c, Point::new(row as i32, col as i32)));
                }
            }
            points
        })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_input_file_returns_string() {
        let result: Vec<String> = read_input_file_as_lines("resource/day01_small");
        assert_eq!(
            result,
            vec!["1000", "2000", "3000", "", "4000", "", "5000", "6000"]
        );
    }

    #[test]
    fn read_input_to_typed_groups_returns_string() {
        let result: Vec<Vec<String>> =
            read_input_to_typed_groups("resource/day01_small", |s| s.to_owned());

        assert_eq!(
            result,
            vec![
                vec!["1000", "2000", "3000"],
                vec!["4000"],
                vec!["5000", "6000"]
            ]
        );
    }

    #[test]
    fn read_input_to_typed_groups_returns_i32() {
        let result: Vec<Vec<i32>> =
            read_input_to_typed_groups("resource/day01_small", |s| s.parse::<i32>().unwrap());

        assert_eq!(
            result,
            vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000]]
        );
    }

    #[test]
    fn read_input_to_typed_groups_returns_float() {
        let result: Vec<Vec<f32>> =
            read_input_to_typed_groups("resource/day01_small", |s| s.parse::<f32>().unwrap());

        assert_eq!(
            result,
            vec![
                vec![1000.0, 2000.0, 3000.0],
                vec![4000.0],
                vec![5000.0, 6000.0]
            ]
        );
    }

    #[test]
    fn test_parse_strings_to_field_item() {
        let input = r".|...\....".to_string();
        assert_eq!(
            HashSet::from([FieldItem::new('|', Point::new(0, 1)), FieldItem::new('\\', Point::new(0, 5))]),
            parse_strings_to_field_item(vec![input], '.')
        );
    }
}
