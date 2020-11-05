use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let joined: Vec<_> = input.join("").chars().collect();
    if joined.len() == 0 {
        return HashMap::new();
    }

    let chunk_size = joined.len() / worker_count + usize::from(joined.len() % worker_count != 0);
    let partitions: Vec<_> = joined
        .chunks(chunk_size)
        .map(|it| it.iter().collect::<String>())
        .collect();

    let mut threads = Vec::with_capacity(worker_count);
    for i in 0..partitions.len() {
        let part = partitions[i].clone();
        threads.push(thread::spawn(move || {
            let mut piece: HashMap<char, usize> = HashMap::new();
            part.to_ascii_lowercase().chars().for_each(|c| {
                if c.is_alphabetic() {
                    *piece.entry(c).or_insert(0) += 1;
                }
            });
            piece
        }))
    }

    let mut ret: HashMap<char, usize> = HashMap::new();
    for t in threads {
        let part = t.join().unwrap();
        for (key, val) in part.iter() {
            *ret.entry(*key).or_insert(0) += val;
        }
    }
    ret
}
