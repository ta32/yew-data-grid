use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GridPaginationBarProps {
    pub page: i32,
    pub page_size: i32,
    pub total_rows: usize
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub props: GridPaginationBarProps
}

#[function_component(GridPaginationBar)]
pub fn grid_pagination_bar(i: &Props) -> Html {
    let total_rows = i.props.total_rows;
    let summary = format!("1 - {total_rows} of {total_rows}");
    const BAR_HEIGHT: i32 = 48;
    let style = format!("min-height: {BAR_HEIGHT}px; display: flex; flex-direction: row;");
    html! {
        <div class="yew-grid-pagination-bar" style={style}>
            <div class="yew-grid-pagination-bar-controls">
                <button class="yew-grid-pagination-bar-control-button">{"<"}</button>
                <button class="yew-grid-pagination-bar-control-button">{"1"}</button>
                <button class="yew-grid-pagination-bar-control-button">{">"}</button>
            </div>
            <div class="yew-grid-pagination-bar-summary">{summary}</div>
        </div>
    }
}