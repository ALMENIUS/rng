use std::fs::File;

use std::io::{stdin, Write};

fn main() {
    // Δήλωση μεταβλητών και αρχικοποίησή τους
    let (mut num, mut m, mut count): (i64, i64, usize);
    let (mut z, mut sum, mut avg, mut sum2, mut variance, mut prev_z): (f64, f64,f64,f64,f64,f64,);
    let mut runs: [usize; 10] = [0; 10];
    let mut areas: [usize; 10] = [0; 10];

    m = 2;
    m = m.pow(32);
    sum = 0.0;
    sum2 = 0.0;
    count = 0;

    let mut output = File::create("results2.txt").expect("Error encountered while creating file!");
    // Input request στον χρήστη
    let mut input_string = String::new();
    println!("Input the number: ");
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    // Παραγωγή του τυχαίου αριθμού με βάση την τιμή seed
    num = input_string.parse().unwrap_or(0i64);
    num = rng(num, m);
    // Υπολογισμός των δευτερευόντων τιμών που ζητούνται
    z = num as f64 / m as f64;
    prev_z = z;
    sum += z;
    sum2 += z * z;
    avg = sum as f64; //Υπολογισμός μέσης τιμής
    variance = (sum2 - (sum * sum)).sqrt(); //Υπολογισμός τυπικής απόκλισης
                                            // Υπολογισμός του τεστ περιοχών
    match (z * 100.0) as i32 {
        0..=09 => areas[0] += 1,
        10..=19 => areas[1] += 1,
        20..=29 => areas[2] += 1,
        30..=39 => areas[3] += 1,
        40..=49 => areas[4] += 1,
        50..=59 => areas[5] += 1,
        60..=69 => areas[6] += 1,
        70..=79 => areas[7] += 1,
        80..=89 => areas[8] += 1,
        90..=100 => areas[9] += 1,
        _ => return (),
    };

    count += 1;
    write!(output, "avg: {}\nStd.Variation: {}\n\n", avg, variance).ok();
    for i in 2..=1000000i64 {
        num = rng(num, m);

        z = num as f64 / m as f64;
        //Υπολογισμός τεστ runs
        if prev_z < 0.5 {
            if z < 0.5 {
                if count == 9 {
                    count = 9;
                } else {
                    count += 1;
                }
            } else {
                runs[count] += 1;
                count = 0;
            }
        } else {
            if z >= 0.5 {
                if count == 9 {
                    count = 9;
                } else {
                    count += 1;
                }
            } else {
                runs[count] += 1;
                count = 0;
            }
        } //Τέλος υπολογισμού runs
        match (z * 100.0) as i32 {
            0..=09 => areas[0] += 1,
            10..=19 => areas[1] += 1,
            20..=29 => areas[2] += 1,
            30..=39 => areas[3] += 1,
            40..=49 => areas[4] += 1,
            50..=59 => areas[5] += 1,
            60..=69 => areas[6] += 1,
            70..=79 => areas[7] += 1,
            80..=89 => areas[8] += 1,
            90..=100 => areas[9] += 1,
            _ => return (),
        };
        prev_z = z;

        sum += z;
        sum2 += z * z;
        avg = sum / i as f64;
        variance = (i as f64 * sum2 - (sum * sum)).sqrt() / (i as f64);

        write!(output, "avg: {}\nStd.Variation: {}\n\n", avg, variance).ok();
    }
    write!(
        output,
        "Area array: {:?}\n Runs array: {:?}\n\n",
        areas, runs
    )
    .ok();
}
//Συνάρτηση για την παραγωγή του ψευδοτυχαίου αριθμού
fn rng(n: i64, m: i64) -> i64 {
    let y: i64;

    y = (1664525 * n + 1013904223) % m;

    return y;
}
