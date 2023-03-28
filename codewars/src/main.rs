mod reverse_string;
mod testing_1_2_3;
mod build_tower;
mod sum_1d;

use reverse_string::reverse_words;

fn main() {
    let input = String::from("hello from the other side");
    let reversed = reverse_words(&input);
    println!("Reversed string: {}", reversed);
    println!("Build tower: \n{}", build_tower::tower_builder(5).join("\n"));
}

/*use std::collections::HashMap;


fn main() {
    println!("{}", dna_strand("AT"));
}

fn dna_strand(dna: &str) -> String {
    let mut dna_map = HashMap::new();
    dna_map.insert('A', 'T');
    dna_map.insert('T', 'A');
    dna_map.insert('C', 'G');
    dna_map.insert('G', 'C');

    let mut and = String::from(dna);
    for (i, c) in dna.char_indices() {
        if let Some(&replacement) = dna_map.get(&c) {
            and.replace_range(i..=i, &replacement.to_string());
        }
    }
    return and;
}*/
