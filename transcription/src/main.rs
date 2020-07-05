use std::io;

fn main() {
    println!("Transcription machine!");

    println!("Please input your DNA sequence.");

    let mut dna = String::new();

    io::stdin()
        .read_line(&mut dna)
        .expect("Failed to read line");

    let rna = dna.replace("T", "U");

    println!("Your RNA strand is: {}", rna);
}
