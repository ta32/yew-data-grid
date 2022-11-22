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
    let row_indexes = use_state(|| {
        let mut row_indexes = Vec::new();
        for i in 0..props.rows.len() {
            row_indexes.push(i);
        }
        row_indexes
    });

    // TODO this design will not work when sorting is implemented the parent component will need to pass ids to track rows between prop updates and sorting
    if props.rows.len() != row_indexes.len() {
        row_indexes.set(props.rows.iter().enumerate().map(|(i, _)| i).collect());
    }

    let total_width = props.columns.iter().fold(0, |acc, column| {
        let config = column.get_config();
        acc + config.width
    });

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
        let row_indexes = row_indexes.clone();
        row_indexes.iter().map(|i| {
            let row_index_str = i.to_string();
            let row = &props.rows[*i];
            // row elements
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
            let table_style = format!("width: {total_width}px; height: 52px;");
            html! (
            <div class="yew-data-grid-row" style={table_style} row-index={row_index_str}>
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
    type ColumnType: GridDataColumn<RowType=Self>;
    fn get_value(&self, field: Self::ColumnType) -> String {
        field.get_value(&self)
    }
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
