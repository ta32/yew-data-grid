use yew::prelude::*;

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
    let num_rows = props.rows.len();
    let col_one = props.columns[2];
    let col_one_value = props.rows[0].get_value(col_one);
    html!(
        <div>
            <h1>{ "data grid" }</h1>
            <p>{num_rows}</p>
            <p>{col_one_value}</p>
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
