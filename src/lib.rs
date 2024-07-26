//! pypies
//! 
//! `pypies` can create scatterpie plots, especially useful for plotting pies
//!  on a map.
mod plot;

use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;

/// Create a scatter pie plot.
///
/// # Arguments
/// - `data`: data
/// - `x`: column name of x-axis data
/// - `y`: column name of y-axis data
/// - `vars`: column names of the piechart data
/// - `pie_scale`: amount to scale the pie size if there is no radius mapping exists
/// - `font_size`: legend font size
/// - `sorted_by_radius`: whether plotting large pie first
/// - `legend_name`: name of fill legend
/// - `long_format`: logical whether use long format of input data
/// - `label_radius`: numeric the radius of label position (relative the radius
/// of pie), default is NULL, when it is provided, the ratio or value label
/// will be displayed.
/// - `label_show_ratio`: logical only work when `label_radius` is not NULL,
// default is TRUE, meaning the ratio of label will be displayed.
/// - `label_threshold`: numeric the threshold is to control display the label,
/// the ratio of slice pie smaller than the threshold will not be displayed.
/// default is 0.
/// - `donut_radius`: numeric the radius of donut chart (relative the radius of
/// circle), default is NULL. It should be between 0 and 1, if it is provided,
/// the donut chart will be displayed instead of pie chart.
/// - `bg_circle_radius`: numeric the radius of background circle, default is
/// FALSE, we suggest setting it to between 1 and 1.5.
///
/// # Returns
/// - A i32
/// 
/// # Examples:
/// ```
/// import polars as pl
/// import pypies
/// df = pl.read_csv("datasets/iris.csv")
/// pypies.scatterpie(df, "Sepal.Length", "Sepal.Width", ["Petal.Length","Petal.Width"])
/// ```
#[pyfunction]
// Keyword arguments with default value must be at the end!
#[pyo3(signature=(data, x, y, vars, pie_scale=1.0, font_size=25, sorted_by_radius=false,
        legend_name="type", long_format=false, label_radius=None,
        label_show_ratio=true, label_threshold=0.0, donut_radius=None,
        bg_circle_radius=None, area_width=600, file_name=String::from("scatterpie")))]

        pub fn scatterpie(data: PyDataFrame,
              x: &str,
              y: &str,
              vars: Vec<String>,  // Cannot be Vec<&str>
              //radius: f64,
              pie_scale: f64,
              font_size: u32,
              sorted_by_radius: bool,
              legend_name: &str,
              long_format: bool,
              label_radius: Option<f64>,
              label_show_ratio: bool,
              label_threshold: f64,
              donut_radius: Option<f64>,
              bg_circle_radius: Option<f64>,
              area_width: u32,
              file_name: String
            ) -> PyResult<()> {
    // Convert python polars dataframe to rust polars dataframe
    let data: DataFrame = data.into();
    plot::scatterpie(data, x, y, vars, pie_scale, font_size, sorted_by_radius,
        legend_name, long_format, label_radius, label_show_ratio,
        label_threshold, donut_radius, bg_circle_radius, area_width,
        file_name)
        .unwrap();

    Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pypies(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scatterpie, m)?)?;

    Ok(())
}