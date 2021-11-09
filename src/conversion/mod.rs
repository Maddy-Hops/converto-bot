#[derive(Debug, PartialEq)]
enum Units {
	Miles(f64, String),
	Feet(f64, String),
	Inches(f64, String),
	Kilometers(f64, String),
	Meters(f64, String),
	Centimeters(f64, String),
	Pounds(f64, String),
	Ounces(f64, String),
	Kilograms(f64, String),
	Grams(f64, String),
	DegreesCelsius(f64, String),
	DegreesFahrenheit(f64, String),
}

impl Units {
	fn new(val: f64, unit: &str) -> Self {
		match unit {
			"km" | "kms" | "kilometer" | "kilometers" => Units::Kilometers(val, String::from("km")),
			"m" | "ms" | "meter" | "meters" => Units::Meters(val, String::from("m")),
			"cm" | "cms" | "centimeter" | "centimeters" => Units::Centimeters(val, String::from("cm")),
			"mile" | "miles" => Units::Miles(val, String::from("miles")),
			"feet" | "foot" | "ft" => Units::Feet(val, String::from("ft")),
			"inches" | "inch" => Units::Inches(val, String::from("inches")),
			"kg" | "kilogram" | "kilograms" => Units::Kilograms(val, String::from("kg")),
			"g" | "gram" | "grams" => Units::Grams(val, String::from("grams")),
			"lbs" | "pound" | "pounds" => Units::Pounds(val, String::from("lbs")),
			"oz" | "ounces" | "ounce" => Units::Ounces(val, String::from("oz")),
			"c" | "℃" | "celsius" => Units::DegreesCelsius(val, String::from("℃")),
			"f" | "℉" | "fahrenheit" => Units::DegreesFahrenheit(val, String::from("℉")),
			_ => panic!("Unknown type was passed into Units::new(), check your input"),
		}
	}

	fn convert(&self) -> Units {
		match self {
			// length
			// imperial
			Units::Miles(val, _) => Units::Kilometers(val * 1.609344, String::from("km")),
			Units::Feet(val, _) => Units::Meters(val * 0.3048, String::from("m")),
			Units::Inches(val, _) => Units::Centimeters(val * 2.54, String::from("cm")),
			// metric
			Units::Kilometers(val, _) => Units::Miles(val * 0.6213712, String::from("miles")),
			Units::Meters(val, _) => Units::Feet(val / 0.3048, String::from("ft")),
			Units::Centimeters(val, _) => Units::Inches(val / 2.54, String::from("inches")),
			// weight
			// imperial
			Units::Pounds(val, _) => Units::Kilograms(val * 0.4535924, String::from("kg")),
			Units::Ounces(val, _) => Units::Grams(val * 28.34952, String::from("grams")),
			// metric
			Units::Kilograms(val, _) => Units::Pounds(val / 0.4535924, String::from("lbs")),
			Units::Grams(val, _) => Units::Ounces(val / 28.34952, String::from("oz")),
			// temperature
			Units::DegreesCelsius(val, _) => Units::DegreesFahrenheit(val * 1.8 + 32_f64, String::from("℉")),
			Units::DegreesFahrenheit(val, _) => Units::DegreesCelsius((val - 32_f64) / 1.8, String::from("℃")),
		}
	}

	fn destruct_enum(unit: &Units) -> (f64, String) {
		let value;
		let unit_identifier;
		match unit {
			Units::Miles(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::Feet(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::Inches(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			// metric
			Units::Kilometers(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::Meters(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::Centimeters(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			// weight
			// imperial
			Units::Pounds(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::Ounces(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			// metric
			Units::Kilograms(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::Grams(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::DegreesCelsius(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
			Units::DegreesFahrenheit(val, unit) => {
				value = val;
				unit_identifier = unit;
			}
		}
		(*value, unit_identifier.to_string())
	}
}
const LIST_POSSIBLE: &[&str] = &[
	"km",
	"kms",
	"kilometers",
	"kilometer",
	"m",
	"meters",
	"meter",
	"cm",
	"centimeters",
	"centimeter",
	"kg",
	"kilograms",
	"kilogram",
	"grams",
	"gram",
	"mile",
	"miles",
	"ft",
	"feet",
	"foot",
	"inches",
	"inch",
	"lbs",
	"pounds",
	"pound",
	"oz",
	"ounces",
	"ounce",
	"c",
	"℃",
	"celsius",
	"f",
	"℉",
	"fahrenheit",
];

fn parse_input(msg: &str) -> Option<Vec<Units>> {
	let mut has_convertibles = false;
	for elem in LIST_POSSIBLE {
		if msg.to_lowercase().contains(elem) {
			has_convertibles = true;
			break;
		}
	}
	if !has_convertibles {
		return None;
	}
	let msg = msg.to_lowercase();
	let msg: Vec<_> = msg.split_ascii_whitespace().collect();
	let mut values_vec = vec![];
	for i in 0..msg.len() {
		let word = msg[i].trim_end_matches(&[',', '.', '/', ';', ':', '|', '"', '\'', '\\'][..]);
		if LIST_POSSIBLE.contains(&word) {
			if let Ok(val) = msg[i - 1].parse::<f64>() {
				values_vec.push(Units::new(val, word));
			}
		}
	}
	if !values_vec.is_empty() {
		Some(values_vec)
	} else {
		None
	}
}

fn assemble_response(values_vec: &[Units]) -> String {
	let mut response = String::new();
	for v in values_vec {
		let (value, unit) = Units::destruct_enum(v);
		let (converted_value, converted_unit) = Units::destruct_enum(&v.convert());
		if converted_value < 1.0 {
			response.push_str(&format!(
				"{} {} is {} {}\n",
				value, unit, converted_value, converted_unit
			));
		} else {
			response.push_str(&format!(
				"{} {} is {:.2} {}\n",
				value, unit, converted_value, converted_unit
			));
		}
	}
	response
}

pub fn respond_to_msg(msg: &str) -> Option<String> {
	parse_input(msg).map(|units| assemble_response(&units))
}

#[cfg(test)]
mod tests {
	use super::*;
	use float_cmp::{approx_eq, F64Margin};

	#[test]
	fn unit_conversion() {
		if let Units::Meters(val_left, _) = Units::Meters(3.048, String::from("m")) {
			if let Units::Meters(val_right, _) = Units::Feet(10.0, String::from("ft")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::Feet(val_left, _) = Units::Feet(32.80839895, String::from("ft")) {
			if let Units::Feet(val_right, _) = Units::Meters(10.0, String::from("m")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::Kilograms(val_left, _) = Units::Kilograms(4.535924, String::from("kg")) {
			if let Units::Kilograms(val_right, _) = Units::Pounds(10.0, String::from("lbs")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::Pounds(val_left, _) = Units::Pounds(22.04623, String::from("lbs")) {
			if let Units::Pounds(val_right, _) = Units::Kilograms(10.0, String::from("kg")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::Ounces(val_left, _) = Units::Ounces(0.3527396, String::from("oz")) {
			if let Units::Ounces(val_right, _) = Units::Grams(10.0, String::from("g")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::Grams(val_left, _) = Units::Grams(283.4952, String::from("g")) {
			if let Units::Grams(val_right, _) = Units::Ounces(10.0, String::from("oz")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::Miles(val_left, _) = Units::Miles(10.0, String::from("miles")) {
			if let Units::Miles(val_right, _) = Units::Kilometers(16.09344, String::from("km")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::Kilometers(val_left, _) = Units::Kilometers(10.0, String::from("km")) {
			if let Units::Kilometers(val_right, _) = Units::Miles(6.213712, String::from("miles")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::DegreesCelsius(val_left, _) = Units::DegreesCelsius(10.0, String::from("℃")) {
			if let Units::DegreesCelsius(val_right, _) = Units::DegreesFahrenheit(50.0, String::from("℉")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::DegreesCelsius(val_left, _) = Units::DegreesCelsius(0.0, String::from("℃")) {
			if let Units::DegreesCelsius(val_right, _) = Units::DegreesFahrenheit(32.0, String::from("℉")).convert() {
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::DegreesCelsius(val_left, _) = Units::DegreesCelsius(-40.0, String::from("℃")) {
			if let Units::DegreesCelsius(val_right, _) = Units::DegreesFahrenheit(-40.0, String::from("℉")).convert()
			{
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::DegreesFahrenheit(val_left, _) = Units::DegreesFahrenheit(15.0, String::from("℃")) {
			if let Units::DegreesFahrenheit(val_right, _) =
				Units::DegreesCelsius(-9.444444, String::from("℉")).convert()
			{
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::DegreesFahrenheit(val_left, _) = Units::DegreesFahrenheit(0.0, String::from("℃")) {
			if let Units::DegreesFahrenheit(val_right, _) =
				Units::DegreesCelsius(-17.77778, String::from("℉")).convert()
			{
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
		if let Units::DegreesFahrenheit(val_left, _) = Units::DegreesFahrenheit(-40.0, String::from("℃")) {
			if let Units::DegreesFahrenheit(val_right, _) = Units::DegreesCelsius(-40.0, String::from("℉")).convert()
			{
				assert!(
					approx_eq!(
						f64,
						val_left,
						val_right,
						F64Margin {
							epsilon: 0.001,
							ulps: 2
						}
					),
					"\nleft != right\n{} != {}",
					val_left,
					val_right
				);
			} else {
				panic!("FAILED TO ASSIGN")
			}
		} else {
			panic!("FAILED TO ASSIGN")
		}
	}

	#[test]
	fn parsing_msg_single_unit() {
		let msg = "Hello, I am 171 cm tall";
		assert_eq!(
			parse_input(msg),
			Some(vec![Units::Centimeters(171.0, String::from("cm"))])
		);
	}

	#[test]
	fn parsing_msg_multiple_units() {
		let msg = "Hello, I am 171 cm tall and weigh 140 pounds";
		assert_eq!(
			parse_input(msg),
			Some(vec![
				Units::Centimeters(171.0, String::from("cm")),
				Units::Pounds(140.0, String::from("lbs"))
			])
		);
	}

	#[test]
	fn parsing_msg_malformed_single_unit() {
		let msg = "Hello, I am none cm tall";
		assert_eq!(parse_input(msg), None);
	}
	#[test]
	fn parsing_msg_malformed_unit_plus_additional_correct_unit() {
		let msg = "Hello, I am none cm tall and weigh 140 pounds";
		assert_eq!(
			parse_input(msg),
			Some(vec![Units::Pounds(140.0, String::from("lbs"))])
		);
	}

	#[test]
	fn parsing_msg_multiple_malformed_units_multiple_correct_units() {
		let msg = "Hello, I am none cm tall and weigh 140 pounds and my city is 343 kms in area and my cat's name is little Foot";
		assert_eq!(
			parse_input(msg),
			Some(vec![
				Units::Pounds(140.0, String::from("lbs")),
				Units::Kilometers(343.0, String::from("km"))
			])
		);
	}

	#[test]
	fn parsing_msg_strip_punctuation_correct_units() {
		let msg = "Hello, I am none cm tall and weigh 140 pounds. My city is 343 kms in area and my cat's name is little Foot";
		assert_eq!(
			parse_input(msg),
			Some(vec![
				Units::Pounds(140.0, String::from("lbs")),
				Units::Kilometers(343.0, String::from("km"))
			])
		);
	}

	#[test]
	fn parsing_msg_strip_multiple_punctuation_correct_units() {
		let msg = "Hello, I am none cm tall and weigh 140 pounds,|.,;. My city is 343 kms.,; in area and my cat's name is little Foot";
		assert_eq!(
			parse_input(msg),
			Some(vec![
				Units::Pounds(140.0, String::from("lbs")),
				Units::Kilometers(343.0, String::from("km"))
			])
		);
	}

	#[test]
	fn parsing_msg_parse_floats() {
		let msg = "Maddy-hops is exactly 0.00171 kilometers tall";
		assert_eq!(
			parse_input(msg),
			Some(vec![Units::Kilometers(0.00171, String::from("km"))])
		);
	}

	#[test]
	fn destructing_units() {
		let unit = Units::Feet(300.0, String::from("ft"));
		assert_eq!(Units::destruct_enum(&unit,), (300.0, String::from("ft")))
	}

	#[test]
	fn assemble_response_single_unit() {
		let msg = "Maddy-hops is exactly 0.00171 kilometers tall";
		let units_vec = parse_input(msg).unwrap();
		assert_eq!(
			"0.00171 km is 0.001062544752 miles\n".to_string(),
			assemble_response(&units_vec)
		);
	}

	#[test]
	fn assemble_response_multiple_units() {
		let msg = "Maddy-hops is exactly 0.00171 kilometers tall and weighs 140 pounds.";
		let units_vec = parse_input(msg).unwrap();
		assert_eq!(
			"0.00171 km is 0.001062544752 miles\n140 lbs is 63.50 kg\n".to_string(),
			assemble_response(&units_vec)
		);
	}

	#[test]
	fn assemble_response_degrees() {
		let msg = "it's -30 c where I live rn";
		let units_vec = parse_input(msg).unwrap();
		assert_eq!(
			"-30 ℃ is -22 ℉\n".to_string(),
			assemble_response(&units_vec)
		);
	}
}
