use std::fmt::Result;
use float_cmp::{self, F64Margin, approx_eq};


#[derive(Debug,PartialEq)]
enum Units {
	Mile(f64,String),
	Feet(f64, String),
	Inches(f64, String),
	Kilometer(f64,String),
	Meters(f64, String),
	Centimetres(f64, String),
	Pound(f64,String),
	Ounce(f64,String),
	Kilogram(f64, String),
	Gram(f64, String),	
}

impl Units {
	fn convert(&self) -> Units {
		match self {
			// length
			// imperial
			Units::Mile(val,_) => Units::Kilometer(val * 1.609344,String::from("km")),
			Units::Feet(val,_) => Units::Meters(val * 0.3048,String::from("m")),
			Units::Inches(val,_) => Units::Centimetres(val * 2.54,String::from("cm")),
			// metric
			Units::Kilometer(val,_) =>  Units::Mile(val * 0.6213712,String::from("miles")),
			Units::Meters(val,_) => Units::Feet(val  / 0.3048,String::from("ft")),
			Units::Centimetres(val,_) => Units::Inches(val / 2.54,String::from("in")),
			// weight
			// imperial
			Units::Pound(val,_) => Units::Kilogram(val * 0.4535924,String::from("kg")),
			Units::Ounce(val,_) => Units::Gram(val * 28.34952,String::from("grams")),
			// metric
			Units::Kilogram(val,_) => Units::Pound(val / 0.4535924,String::from("lbs")),
			Units::Gram(val,_) => Units::Ounce(val / 28.34952,String::from("oz")),
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
		if let Units::Kilogram(val_left,_) = Units::Kilogram(4.535924,String::from("kg")) {
			if let Units::Kilogram(val_right,_) = Units::Pound(10.0,String::from("lbs")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Pound(val_left,_) = Units::Pound(22.04623,String::from("lbs")) {
			if let Units::Pound(val_right,_) = Units::Kilogram(10.0,String::from("kg")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Ounce(val_left,_) = Units::Ounce(0.3527396,String::from("oz")) {
			if let Units::Ounce(val_right,_) = Units::Gram(10.0,String::from("g")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Gram(val_left,_) = Units::Gram(283.4952,String::from("g")) {
			if let Units::Gram(val_right,_) = Units::Ounce(10.0,String::from("oz")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
		if let Units::Mile(val_left,_) = Units::Mile(10.0,String::from("miles")) {
			if let Units::Mile(val_right,_)  = Units::Kilometer(16.09344,String::from("km")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		} else {panic!("FAILED TO ASSIGN")}
		if let Units::Kilometer(val_left,_) = Units::Kilometer(10.0,String::from("km")) {
			if let Units::Kilometer(val_right,_)  = Units::Mile(6.213712,String::from("miles")).convert() {
				assert!(approx_eq!(f64,val_left,val_right,F64Margin { epsilon: 0.001, ulps: 2 }),"\nleft != right\n{} != {}",val_left,val_right);
			}else {panic!("FAILED TO ASSIGN")}
		}else {panic!("FAILED TO ASSIGN")}
	}

	fn test_parsing() {

	}
}

