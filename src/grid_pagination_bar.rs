use yew::prelude::*;
use crate::hooks::pagination::Pagination;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub pagination: UseStateHandle<Pagination>
}

#[function_component(GridPaginationBar)]
pub fn grid_pagination_bar(props: &Props) -> Html {
    let total_rows = props.pagination.total_rows;
    let summary = format!("1 - {total_rows} of {total_rows}");
    const BAR_HEIGHT: i32 = 48;
    let style = format!("min-height: {BAR_HEIGHT}px; display: flex; flex-direction: row;");

    // TODO extract this logic to reduce the duplication
    let inc_page = {
        let state = props.pagination.clone();
        Callback::from(move |_| {
            let mut new_state = Pagination::new(state.total_rows, state.page_size);
            let new_page_num = if state.page + 1 > state.number_pages {
                state.number_pages
            } else {
                state.page + 1
            };
            new_state.page = new_page_num;
            state.set(new_state);
        })
    };

    let dec_page = {
        let state = props.pagination.clone();
        Callback::from(move |_| {
            let mut new_state = Pagination::new(state.total_rows, state.page_size);
            let new_page_num = if state.page - 1 < 1 {
                1
            } else {
                state.page - 1
            };
            new_state.page = new_page_num;
            state.set(new_state);
        })
    };

    let page_buttons = {
        let num_pages = props.pagination.number_pages;
        (1..num_pages).map(|i| {
            let page = i.to_string();
            html! {
                <button class="yew-grid-pagination-bar-control-button">{page}</button>
            }
        }).collect::<Html>()
    };
    html! {
        <div class="yew-grid-pagination-bar" style={style}>
            <div class="yew-grid-pagination-bar-controls">
                <button onclick={dec_page} class="yew-grid-pagination-bar-control-button">{"<"}</button>
                {{page_buttons}}
                <button onclick={inc_page} class="yew-grid-pagination-bar-control-button">{">"}</button>
            </div>
            <span class="yew-grid-pagination-bar-summary">{summary}</span>
        </div>
    }
}