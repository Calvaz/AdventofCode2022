use std::fs;

pub struct FileReader {
	pub file: String
}

impl FileReader {
	
	pub fn calculate_calories(&self) -> i32 {
		let content = fs::read_to_string(&self.file)
			.unwrap();
			
		let calories = FileReader::parse_and_calculate(content);
		calories
	}
	
	fn parse_and_calculate(content: String) -> i32 {
		let mut max_calories = 0;

		let mut current_calories: i32 = 0;
		for l in content.lines() {
			if l.is_empty() {
				current_calories = 0;
				continue;
			}

			current_calories += l.parse::<i32>().unwrap();
			if current_calories > max_calories {
				max_calories = current_calories;
			}
		}
		
		max_calories
	}
}
