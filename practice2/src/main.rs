use std::collections::HashMap;

fn main() {
    let nums: Vec<usize> = vec![12, 15, 45, 78, 14, 1, 14, 5, 9, 85, 45, 6, 4, 14, 14, 14];
    fn mean_range(numbers: &Vec<usize>) -> usize {
        let mut sum = 0;
        let mut records: HashMap<&usize, u32> = HashMap::new();
        for item in numbers.iter() {
            if !records.contains_key(item) {
                records.insert(item, 1);
            } else {
                let count = match records.get(item) {
                    Some(v) => v + 1,
                    None => 1,
                };
                records.insert(item, count);
            }
            sum += item;
        }
        sum / numbers.len()
    }

    fn bubble_sort(numbers: &mut Vec<usize>) {
        numbers.sort();
        for num in numbers.iter() {}
    }

    println!("{:?}", mean_range(&nums));
}
