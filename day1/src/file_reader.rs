use std::fs;

pub struct FileReader {
	pub file: String
}

impl FileReader {
	
	pub fn calculate_calories(&self) -> i32 {
		let content = fs::read_to_string(&self.file)
			.unwrap();
			
		let calories: i32 = FileReader::parse_and_calculate(content);
		calories
	}
	
	fn parse_and_calculate(content: String) -> i32 {
		let mut first_elf_calories: i32 = 0;
		let mut second_elf_calories: i32 = 0;
		let mut third_elf_calories: i32 = 0;

		let mut current_calories: i32 = 0;
		for l in content.lines() {
			if l.is_empty() {
				current_calories = 0;
				continue;
			}

			current_calories += l.parse::<i32>().unwrap();
			if current_calories > first_elf_calories {
				third_elf_calories = second_elf_calories;
				second_elf_calories = first_elf_calories;
				first_elf_calories = current_calories;
			} else if current_calories > second_elf_calories { 
				third_elf_calories = second_elf_calories;
				second_elf_calories = current_calories;
			} else if current_calories > third_elf_calories {
				third_elf_calories = current_calories;
			}
		}
		
		first_elf_calories + second_elf_calories + third_elf_calories
	}
}
