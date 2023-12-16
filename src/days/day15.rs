#[derive(Clone, Debug)]
struct Lens {
    pub label: String,
    pub focal_length: usize,
}

fn hashify(step: &str) -> usize {
    step.bytes()
        .fold(0, |acc, byte| ((acc + byte as usize) * 17) % 256)
}

fn get_lens_box<'a>(label: &str, slots: &'a mut [Option<Vec<Lens>>]) -> &'a mut Vec<Lens> {
    let slot = &mut slots[hashify(label)];
    if slot.is_none() {
        slot.replace(Vec::new());
    }
    slot.as_mut().unwrap()
}

fn update_lenses(step: &str, slots: &mut [Option<Vec<Lens>>]) {
    if let Some((label, focal_str)) = step.split_once('=') {
        // Replace or add lens!
        let lens_box = get_lens_box(label, slots);
        let matching_lens_index = lens_box.iter().position(|lens| lens.label == label);
        let focal_length = focal_str.parse().unwrap();
        if let Some(index) = matching_lens_index {
            lens_box[index].focal_length = focal_length;
        } else {
            lens_box.push(Lens {
                label: label.to_owned(),
                focal_length,
            });
        }
    } else {
        // Remove lens!
        let label = &step[0..(step.len() - 1)];
        let lens_box = get_lens_box(label, slots);
        if let Some(index) = lens_box.iter().position(|lens| lens.label == label) {
            lens_box.remove(index);
        }
    }
}

fn calculate_focusing_power(box_index: usize, lens_box: Vec<Lens>) -> usize {
    lens_box
        .into_iter()
        .enumerate()
        .map(|(lens_index, lens)| (box_index + 1) * (lens_index + 1) * lens.focal_length)
        .sum()
}

pub fn day15_star1(input: &str) -> usize {
    input.split(',').map(hashify).sum()
}

pub fn day15_star2(input: &str) -> usize {
    let mut slots: [Option<Vec<Lens>>; 256] = std::array::from_fn(|_| None);
    for step in input.split(',') {
        update_lenses(step, &mut slots);
    }
    slots
        .into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|lens_box| calculate_focusing_power(i, lens_box)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn day15_star1_example() {
        let actual = day15_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 1320);
    }

    #[test]
    fn day15_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day15.txt"))?;
        let actual = day15_star1(&file);
        Ok(assert_eq!(actual, 513_214))
    }

    #[test]
    fn day15_star2_example() {
        let actual = day15_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 145);
    }

    #[test]
    fn day15_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day15.txt"))?;
        let actual = day15_star2(&file);
        Ok(assert_eq!(actual, 258_826))
    }
}
