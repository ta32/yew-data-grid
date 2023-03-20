use yew::prelude::*;
use crate::hooks::pagination::Pagination;

#[derive(Clone, Copy, PartialEq, Debug)]
enum PageRange {
    Lower,
    Mid,
    Upper
}

fn get_page_range(i: i32, num_pages: i32, max_num_pages: i32) -> PageRange {
    if i < max_num_pages {
        PageRange::Lower
    } else if i > num_pages - max_num_pages {
        PageRange::Upper
    } else {
        PageRange::Mid
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub max_pages_to_show: i32,
    pub pagination: UseStateHandle<Pagination>
}

#[function_component(GridPaginationBar)]
pub fn grid_pagination_bar(props: &Props) -> Html {
    let total_rows = props.pagination.total_rows;
    let summary = format!("1 - {total_rows} of {total_rows}");
    const BAR_HEIGHT: i32 = 48;
    let style = format!("min-height: {BAR_HEIGHT}px; display: flex; flex-direction: row;");

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

    let jump_page = {
        let state = props.pagination.clone();
        Callback::from(move |page_num| {
            let mut new_state = Pagination::new(state.total_rows, state.page_size);
            let new_page_num = if page_num < 1 {
                1
            } else if page_num > state.number_pages {
                state.number_pages
            } else {
                page_num
            };
            new_state.page = new_page_num;
            state.set(new_state);
        })
    };

    let page_buttons = {
        if props.pagination.number_pages <= props.max_pages_to_show {
            pages_view_uncapped(props, jump_page)
        } else {
            let page_range = get_page_range(props.pagination.page,
                                            props.pagination.number_pages,
                                            props.max_pages_to_show);
            match page_range {
                PageRange::Lower => pages_in_lower_range_view(props, jump_page),
                PageRange::Mid => pages_fit_in_limit_view(props, jump_page),
                PageRange::Upper => pages_fit_in_limit_view(props, jump_page)
            }
        }
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

fn pages_view_uncapped(props: &Props, jump_page: Callback<i32>) -> Html {
    let num_pages = props.pagination.number_pages;
    (1..num_pages+1).map(|i| {
        let page = i.to_string();
        let jump_page = jump_page.clone();
        return if i == props.pagination.page {
            html! {
                <button class="yew-grid-pagination-bar-control-button yew-grid-pagination-bar-control-button-selected">{page}</button>
            }
        } else {
            html! {
                <button onclick={ move|_| {jump_page.emit(i)}} class="yew-grid-pagination-bar-control-button">{page}</button>
            }
        }
    }).collect::<Html>()
}

fn pages_in_lower_range_view(props: &Props, jump_page: Callback<i32>) -> Html {
    let max_pages_to_show = props.max_pages_to_show;
    // show ellipses at second last page
    (1..max_pages_to_show + 1).map(|i| {
        let page = i.to_string();
        let jump_page = jump_page.clone();
        if i == max_pages_to_show - 1 {
            html! {
                <button class="yew-grid-pagination-bar-control-button yew-grid-pagination-bar-control-button-ellipsis">{"..."}</button>
            }
        } else if i == props.pagination.page {
            html! {
                <button class="yew-grid-pagination-bar-control-button yew-grid-pagination-bar-control-button-selected">{page}</button>
            }
        } else {
            html! {
                <button onclick={ move|_| {jump_page.emit(i)}} class="yew-grid-pagination-bar-control-button">{page}</button>
            }
        }
    }).collect::<Html>()
}

fn pages_fit_in_limit_view(props: &Props, jump_page: Callback<i32>) -> Html {
    let num_pages = props.pagination.number_pages;
    (1..num_pages+1).map(|i| {
        let page = i.to_string();
        let jump_page = jump_page.clone();
        return if i == props.pagination.page {
            html! {
                <button class="yew-grid-pagination-bar-control-button yew-grid-pagination-bar-control-button-selected">{page}</button>
            }
        } else {
            html! {
                <button onclick={ move|_| {jump_page.emit(i)}} class="yew-grid-pagination-bar-control-button">{page}</button>
            }
        }
    }).collect::<Html>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_page_range() {
        let max_num_pages = 10;
        let num_pages = 100;
        // lower range of pages is 1 to 10
        // ellipses shown at second last page
        let page_range = get_page_range(1, num_pages, max_num_pages);
        assert_eq!(page_range, PageRange::Lower);

        // midrange of pages is 11 to 90
        // ellipses shown at second page and second last page
        let page_range = get_page_range(11, num_pages, max_num_pages);

        // end range of pages is 91 to 100
        // ellipses shown at second page
        let page_range = get_page_range(91, num_pages, max_num_pages);
    }
}