use clap::Parser;

#[derive(Parser)]
#[command(arg_required_else_help = false)]
struct Args {
    filename: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let filename = args.filename;
    let mut first_calories = 0;
    let mut second_calories = 0;
    let mut third_calories = 0;
    let mut current_calories = 0;
    for line in std::fs::read_to_string(&filename).unwrap().lines() {
        if line.is_empty() {
            if current_calories > first_calories {
                third_calories = second_calories;
                second_calories = first_calories;
                first_calories = current_calories;
            } else if current_calories > second_calories {
                third_calories = second_calories;
                second_calories = current_calories;
            } else if current_calories > third_calories {
                third_calories = current_calories;
            }
            current_calories = 0;
            continue
        }
        let kcals = line.parse::<i32>().unwrap();
        current_calories += kcals;
    }
    // check if leftovers from last line
    if current_calories > 0 {
        if current_calories > first_calories {
            third_calories = second_calories;
            second_calories = first_calories;
            first_calories = current_calories;
        } else if current_calories > second_calories {
            third_calories = second_calories;
            second_calories = current_calories;
        } else if current_calories > third_calories {
            third_calories = current_calories;
        }
    }


    println!("[1] {}", first_calories);
    println!("[2] {}", second_calories);
    println!("[3] {}", third_calories);
    println!("Combined: {}", first_calories + second_calories + third_calories);
}
