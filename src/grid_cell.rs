use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    key: String,
    col_index: usize,
    row_index: usize,
    field: String,
    value: String,
    height: i32,
    width: i32
}

#[function_component(DataCell)]
pub fn grid_cell(props: &Props) -> Html {
    // todo styles for min width, max width, etc
    html! (
        <div class="yew-grid-cell">
            <div class="yew-grid-cell-content">{&props.value}</div>
        </div>
    )
}