use yew::prelude::*;
use yew_data_grid::data_grid::{GridData, GridDataColumn, DataGrid, GridDataColumnProps};

// row data type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
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
    fn get_config(&self) -> GridDataColumnProps {
        match self {
            TaskFields::Id => GridDataColumnProps {
                header_name: "Id".to_string(),
                width: 150,
                editable: false,
                sortable: true
            },
            TaskFields::Name => GridDataColumnProps {
                header_name: "Task Name".to_string(),
                width: 150,
                editable: true,
                sortable: true
            },
            TaskFields::Description => GridDataColumnProps {
                header_name: "Description Testing".to_string(),
                width: 150,
                editable: true,
                sortable: true
            },
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
#[function_component(App)]
fn app() -> Html {
    let rows: Vec<Task> = vec![Task{ id: 1, name: "task 1".to_string(), description: "task 1 description".to_string() },
                               Task{ id: 2, name: "task b".to_string(), description: "task b description".to_string() }];
    let columns: Vec<TaskFields> = vec![TaskFields::Id, TaskFields::Name, TaskFields::Description];
    let height = 400;
    let style = format!("width: 100%; height: {height}px;");
    html! (
        // https://yew.rs/docs/next/concepts/basic-web-technologies/css#inline-styles
        <div style={style}>
            <DataGrid<Task, TaskFields> rows={rows} columns={columns} page_size={5}/>
        </div>
    )
}

fn main() {
    yew::start_app::<App>();
}

#[cfg(test)]
mod tests {
    use yew_data_grid::data_grid::{GridData, GridDataColumn};
    use super::*;

    #[test]
    fn data_grid_column_and_row_api() {
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