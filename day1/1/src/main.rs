use file_reader::FileReader;

mod file_reader;

fn main() {
    let reader = FileReader {
        file: String::from("./input")
    };

    println!("{}", reader.calculate_calories());
}
