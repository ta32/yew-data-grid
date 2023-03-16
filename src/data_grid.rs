use std::collections::HashMap;
use instant::{Instant as InstantWeb};
use crate::hooks::pagination::{Pagination, use_page_view};
use crate::grid_pagination_bar::{GridPaginationBar};
use yew::prelude::*;

const DATA_GRID_STYLE: &'static str = include_str!("data_grid.rs.css");

pub struct RowState {
    pub row_index_map: HashMap<String, usize>,
    pub sort_order: Vec<String>
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: GridData<ColumnType=U> + PartialEq, U: GridDataColumn<RowType=T> + PartialEq + Copy> {
    pub rows: Vec<T>,
    pub columns: Vec<U>,
    pub page_size: i32
}

#[function_component(DataGrid)]
pub fn data_grid<T: GridData<ColumnType=U> + PartialEq,
                 U: GridDataColumn<RowType=T> + PartialEq + Copy>
                (props: &Props<T, U>) -> Html {
    // TODO conditional compilation of this effect
    let _force_update = use_state(|| InstantWeb::now());
    let pg = use_state(|| Pagination::new(props.rows.len(), props.page_size));

    let start = use_mut_ref(|| InstantWeb::now());
    start.replace(InstantWeb::now());
    use_effect(move || {
        let render_end = InstantWeb::now();
        let render_start = *start.borrow();
        let elapsed = (render_end - render_start).as_millis();
        log::info!("data-grid rendered in {elapsed}ms");
        || {}
    });

    let row_state = use_mut_ref(|| {
        let mut row_index_map = HashMap::new();
        let mut sort_order = Vec::new();
        for (i, row) in props.rows.iter().enumerate() {
            row_index_map.insert(row.get_id(), i);
            sort_order.push(row.get_id());
        }
        RowState { row_index_map, sort_order }
    });

    if props.rows.len() != row_state.borrow().sort_order.len() {
        let new_rows = props.rows.iter().filter(|r| !row_state.borrow().row_index_map.contains_key(&r.get_id())).collect::<Vec<&T>>();
        if row_state.borrow().sort_order.len() + new_rows.len() != props.rows.len() {
            log::error!("duplicate keys in rows detected cannot render data grid");
        } else {
            let mut new_sort_order = row_state.borrow().sort_order.clone();
            for row in new_rows.iter() {
                new_sort_order.push(row.get_id());
            }
            let last_index = row_state.borrow().row_index_map.len();
            let new_row_indexes: HashMap<String, usize> = new_rows.iter().enumerate().map(|(i, r)| (r.get_id(), i + last_index)).collect();
            let mut current_row_indexes = row_state.borrow().row_index_map.clone();
            current_row_indexes.extend(new_row_indexes);
            row_state.replace(RowState { row_index_map: current_row_indexes, sort_order: new_sort_order });
            pg.set(Pagination::new(props.rows.len(), props.page_size));
        }
    }

    // let total_width = props.columns.iter().fold(0, |acc, column| {
    //     let config = column.get_config();
    //     acc + config.width
    // });

    let columns = props.columns.iter().map(|column| {
        let config = column.get_config();
        let header_name = config.header_name;
        let width = config.width;
        let style = format!("width: {width}px");
        html! {
            <div class="yew-data-grid-header-cell" style={style}>{header_name}</div>
        }
    }).collect::<Html>();

    let grid = {
        // let page_view = &row_state.borrow().sort_order;
        let page_view = use_page_view(*pg, &row_state.borrow().sort_order);
        page_view.iter().map(|i| {
            let row_key = i.to_string();
            let row = &props.rows[row_state.borrow().row_index_map[&row_key]];
            const CELL_HEIGHT: i32 = 52;
            let cell_values = props.columns.iter().enumerate().map(|(i,col)| {
                let value = col.get_value(row);
                let col_index_str = i.to_string();
                let cell_width = col.get_config().width;
                let style = format!("width: {cell_width}px; min-height: {CELL_HEIGHT}px;");
                html! {
                <div class="yew-data-grid-cell" style={style} row-index={row_key.clone()} col-index={col_index_str}>
                    <div class="yew-data-grid-cell-content">{value}</div>
                </div>
            }
            }).collect::<Html>();
            let key = row.get_id();
            let style = format!("width: 100%; min-height: {CELL_HEIGHT}px; display: flex");
            let empty_cell = html! {
                <div class="yew-data-grid-cell" style={style} row-index={row_key.clone()} col-index="0">
                    <div class="yew-data-grid-cell-content"></div>
                </div>
            };
            let row_style = format!("width: 100%; min-height: {CELL_HEIGHT}px;");
            html! (
            <div class="yew-data-grid-row" key={key.to_string()} style={row_style} row-index={row_key}>
                {cell_values}
                {empty_cell}
            </div>
        )
        }).collect::<Html>()
    };
    let table_style = format!("width: 100%; min-height: 52px;");
    let empty_header = html! {
        <div class="yew-data-grid-header-cell" style="width: 100%; display: flex"></div>
    };

    html!(
         <div class="yew-data-grid-container">
            <style>{DATA_GRID_STYLE}</style>
            <div class="yew-data-grid-header-row" style={table_style}>
                {columns}
                {empty_header}
            </div>
            <div class="yew-data-grid-scrollable">
                {grid}
            </div>
            <div class="yew-data-grid-footer-container">
               <GridPaginationBar pagination={{pg}}/>
            </div>
        </div>
    )
}

pub struct GridDataColumnProps {
    pub header_name: String,
    pub width: i32,
    pub editable: bool,
    pub sortable: bool
}

pub trait GridDataColumn {
    type RowType;
    fn get_config(&self) -> GridDataColumnProps;
    fn get_value(&self, row: &Self::RowType) -> String;
    fn get_field(&self) -> Self
        where Self: Sized + Copy
    {
       *self
    }
}

pub trait GridData {
    type IdType: Sized + Clone;
    type ColumnType: GridDataColumn<RowType=Self>;
    fn get_value(&self, field: Self::ColumnType) -> String {
        field.get_value(&self)
    }
    fn get_id(&self) -> String;
}


#[cfg(test)]
mod tests {

    #[test]
    fn example_cross_product() {
        let rows = vec![1, 2 , 3];
        let cols = vec![4, 5];
        let c = rows.iter().map(|r| cols.iter().map(move |c| (r, c))).flatten();
        let d: Vec<(i32, i32)> = c.map(|(a, b)| (*a, *b)).collect();
        assert_eq!(d, vec![(1, 4), (1, 5), (2, 4), (2, 5), (3, 4), (3, 5)]);

        let rows = vec![4, 5];
        let cols = vec![1, 2 , 3];
        let c = rows.iter().map(|r| cols.iter().map(move |c| (r, c))).flatten();
        let d: Vec<(i32, i32)> = c.map(|(a, b)| (*a, *b)).collect();
        assert_eq!(d, vec![(4, 1), (4, 2), (4, 3), (5, 1), (5, 2), (5, 3)]);

        // iterator with index (enumerate)
        let rows = vec![1, 2 , 3];
        let cols = vec![4, 5];
        let c = rows.iter().enumerate().map(|(i, r)| cols.iter().map(move |c| (i, r, c))).flatten();
        let d: Vec<(usize, i32, i32)> = c.map(|(a, b, c)| (a, *b, *c)).collect();
        assert_eq!(d, vec![(0, 1, 4), (0, 1, 5), (1, 2, 4), (1, 2, 5), (2, 3, 4), (2, 3, 5)]);
    }
}
