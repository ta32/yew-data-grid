use std::collections::HashMap;
use instant::Instant as InstantWeb;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: GridData<ColumnType=U> + PartialEq, U: GridDataColumn<RowType=T> + PartialEq + Copy> {
    pub rows: Vec<T>,
    pub columns: Vec<U>,
    pub page_size: usize
}

const DATA_GRID_STYLE: &'static str = include_str!("data_grid.rs.css");

#[function_component(DataGrid)]
pub fn data_grid<T: GridData<ColumnType=U> + PartialEq,
                 U: GridDataColumn<RowType=T> + PartialEq + Copy>
                (props: &Props<T, U>) -> Html {
    // TODO conditional compilation of this effect
    let start = use_state(|| InstantWeb::now());
    {
        let start = start.clone();
        use_effect(move || {
            let elapsed = start.elapsed().as_millis();
            log::info!("data-grid rendered in {elapsed}ms");
            || {}
        });
    }

    let row_index_map = use_state(|| {
        let mut row_index_map = HashMap::new();
        for (i, v) in props.rows.iter().enumerate() {
            row_index_map.insert(v.get_id(), i);
        }
        row_index_map
    });

    let sort_order = use_state(|| {
        let mut sort_order = Vec::new();
        for row in props.rows.iter() {
            sort_order.push(row.get_id());
        }
        sort_order
    });

    let new_rows = props.rows.iter().filter(|r| !row_index_map.contains_key(&r.get_id())).collect::<Vec<&T>>();
    if new_rows.len() > 0 {
        log::info!("adding new {} rows", new_rows.len());
        let mut new_sort_order = (*sort_order).clone();
        for row in new_rows.iter() {
            new_sort_order.push(row.get_id());
        }

        let last_index = row_index_map.len();
        let new_row_indexes: HashMap<String, usize> = new_rows.iter().enumerate().map(|(i, r)| (r.get_id(), i + last_index)).collect();
        let mut current_row_indexes = (*row_index_map).clone();
        current_row_indexes.extend(new_row_indexes);

        row_index_map.set(current_row_indexes);
        sort_order.set(new_sort_order);
    }

    let total_width = props.columns.iter().fold(0, |acc, column| {
        let config = column.get_config();
        acc + config.width
    });

    log::info!("sort_order len {}", sort_order.len());
    log::info!("prop.rows len {}", props.rows.len());
    log::info!("row_index_map len {}", row_index_map.len());

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
        sort_order.iter().map(|i| {
            let row_index_str = i.to_string();
            let row = &props.rows[(*row_index_map)[&row_index_str]];
            // row column values
            let row_values = props.columns.iter().enumerate().map(|(i,col)| {
                let value = col.get_value(row);
                let col_index_str = i.to_string();
                let cell_width = col.get_config().width;
                let style = format!("width: {cell_width}px; height: 52px;");
                html! {
                <div class="yew-data-grid-cell" style={style} row-index={row_index_str.clone()} col-index={col_index_str}>
                    <div class="yew-data-grid-cell-content">{value}</div>
                </div>
            }
            }).collect::<Html>();
            let key = row.get_id();
            let table_style = format!("width: {total_width}px; height: 52px;");
            html! (
            <div class="yew-data-grid-row" key={key.to_string()} style={table_style} row-index={row_index_str}>
                {row_values}
            </div>
        )
        }).collect::<Html>()
    };
    let table_style = format!("width: {total_width}px; height: 52px;");
    html!(
        <div class="yew-data-grid-container">
            <style>{DATA_GRID_STYLE}</style>
            <h1>{ "data grid" }</h1>
            <div class="yew-data-grid-header-row" style={table_style}>
                {columns}
            </div>
            {grid}
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
