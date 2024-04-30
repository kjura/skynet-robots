use polars::prelude::*;

fn print_whole_frame(s: &Series) {
    println!("{}", &s)
}

// We have four features for iris, so 4 weights
fn initalize_weights(weights: Series) -> Series{
    weights
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
    
    (x_left * x_right).sum() 
}

fn sigma(z: f64) -> u8 {
    if z >= 0.0 {
        1
    }
    else {
        0
    }
}


fn main() {


    let iris_dataset = CsvReader::from_path("data/iris.csv")
    .unwrap()
    .finish()
    .unwrap();


    // "sepal.length","sepal.width","petal.length","petal.width","variety"
    // let col_variety: DataFrame = iris_dataset.clone().lazy().select([col("variety")]).collect().unwrap();
    let variety = &iris_dataset["variety"];
    let sepal_length = &iris_dataset["sepal.length"];
    let sepal_width = &iris_dataset["sepal.width"];
    let petal_length = &iris_dataset["petal.length"];
    let petal_width = &iris_dataset["petal.width"];
    
    // print_whole_frame(variety);
    // print_whole_frame(sepal_length);
    // print_whole_frame(sepal_width);
    // print_whole_frame(petal_length);
    // print_whole_frame(petal_width);

    let s = Series::new("a", [1 , 2, 3]);
    let s_two = Series::new("b", [2 , 4, 6]);
    let dot = dot_produt(s, s_two).unwrap();
    println!("{}", dot);
    // let out_mul = &s * &s;
    // println!("{}", out_mul);
    // println!("Finished!");

}
