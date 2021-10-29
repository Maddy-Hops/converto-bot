use std::fmt::Result;
use float_cmp::{self, F64Margin, approx_eq};


#[derive(Debug,PartialEq)]
enum Units {
	Miles(f64,String),
	Feet(f64, String),
	Inches(f64, String),
	Kilometers(f64,String),
	Meters(f64, String),
	Centimeters(f64, String),
	Pounds(f64,String),
	Ounces(f64,String),
	Kilograms(f64, String),
	Grams(f64, String),	
}

impl Units {
	fn convert(&self) -> Units {
		match self {
			// length
			// imperial
			Units::Miles(val,_) => Units::Kilometers(val * 1.609344,String::from("km")),
			Units::Feet(val,_) => Units::Meters(val * 0.3048,String::from("m")),
			Units::Inches(val,_) => Units::Centimeters(val * 2.54,String::from("cm")),
			// metric
			Units::Kilometers(val,_) =>  Units::Miles(val * 0.6213712,String::from("miles")),
			Units::Meters(val,_) => Units::Feet(val  / 0.3048,String::from("ft")),
			Units::Centimeters(val,_) => Units::Inches(val / 2.54,String::from("in")),
			// weight
			// imperial
			Units::Pounds(val,_) => Units::Kilograms(val * 0.4535924,String::from("kg")),
			Units::Ounces(val,_) => Units::Grams(val * 28.34952,String::from("grams")),
			// metric
			Units::Kilograms(val,_) => Units::Pounds(val / 0.4535924,String::from("lbs")),
			Units::Grams(val,_) => Units::Ounces(val / 28.34952,String::from("oz")),
		}
	}
}
const list_possible:&'static [&'static str] = &[
	"km","kms","kilometers","kilometer",
	"m","meters","meter",
	"cm","centimeters","centimeter",
	"kg","kilograms","kilogram",
	"grams","gram",
	"mile","miles",
	"ft","feet","foot",
	"inches","inch",
	"lbs","pounds","pound",
	"oz","ounces","ounce",
	];

fn parse_input(msg: &str) -> Option<Vec<Units>> {
	let mut has_convertibles = false;
	for elem in list_possible {
		if msg.to_lowercase().contains(elem) {
			has_convertibles = true;
			break;
		}
	}
	if !has_convertibles {
		return None;
	} else {
		let msg = msg.to_lowercase();
		let msg_vec: Vec<&str> = msg.split_whitespace().collect();

	}
	None
}

#[cfg(test)]
mod tests {
	
	use float_cmp::{F64Margin, approx_eq};
	use super::*;
	#[test]
	fn test_conversion() {
		if let Units::Meters(val_left,_) = Units::Meters(3.048,String::from("m")) {
			if let Units::Meters(val_right,_) = Units::Feet(10.0,String::from("ft")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Feet(val_left,_) = Units::Feet(32.80839895,String::from("ft")) {
			if let Units::Feet(val_right,_) = Units::Meters(10.0,String::from("m")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Kilograms(val_left,_) = Units::Kilograms(4.535924,String::from("kg")) {
			if let Units::Kilograms(val_right,_) = Units::Pounds(10.0,String::from("lbs")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Pounds(val_left,_) = Units::Pounds(22.04623,String::from("lbs")) {
			if let Units::Pounds(val_right,_) = Units::Kilograms(10.0,String::from("kg")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Ounces(val_left,_) = Units::Ounces(0.3527396,String::from("oz")) {
			if let Units::Ounces(val_right,_) = Units::Grams(10.0,String::from("g")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Grams(val_left,_) = Units::Grams(283.4952,String::from("g")) {
			if let Units::Grams(val_right,_) = Units::Ounces(10.0,String::from("oz")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Miles(val_left,_) = Units::Miles(10.0,String::from("miles")) {
			if let Units::Miles(val_right,_)  = Units::Kilometers(16.09344,String::from("km")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		} else {panic!("FAILED TO ASSIGN")}
		if let Units::Kilometers(val_left,_) = Units::Kilometers(10.0,String::from("km")) {
			if let Units::Kilometers(val_right,_)  = Units::Miles(6.213712,String::from("miles")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
	}

	fn test_parsing() {

	}
}

