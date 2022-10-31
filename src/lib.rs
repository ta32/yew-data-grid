use yew::prelude::*;
use yew::Html;



trait GridDataColumn {
    type RowType;
    fn get_value(&self, row: &Self::RowType) -> String;
    fn get_field(&self) -> Self
        where Self: Sized + Copy
    {
       *self
    }
}

trait GridData {
    type ColumnType: GridDataColumn<RowType=Self>;
    fn get_value(&self, field: Self::ColumnType) -> String {
        field.get_value(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_grid_column_and_row_api() {
        // row data type
        struct Task {
            id: u32,
            name: String,
            description: String,
        }
        // column data type
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        enum TaskFields {
            Id,
            Name,
            Description,
        }
        // TODO - create derive macro for this
        impl GridDataColumn for TaskFields {
            type RowType = Task;
            fn get_value(&self, row: &Task) -> String {
                match self {
                    TaskFields::Id => row.id.to_string(),
                    TaskFields::Name => row.name.to_string(),
                    TaskFields::Description => row.description.to_string(),
                }
            }
        }
        // TODO - create derive macro for this
        impl GridData for Task {
            type ColumnType = TaskFields;
        }
        // wont compile be cause we are using associated type in trait
        // impl GridData for Task {
        // type ColumnType = TaskFields2;
        // }
        let row_instance = Task {
            id: 1,
            name: "Task 1".to_string(),
            description: "Description 1".to_string(),
        };
        assert_eq!(TaskFields::Id.get_value(&row_instance), "1");
        assert_eq!(TaskFields::Name.get_value(&row_instance), "Task 1");
        assert_eq!(TaskFields::Description.get_value(&row_instance), "Description 1");
        assert_eq!(TaskFields::Id.get_value(&row_instance), row_instance.get_value(TaskFields::Id));
    }
}
