use std::io;
use std::env;

#[derive(Debug, Clone)]
struct Member {
	polarity: i64,
	value: f64,
	power: u64,
	right_side: bool
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let equation: String;

	if args.len() == 2 {
		let trimmed_equation = args[1].clone().trim().to_string();
		equation = trimmed_equation;
	} else if args.len() == 1 {
		println!("Type an up to second degree polynomial equation:");

		let mut input_equation = String::new();

		io::stdin()
			.read_line(&mut input_equation)
			.expect("Failed to read line");

		// input will come as "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
		// 42 * X^0 = 42 * X^0

		let trimmed_equation = input_equation.trim().to_string();
		equation = trimmed_equation;

	} else {
		println!("Usage: Use no arguments for interactive prompt or type up to second degree polynomial equation as a command line argument.");
		return ;
	}


	let equation_iter = equation.split_whitespace();

	let mut curr_polarity: i64 = 1;
	let mut curr_value = 0.0;
	let mut curr_power = 0;
	let mut right_side = false;
	let mut members_array: Vec<Member> = Vec::new();
	let mut member: Member;

	for entry in equation_iter {
		match entry {
			s if s.starts_with("X^") => {
				if let Some(power_str) = s.strip_prefix("X^") {
					if let Ok(power) = power_str.parse::<u64>() {
						curr_power = power;
					} else {
						println!("Failed to parse power value: {}", power_str);
					}
				} else {
					println!("Invalid format: {}", s);
				}
			},
			"+" => {
				member = Member {
					polarity: curr_polarity,
					value: curr_value,
					power: curr_power,
					right_side
				};
				members_array.push(member.clone());
				curr_polarity = 1;
			},
			"-" => {
				member = Member {
					polarity: curr_polarity,
					value: curr_value,
					power: curr_power,
					right_side
				};
				members_array.push(member.clone());
				curr_polarity = -1;
			},
			"=" => {
				member = Member {
					polarity: curr_polarity,
					value: curr_value,
					power: curr_power,
					right_side
				};
				members_array.push(member.clone());
				right_side = true;
				curr_polarity = 1;

			},
			"*" => {},
			_ => {
				if let Ok(parsed_value) = entry.parse::<f64>() {
					curr_value = parsed_value;
					curr_power = 0;
				} else {
					println!("Invalid input: {}", entry);
				}
			}
		}
	}
	member = Member {
		polarity: curr_polarity,
		value: curr_value,
		power: curr_power,
		right_side
	};

	members_array.push(member.clone());

	for group in &members_array {

		println!("Member is {:?}", group);
	}

	let max_power = members_array
						.iter()
						.map(|member| member.power)
						.max()
						.unwrap_or(0);

	let mut coefficients: Vec<f64> = vec![0.0; (max_power + 1) as usize];

	for ele in &members_array {
		let index = ele.power as usize;
		let value = ele.value * ele.polarity as f64;

		if ele.right_side {
			coefficients[index] -= value;
		} else {
			coefficients[index] += value;
		}
	}

	println!("Coefficients: {:?}", coefficients);

	print_polynomial(&coefficients);

	println!("Polynomial degree: {}", max_power);
	
	if max_power > 2 {
		println!("The polynomial degree is strictly greater than 2, I can't solve.");
		return;
	}

	if max_power == 2 {
		let a = coefficients[2];
		let b = coefficients[1];
		let c = coefficients[0];

		if a == 0.0 && b == 0.0 && c == 0.0 {
			println!("Every real number is a solution");
			return;
		}
		
		let delta = b * b - 4.0 * a * c;
	
		if delta < 0.0 {
			println!("Discriminant is strictly negative. No real solutions");
		} else if delta == 0.0 {
			let x = -b / (2.0 * a);
			println!("The solution is:\n{}", x);
		} else {
			let x1 = (-b + delta.sqrt()) / (2.0 * a);
			let x2 = (-b - delta.sqrt()) / (2.0 * a);
			println!("Discriminant is strictly positive, the two solutions are:\n{:.6}\n{:.6}", x1, x2);
		}
	} else if max_power == 1 {
		if coefficients[0] == 0.0 && coefficients[1] == 0.0 {
			println!("Every real number is a solution");
		} else {
			println!("The solution is:\n{}", -coefficients[0] / coefficients[1]);
		}
	} else if max_power == 0 {
		if coefficients[0] == 0.0 {
			println!("Every real number is a solution");
		} else  {
			println!("The equation provided is invalid");
		}
	}



}

fn print_polynomial(coefficients: &[f64]) {
    // let degree = coefficients.len() - 1;

    if coefficients.is_empty() {
        println!("The polynomial is empty.");
        return;
    }

    print!("Reduced form: ");
    
    for (i, &coef) in coefficients.iter().enumerate() {
        let power = i;

        if coef >= 0.0 && i > 0 {
            print!("+ ");
        } else if coef < 0.0 {
			print!("- ");
		}

		print!("{} * X^{}", coef.abs(), power);

        print!(" ");
    }

    println!("= 0");
}
