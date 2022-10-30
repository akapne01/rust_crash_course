use polars::prelude::*;
use polars_lazy::prelude::*;
/// References:
/// https://pola-rs.github.io/polars-book/user-guide/index.html
/// https://docs.rs/polars/latest/polars/
/// https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html
/// https://docs.rs/polars/latest/polars/series/struct.Series.html#method.iter
/// https://doc.rust-lang.org/std/iter/struct.Map.html

pub fn run() {
    println!("### Polars Dataframe ###");

    let dft = df! [
        "A"        => [156.742, 2.34, 3.78, 4.89, 5.98],
        "fruits"   => ["banana", "banana", "apple", "apple", "banana"],
        "B"        => [5, 4, 3, 2, 1],
        "cars"     => ["beetle", "audi", "beetle", "beetle", "beetle"],
        "optional" => [Some(28), Some(300), None, Some(2), Some(-30)],
    ];
    let dft = match dft {
        Ok(dft) => dft,
        Err(_) => DataFrame::default(),
    };

    println!("{:?}", dft);

    // Given a degree in f64, cast it to i32
    fn cast_float_to_integer(ser: Series) -> Series {
        let res: Series = ser
            .f64()
            .expect("Series was not an f64 dtype")
            .into_iter()
            .map(|n| n.unwrap() as i32)
            .collect();
        res
    }

    // Given a degree in f64, calculate min from the fraction.
    fn get_min(ser: Series) -> Series {
        let res: Series = ser
            .f64()
            .expect("Series was not an f64 dtype")
            .into_iter()
            .map(|n| {
                let deg_f = n.unwrap();
                let deg_i = deg_f as i32;
                ((deg_f - deg_i as f64) * 60.0) as i32
            })
            .collect();
        res
    }

    // Given a degree in f64, calculate sec from the fraction.
    // Used for testing:
    // https://www.calculatorsoup.com/calculators/conversions/convert-decimal-degrees-to-degrees-minutes-seconds.php
    fn get_sec(ser: Series) -> Series {
        let res: Series = ser
            .f64()
            .expect("Series was not an f64 dtype")
            .into_iter()
            .map(|n| {
                let deg_f = n.unwrap();
                println!("\n1. def_f: {deg_f}"); // 156.742 Example
                let deg_i = deg_f as i32;
                println!("2. def_i: {deg_i}"); // 156 Ok
                let rem1 = deg_f - (deg_i as f64);
                println!("3. rem1: {rem1}"); // 0.7419999999999902 OK
                let min_f = rem1 * 60.0;
                println!("4. min_f: {}", min_f); // 44.51999999999941 OK
                let min = min_f as i32;
                println!("5. min: {}", min); // 44 OK
                let rem2 = min_f - min as f64;
                println!("6. rem2: {}", rem2); // 0.5199999999994134 OK
                let sec = rem2 * 60.0;
                println!("7. sec: {sec}"); // 31.199999999964803 => 31.2 OK
                sec
            })
            .collect();
        res
    }

    fn get_sign_number(ser: Series) -> Series {
        let res: Series = ser
            .f64()
            .expect("Series was not an f64 dtype")
            .into_iter()
            .map(|n| n.unwrap() as i32 / 30)
            .collect();
        res
    }

    fn get_sign_degree(ser: Series) -> Series {
        let res: Series = ser
            .f64()
            .expect("Series was not an f64 dtype")
            .into_iter()
            .map(|n| n.unwrap() as i32 % 30)
            .collect();
        res
    }

    fn map_to_sign(ser: Series) -> Series {
        ser.i32()
            .expect("Series was not an i32 dtype")
            .into_iter()
            .map(|n| {
                let sign_num = n.unwrap();
                match sign_num {
                    0 => "Aries",
                    1 => "Taurus",
                    2 => "Gemini",
                    3 => "Cancer",
                    4 => "Leo",
                    5 => "Virgo",
                    6 => "Libra",
                    7 => "Scorpio",
                    8 => "Sagittarius",
                    9 => "Capricorn",
                    10 => "Aquarius",
                    11 => "Pisces",
                    _ => "Sign number not defined, can't match.",
                }
            })
            .collect()
    }

    // Add a column that casts column A to integer values
    let df = dft.lazy().with_columns([
        col("A"),
        col("A")
            .map(|n| Ok(cast_float_to_integer(n)), GetOutput::default())
            .alias("A_deg"),
        col("A")
            .map(|s| Ok(get_min(s)), GetOutput::default())
            .alias("A_min"),
        col("A")
            .map(|s| Ok(get_sec(s)), GetOutput::default())
            .alias("A_sec"),
        col("A")
            .map(|s| Ok(get_sign_degree(s)), GetOutput::default())
            .alias("A_sign_degree"),
        col("A")
            .map(|s| Ok(get_sign_number(s)), GetOutput::default())
            .alias("sign_number"),
    ]);

    let df = df
        .with_columns([
            all(),
            col("sign_number")
                .map(|s| Ok(map_to_sign(s)), GetOutput::default())
                .alias("sign"),
        ])
        .collect()
        .unwrap();
    println!("{:?}", df);
}
