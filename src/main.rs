use std::io;
use std::env;
use std::process::exit;

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
		println!("Type an up to second degree polynomial equation (without double quotes):");

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
		exit(-1);
	}


	let equation_iter = equation.split_whitespace();

	let mut curr_polarity: i64 = 1;
	let mut curr_value = 1.0;
	let mut curr_power = 0;
	let mut right_side = false;
	let mut members_array: Vec<Member> = Vec::new();
	let mut member: Member;

	for entry in equation_iter {
		match entry {
			s if s.starts_with("X") => {
				if s.starts_with("X^") {
					if let Some(power_str) = s.strip_prefix("X^") {
						if let Ok(power) = power_str.parse::<u64>() {
							curr_power = power;
						} else {
							println!("Failed to parse power value: {}", power_str);
							exit(1);
						}
					} else {
						println!("Invalid format: {}", s);
						exit(2);
					}
				} else if s.len() == 1 {
					curr_power = 1;
				} else {
					println!("Invalid format: {}", s);
					exit(2);
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
				curr_value = 1.0;
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
				curr_value = 1.0;
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
				curr_value = 1.0;

			},
			"*" => {},
			_ => {
				if let Ok(parsed_value) = entry.parse::<f64>() {
					curr_value = parsed_value;
					curr_power = 0;
				} else {
					println!("Invalid input: {}", entry);
					exit(2);
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

	let mut max_power = members_array
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

	while !coefficients.is_empty() && coefficients[max_power as usize] == 0.0 {
		if max_power == 0 {
			coefficients.clear()
		} else {
			coefficients.pop();
		}
		if max_power != 0 {
			max_power = max_power - 1;
		}
	}

	// println!("Coefficients: {:?}", coefficients);

	print_polynomial(&coefficients);

	println!("Polynomial degree: {}", max_power);

	if max_power > 2 {
		println!("The polynomial degree is strictly greater than 2, I can't solve.");
		exit(3);
	}

	if max_power == 2 {
		let a = coefficients[2];
		let b = coefficients[1];
		let c = coefficients[0];

		if a == 0.0 && b == 0.0 && c == 0.0 {
			println!("Every real number is a solution");
			exit(0);
		}
		
		let delta = b * b - 4.0 * a * c;
	
		if delta < 0.0 {
			println!("Discriminant is strictly negative. No real solutions");
			let x1 = -b / (2.0 * a);
			let x2 = my_sqrt(delta * -1.0) / (2.0 * a);
			println!("Complex solutions are:\n{} + {:.6}i\n{} - {:.6}i", x1, x2, x1, x2);
			exit(0);
		} else if delta == 0.0 {
			let x = -b / (2.0 * a);
			println!("The discriminant is 0. The solution is:\n{}",
					if x == 0.0 {
						0.0
					} else {
						x
					}
			);
			exit(0);
		} else {
			let x1 = (-b + my_sqrt(delta)) / (2.0 * a);
			let x2 = (-b - my_sqrt(delta)) / (2.0 * a);
			println!("Discriminant is strictly positive, the two solutions are:\n{:.6}\n{:.6}", x1, x2);
			exit(0);
		}
	} else if max_power == 1 {
		if coefficients[0] == 0.0 && coefficients[1] == 0.0 {
			println!("Every real number is a solution");
			exit(0);
		} else {
			println!("The solution is:\n{}",
					 if (-coefficients[0] / coefficients[1]) == 0.0 {
						 0.0
					 } else {
						 -coefficients[0] / coefficients[1]
					 }
			);
			exit(0);
		}
	} else if max_power == 0 {
		if coefficients.is_empty() {
			println!("Every real number is a solution");
			exit(0);
		} else  {
			println!("The equation provided is invalid. There is no solution.");
			exit(4);
		}
	}
}

fn print_polynomial(coefficients: &[f64]) {
    // let degree = coefficients.len() - 1;

    if coefficients.is_empty() {
        println!("The polynomial is empty: 0 = 0");
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

fn my_sqrt(x: f64) -> f64 {
	if x < 0.0 {
		panic!("Cannot compute the square root of a negative number");
	}

	// Initial guess for the square root
	let mut guess = x / 2.0;

	// The precision you desire
	let epsilon = 1e-10;

	// Loop until the guess is close enough to the actual square root
	while (guess * guess - x).abs() > epsilon {
		guess = (guess + x / guess) / 2.0;
	}

	guess
}
