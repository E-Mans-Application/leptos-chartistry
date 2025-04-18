mod data;
mod range;

pub use range::Range;

use crate::{
    series::{use_y::RenderUseY, UseY},
    state::State,
    Series, Tick,
};
use data::Data;
use leptos::prelude::*;

#[derive(Clone)]
#[non_exhaustive]
pub struct UseData<X: Tick, Y: Tick> {
    data: Memo<Data<X, Y>>,
    pub len: Memo<usize>,
    pub series: Memo<Vec<UseY>>,
    pub includes_bars: Memo<bool>,
    pub range_x: Memo<Range<X>>,
    pub range_y: Memo<Range<Y>>,
}

impl<X: Tick, Y: Tick> UseData<X, Y> {
    pub fn new<T: Send + Sync + 'static>(
        series: Series<T, X, Y>,
        data: Signal<Vec<T>>,
    ) -> UseData<X, Y> {
        let lines = series.to_use_lines();

        // Data values
        let data = {
            let lines = lines.clone();
            Memo::new(move |_| {
                let get_x = series.get_x.clone();
                data.with(|data| {
                    Data::new(
                        get_x,
                        lines
                            .clone()
                            .into_iter()
                            .map(|(use_y, get_y)| (use_y.id, get_y))
                            .collect(),
                        data,
                    )
                })
            })
        };

        // Range signals
        let range_x: Memo<Range<X>> = Memo::new(move |_| {
            data.with(|data| data.range_x())
                .replace(series.min_x.get(), series.max_x.get())
        });
        let range_y: Memo<Range<Y>> = Memo::new(move |_| {
            data.with(|data| data.range_y())
                .replace(series.min_y.get(), series.max_y.get())
        });

        // Sort series by name
        let series = {
            let (lines, _): (Vec<_>, Vec<_>) = lines.into_iter().unzip();
            Memo::new(move |_| {
                let mut lines = lines.clone();
                lines.sort_by_key(|line| line.name.get());
                lines
            })
        };
        let includes_bars =
            Memo::new(move |_| series.get().iter().any(|use_y| use_y.bar().is_some()));

        UseData {
            data,
            len: Memo::new(move |_| data.with(|data| data.len())),
            series,
            includes_bars,
            range_x,
            range_y,
        }
    }
}

impl<X: Tick, Y: Tick> UseData<X, Y> {
    pub fn nearest_data_x(&self, pos_x: Memo<f64>) -> Memo<Option<X>> {
        let data = self.data;
        Memo::new(move |_| data.with(|data| data.nearest_data_x(pos_x.get())))
    }

    pub fn nearest_position_x(&self, pos_x: Memo<f64>) -> Memo<Option<f64>> {
        let data = self.data;
        Memo::new(move |_| data.with(|data| data.nearest_position_x(pos_x.get())))
    }

    // TODO: this can never be None
    pub fn nearest_data_y(&self, pos_x: Memo<f64>) -> Memo<Vec<(UseY, Option<Y>)>> {
        let series = self.series;
        let data = self.data;
        Memo::new(move |_| {
            let y_values = data.with(|data| data.nearest_data_y(pos_x.get()));
            series
                .get()
                .into_iter()
                .map(|line| {
                    let y_value = y_values.get(&line.id).cloned();
                    (line, y_value)
                })
                .collect::<Vec<_>>()
        })
    }
}

#[component]
pub fn RenderData<X: Tick, Y: Tick>(state: State<X, Y>) -> impl IntoView {
    let data = state.pre.data.clone();
    let mk_svg_coords = move |id| {
        Signal::derive(move || {
            let proj = state.projection.get();
            let range_x = data.range_x.get();
            let range_y = data.range_y.get();
            data.data.with(|data| {
                data.series_positions(id)
                    .iter()
                    .copied()
                    .map(|pos| {
                        let x = if range_x.contains_pos(&pos.0) {
                            pos.0
                        } else {
                            f64::NAN
                        };
                        let y = if range_y.contains_pos(&pos.1) {
                            pos.1
                        } else {
                            f64::NAN
                        };
                        (x, y)
                    })
                    .map(|(x, y)| proj.position_to_svg(x, y))
                    .collect::<Vec<_>>()
            })
        })
    };

    view! {
        <g class="_chartistry_series">
            <For
                each=move || data.series.get()
                key=|use_y| use_y.id
                let:use_y>
                <RenderUseY use_y=use_y.clone() state=state.clone() positions=mk_svg_coords(use_y.id) />
            </For>
        </g>
    }.into_any()
}
