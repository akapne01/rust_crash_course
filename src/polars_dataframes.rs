use polars::prelude::*;
use polars_lazy::{dsl::GetOutput, prelude::*};
/// References:
/// https://pola-rs.github.io/polars-book/user-guide/index.html
/// https://docs.rs/polars/latest/polars/
/// https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html
/// https://docs.rs/polars/latest/polars/series/struct.Series.html#method.iter
/// https://doc.rust-lang.org/std/iter/struct.Map.html

// Given a degree in f64, cast it to i32
fn cast_float_to_integer(ser: Series) -> Option<Series> {
    let res: Series = ser
        .f64()
        .expect("Series was not an f64 dtype")
        .into_iter()
        .map(|n| n.unwrap() as i32)
        .collect();
    Some(res)
}

// Given a degree in f64, calculate min from the fraction.
fn get_min(ser: Series) -> Option<Series> {
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
    Some(res)
}

// Given a degree in f64, calculate sec from the fraction.
// Used for testing:
// https://www.calculatorsoup.com/calculators/conversions/convert-decimal-degrees-to-degrees-minutes-seconds.php
fn get_sec(ser: Series) -> Option<Series> {
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
    Some(res)
}

fn get_sign_number(ser: Series) -> Option<Series> {
    let res: Series = ser
        .f64()
        .expect("Series was not an f64 dtype")
        .into_iter()
        .map(|n| n.unwrap() as i32 / 30)
        .collect();
    Some(res)
}

fn get_sign_degree(ser: Series) -> Option<Series> {
    let res: Series = ser
        .f64()
        .expect("Series was not an f64 dtype")
        .into_iter()
        .map(|n| n.unwrap() as i32 % 30)
        .collect();
    Some(res)
}

fn map_to_sign(ser: Series) -> Option<Series> {
    let result = ser.i32()
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
        .collect();
    Some(result)
}

fn add_house_for_planets_deg(series: Series, house_df: DataFrame) -> Option<Series> {
    let result = series
        .f64()
        .expect("Series was not an f64 dtype")
        .into_iter()
        .map(|n| map_degree_to_house(n.unwrap(), house_df.clone()))
        .collect();
    Some(result)
}

fn map_degree_to_house(given_value: f64, dfh: DataFrame) -> i32 {
    // To map to the house, looks for which house cusp is nearest in the degree.
    // Then obtains abs_deg and house number for this cusp. House number is adjusted
    // based in the abs_deg, and then returned. 

    // 1. Add col: nearest_deg
    let dfh = dfh
        .lazy()
        .with_columns([col("abs_deg")
            .map(
                move |s| Ok(add_nearest_degree_column(given_value, s)),
                GetOutput::default(),
            )
            .alias("nearest_deg")])
        .collect()
        .unwrap();

    // 2. Select Col: nearest_deg, and get a minimum degree, then extract it.
    let df1 = dfh
        .clone()
        .lazy()
        .select([col("nearest_deg").min()])
        .collect()
        .unwrap();
    println!("df1: {:?}", df1);
    let min_value = &df1[0].f64().unwrap().get(0).unwrap();
    println!("{:?}", min_value.clone());

    // 3. Filter dataframe by nearest_deg where nearest_deg equals the minimum value.
    let df3 = dfh
        .clone()
        .lazy()
        .filter(col("nearest_deg").eq(min_value.clone()))
        .collect()
        .unwrap();
    let deg = &df3[0].f64().unwrap().get(0).unwrap();
    let num = &df3[1].i32().unwrap().get(0).unwrap();

    println!("{:?}", df3);
    println!("Given Value: {}", given_value);
    println!("Deg : {deg}");
    println!("Num : {:?}", num);

    if given_value > deg.clone() {
        num.clone()
    } else {
        let res = num.clone() - 1;
        if res == 0 {
            // Returns the value of last house.
            return 5;
        }
        res
    }
}

fn nearest_value(given: f64, n: f64) -> f64 {
    (given - n).abs()
}

fn add_nearest_degree_column(given: f64, series: Series) -> Option<Series> {
    let result = series
        .f64()
        .expect("Series was not an f64 dtype")
        .into_iter()
        .map(move |d| nearest_value(given, d.unwrap()))
        .collect();
    Some(result)
}

fn add_house_column(planets: DataFrame, houses: DataFrame) -> DataFrame {
    planets
        .lazy()
        .with_columns([col("abs_deg")
            .map(
                move |abs_degrees| Ok(add_house_for_planets_deg(abs_degrees, houses.clone())),
                GetOutput::default(),
            )
            .alias("house")])
        .collect()
        .unwrap()
}

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

    let planets = df![
        "planets" => ["Sun", "Moon", "Mercury", "Venus", "Mars"],
        "abs_deg" => [226.06, 85.36, 220.12, 1.45, 90.45 ],
    ]
    .unwrap();

    let df_h = df! [
        "abs_deg"        => [ 96.3, 150.56, 270.56, 5.93, 60.97],
        "num"        => [1, 2, 3, 4, 5],
    ]
    .unwrap();

    let df = add_house_column(planets, df_h);
    println!("Resulting dataframe: {:?}", df);
}
