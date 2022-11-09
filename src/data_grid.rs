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
    let grid = props.rows.iter().enumerate().map(|(i, row)| {
        let row_index_str = i.to_string();
        // row elements
        let row_values = props.columns.iter().enumerate().map(|(i,col)| {
            let value = col.get_value(row);
            let col_index_str = i.to_string();
            let style = format!("width: 150px; height: 52px;");
            html! {
                <div class="yew-data-grid-cell" style={style} row-index={row_index_str.clone()} col-index={col_index_str}>
                    <div class="yew-data-grid-cell-content">{value}</div>
                </div>
            }
        }).collect::<Html>();
        let style = format!("height: 52px;");
        html! (
            <div class="yew-data-grid-row" style={style} row-index={row_index_str}>
                {row_values}
            </div>
        )
    }).collect::<Html>();
    html!(
        <div class="yew-data-grid-container">
            <style>{DATA_GRID_STYLE}</style>
            <h1>{ "data grid" }</h1>
            {grid}
        </div>
    )
}


pub trait GridDataColumn {
    type RowType;
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
