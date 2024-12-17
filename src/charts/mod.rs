use charming::{component::{Axis}, datatype::{Dataset, Transform}, element::{
    AxisType, SplitArea, SplitLine,
}, series::{Boxplot}, Chart, HtmlRenderer};
use charming::component::Legend;

pub fn get_chart_multiple_dataset_5(data: Vec<Vec<f64>>, countries: Vec<&str>, file_name: &str) {
    let filtered_data: Vec<Vec<f64>> = data.into_iter()
        .map(|inner_vec| inner_vec.into_iter().filter(|&x| x != 0.0).collect())
        .collect();

    let data0 = vec![filtered_data[0].clone()];
    let data1 = vec![filtered_data[1].clone()];
    let data2 = vec![filtered_data[2].clone()];
    let data3 = vec![filtered_data[3].clone()];
    let data4 = vec![filtered_data[4].clone()];

    let ds = Dataset::new()
        .source(data0)
        .source(data1)
        .source(data2)
        .source(data3)
        .source(data4)
        .transform(
            Transform::new()
                .from_dataset_index(0)
                .transform(r#"{"type": "boxplot"}"#),
        )
        .transform(
            Transform::new()
                .from_dataset_index(1)
                .transform(r#"{"type": "boxplot"}"#),
        )
        .transform(
            Transform::new()
                .from_dataset_index(2)
                .transform(r#"{"type": "boxplot"}"#),
        )
        .transform(
            Transform::new()
                .from_dataset_index(3)
                .transform(r#"{"type": "boxplot"}"#),
        )
        .transform(
            Transform::new()
                .from_dataset_index(4)
                .transform(r#"{"type": "boxplot"}"#),
        );

    let chart = Chart::new()
        .dataset(ds)
        .legend(Legend::new().top("4%"))
        .x_axis(Axis::new()
                    .type_(AxisType::Category)
                    .boundary_gap(true)
                    .name_gap(30)
                    .split_area(SplitArea::new().show(false))
                    .split_line(SplitLine::new().show(false)))
        .y_axis(Axis::new()
                    .type_(AxisType::Value)
                    .name("sentiment value")
                    .min(-1)
                    .max(1)
                    .interval(0.1)
                    .split_area(SplitArea::new().show(true)))
        .series(Boxplot::new().name(countries[0]).dataset_index(5))
        .series(Boxplot::new().name(countries[1]).dataset_index(6))
        .series(Boxplot::new().name(countries[2]).dataset_index(7))
        .series(Boxplot::new().name(countries[3]).dataset_index(8))
        .series(Boxplot::new().name(countries[4]).dataset_index(9));

    let mut renderer = HtmlRenderer::new("boxplot", 1000, 800);
    renderer.save(&chart, file_name).expect("Failed to save chart");
}

pub fn get_chart_single_dataset(data: Vec<f64>, country: &str, file_name: &str) {
    let filtered_data: Vec<f64> = data.into_iter().filter(|&x| x != 0.0).collect();

    let data0 = vec![filtered_data];

    let ds = Dataset::new()
        .source(data0)
        .transform(
            Transform::new()
                .from_dataset_index(0)
                .transform(r#"{"type": "boxplot"}"#),
        );

    let chart = Chart::new()
        .dataset(ds)
        .legend(Legend::new().top("4%"))
        .x_axis(Axis::new()
                    .type_(AxisType::Category)
                    .boundary_gap(true)
                    .name_gap(30)
                    .split_area(SplitArea::new().show(false))
                    .split_line(SplitLine::new().show(false)))
        .y_axis(Axis::new()
                    .type_(AxisType::Value)
                    .name("sentiment value")
                    .min(-1)
                    .max(1)
                    .interval(0.1)
                    .split_area(SplitArea::new().show(true)))
        .series(Boxplot::new().name(country).dataset_index(1));

    let mut renderer = HtmlRenderer::new("boxplot", 1000, 800);
    renderer.save(&chart, file_name).expect("Failed to save chart");
}