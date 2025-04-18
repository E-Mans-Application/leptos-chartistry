use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .bar(|data: &MyData| data.y1)
        .bar(|data: &MyData| if data.x < 6.0 { data.y2 } else { -data.y2 })
        .with_y_range(-10.0, 10.0);
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            left=TickLabels::aligned_floats()
            inner=[
                AxisMarker::left_edge().into_inner(),
                AxisMarker::bottom_edge().into_inner(),
                YGridLine::default().into_inner(),
            ]
        />
    }
}
