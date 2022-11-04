use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: GridData<ColumnType=U> + PartialEq, U: GridDataColumn<RowType=T> + PartialEq + Copy> {
    pub rows: Vec<T>,
    pub columns: Vec<U>,
    pub page_size: usize
}

#[function_component(DataGrid)]
pub fn data_grid<T: GridData<ColumnType=U> + PartialEq,
                 U: GridDataColumn<RowType=T> + PartialEq + Copy>
                (props: &Props<T, U>) -> Html {
    // let c = rows.iter().map(|r| cols.iter().map(move |c| (r, c))).flatten();
    let grid: Html = props.rows.iter().map(|row| props.columns.iter().map(move |col| (row, col))).flatten().map(|(row, col)| {
        let value = col.get_value(row);
        html! {
            <div>{value}</div>
        }
    }).collect::<Html>();
    html!(
        <div>
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
    use super::*;

    #[test]
    fn example_cross_product() {
        let rows = vec![1, 2];
        let cols = vec![4, 5];
        let c = rows.iter().map(|r| cols.iter().map(move |c| (r, c))).flatten();
        let d: Vec<(i32, i32)> = c.map(|(a, b)| (*a, *b)).collect();
        assert_eq!(d, vec![(1, 4), (1, 5), (2, 4), (2, 5)]);
    }
}
