#![allow(dead_code)]
#![allow(unused)]
use polars::{lazy::dsl::lit, prelude::*};

fn print_whole_frame(s: &Series) {
    todo!()
}

// We have four features for iris, so 4 weights
fn initalize_weights(weights: Series) -> Series{
    todo!()
}

fn initalize_bias(){
    todo!()
}


fn compute_output_value(){
    todo!()
}

fn update_weights(){
    todo!()
}

fn update_bias(){
    todo!()
}

fn dot_produt(x_left: Series, x_right: Series) -> PolarsResult<f64> {
    
    todo!() 
}

fn sigma(z: f64) -> u8 {
    todo!()
}


fn main() {


    let iris_dataset = CsvReader::from_path("data/iris.csv")
    .unwrap()
    .finish()
    .unwrap();

    // "sepal.length","sepal.width","petal.length","petal.width","variety"
    // Take sepal.length and petal.length only
    // restrict to setosa and versicolor

    let feature_matrix = iris_dataset
        .clone()
        .lazy()
        .select([cols(["sepal.length", "petal.length"])])
        .collect()
        .unwrap();

    let mut x_train = feature_matrix.clone().slice(0, 100);
    let mut x_test = feature_matrix.clone().slice(100, iris_dataset.shape().0);

    let mut x_train_sepal_petal_length = x_train.clone().select(["sepal.length", "petal.length"]);
    let mut y_train = 
        iris_dataset
        .clone()
        .lazy()
        .slice(0, 100)
        .with_column(
            when(col("variety").eq(lit("Versicolor"))).then(lit("kubus")).otherwise(lit("nic")).alias("haha")
        )
        .collect()
        .unwrap();
    
    let check_encoding_series = 
    y_train
    .clone()
    .select(["haha"])
    .unwrap()
    .select_series(["haha"])
    .unwrap();

    println!("{}", y_train);
    println!("{}", check_encoding_series[0].unique().unwrap());

    // let mut weights = vec![0.12, 0.34];

    // let eta: f64 = 0.01;
    // let number_of_iterations: u8 = 80;
    // let seed: u8 = 42;
    // let bias = 0.0;
    
    // .select([col("variety")])
    // .collect()
    // .unwrap();

    // let weights = Series::new("weights", [0.0, 0.0, 0.0, 0.0]);


}
