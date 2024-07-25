use polars::prelude::*;
use plotters::prelude::*;
use plotters::style::Color;
use colorous::*;

// Set palette to plot. Globals are declared outside all other scopes.
const TABLEAU20: [colorous::Color; 20] = [
    colorous::Color {r:31, g:119, b:180}, colorous::Color {r:174, g:199, b:232},
    colorous::Color {r:255, g:127, b:14}, colorous::Color {r:255, g:187, b:120},
    colorous::Color {r:44, g:160, b:44}, colorous::Color {r:152, g:223, b:138},
    colorous::Color {r:214, g:39, b:40}, colorous::Color {r:255, g:152, b:150},
    colorous::Color {r:148, g:103, b:189}, colorous::Color {r:197, g:176, b:213},
    colorous::Color {r:140, g:86, b:75}, colorous::Color {r:196, g:156, b:148},
    colorous::Color {r:227, g:119, b:194}, colorous::Color {r:247, g:182, b:210},
    colorous::Color {r:127, g:127, b:127}, colorous::Color {r:199, g:199, b:199},
    colorous::Color {r:188, g:189, b:34}, colorous::Color {r:219, g:219, b:141},
    colorous::Color {r:23, g:190, b:207}, colorous::Color {r:158, g:218, b:229}
];

// Set palette, colors will be cyclicly used if the number of items > 20
fn colors(vars: &[String]) -> Result<Vec<RGBColor>, &'static str> {
    if vars.len() == 0 {
        return Err("not enough variables for colormap");
    }
    let colormap: Vec<colorous::Color> = if vars.len() > 10 { TABLEAU20.to_vec() } else { CATEGORY10.to_vec() };
    let colormap: Vec<RGBColor> = colormap.iter()
        .map(|c| RGBColor(c.as_tuple().0, c.as_tuple().1, c.as_tuple().2))
        .collect();
    let mut colormap = colormap.into_iter().cycle();
    let mut colors = vec![];
    for _ in 0..vars.len() {
        colors.push(colormap.next().expect("Something wrong with colormap"));
    }
    Ok(colors)
}

// Generate plotting area and pie series
fn chart_elements(dataset: &DataFrame, x: &str, y: &str, pie_scale: f64, area_width: u32) -> Result<(u32, u32, Vec<((i32, i32), f64, Vec<f64>)>), Box<dyn std::error::Error>> {
    // Set the pie radius and zoom scale, this is the default radius when radius is not set.
    let x_min = dataset.column(x)?.min::<f64>()?.ok_or("null x_min")?;
    let x_max = dataset.column(x)?.max::<f64>()?.ok_or("null x_max")?;
    let y_min = dataset.column(y)?.min::<f64>()?.ok_or("null y_min")?;
    let y_max = dataset.column(y)?.max::<f64>()?.ok_or("null y_max")?;
    // 半径默认为 x 轴范围的 2%
    let radius = (x_max - x_min)/50.0*pie_scale;
    // x、y 轴向两边扩展一个半径长度，以免饼图出界
    let mut x_start = x_min - radius;  
    let mut x_end = x_max + radius;
    let mut y_start = y_min - radius;
    let mut y_end = y_max + radius;
    // x、y 轴向两边再各扩展 5%，作为边界留白
    let x_start = x_start - (x_end - x_start)*0.05;
    let x_end = x_end + (x_end - x_start)*0.05;
    let y_start = y_start - (y_end - y_start)*0.05;
    let y_end = y_end + (y_end - y_start)*0.05;
    // Set plotting area and scale factor (ratio) to match x, y limit to pixel.
    let ratio = area_width as f64 / (x_end-x_start);
    let area_height = (ratio * (y_end-y_start)) as u32;

    // Extract plotting values of each "ID" and push them into a vector.
    let id: Vec<u32> = dataset.column("ID")?.unique_stable()?.u32()?.into_no_null_iter().collect();
    // Generate a vector of pie
    let mut pies = vec![];
    for i in id.into_iter() {
        let data = dataset.clone().lazy()
            .filter(col("ID").eq(lit(i)))
            .collect().unwrap();
        let x: Vec<f64> = data.column(x)?.f64()?.unique()?.into_no_null_iter().collect();
        let y: Vec<f64> = data.column(y)?.f64()?.unique()?.into_no_null_iter().collect();
        // 数学坐标转化为像素坐标，由于将浮点数转化为了整数，所以可能有细微的偏差，同时补偿一个半径，以免图像出界
        let center = ((
            ((x[0]-x_start)*ratio) as i32,
            ((y[0]-y_start)*ratio) as i32  // 像素坐标和位置坐标转换
        ));
        let sizes: Vec<f64> = data.column("value")?.f64()?.into_no_null_iter().collect();
        pies.push((center, radius*ratio, sizes));
    }

    Ok((area_width, area_height, pies))
}

// Create a scatterpie plot
pub fn scatterpie(data: DataFrame,
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
              area_width: u32
            ) -> Result<(), Box<dyn std::error::Error>> {
    // Set palette, colors will be cyclicly used if the number if items > 20
    let colors = colors(&vars)?;
    // Reshape the wide dataframe to long dataframe
    let dataset = data.lazy()
        .with_columns([
            col(x).cast(DataType::Float64),
            col(y).cast(DataType::Float64),
            cols(&vars).cast(DataType::Float64),
        ])
        .with_row_index("ID", Some(0))
        .unpivot(UnpivotArgs {
            on: vars.iter().map(|x| x.into()).collect::<Vec<_>>(),
            index: vec!["ID".into(), x.into(), y.into()],
            variable_name: Some("variable".into()), value_name: Some("value".into()), streamable: false,
        })
        .collect()?;
    println!("{}", dataset.head(Some(3)));

    // Generate plotting area and pie series
    let (area_width, area_height, pies) = chart_elements(&dataset, x, y, pie_scale, area_width)?;
    
    // Plot and save image.
    let mut root = SVGBackend::new("output.svg", (area_width+200, area_height)).into_drawing_area();
    let (left, right) = root.split_horizontally(area_width);
    for i in pies.iter() {
        // Don't show labels, we will use legend later.
        let labels = vec![""; vars.len()];
        let mut pie = Pie::new(&i.0, &i.1, &i.2, &colors, &labels);
        left.draw(&pie)?;
    }
    println!("{:?}",pies[1]);
    let mut legend_ctx = ChartBuilder::on(&right)
        .caption(legend_name, ("sans-serif", font_size+5))
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)?;
    // 绘制网格及坐标
    legend_ctx
        .configure_mesh()
        .set_all_tick_mark_size(0)
        .disable_x_axis()
        .disable_y_axis()
        .disable_x_mesh()
        .disable_y_mesh()
        .axis_style(BLACK)
        .draw()?;
    // 产生数据点并绘制折线
    let mut colors = colors.iter();
    for i in vars.iter() {
        let color = colors.next().expect("Should never break");
        // 绘制 bar
        legend_ctx
            .draw_series(vec![Circle::new((0.0, 0.0), 0, &WHITE)]).unwrap()  // 点的直径设为 0 以不显示出来
            .label(format!("{}", i))  // format 使得 v 可以为 string 又可以为 &str
            // 下面 legend 的 move 是为了获取 color 的所有权，以免被下一个图例覆盖，必须使用，否则报错
            .legend(move |(x, y)| Rectangle::new([(x, y-7), (x+14, y+7)], color.filled()));
    }
    legend_ctx
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .margin(5)
        .label_font(("Calibri", font_size))  // 后面也可以加 into_font()或 into_text_style(&root)
        .draw()?;

    root.present()?;

    Ok(())
}