use yew::prelude::*;
use yew::Callback;
use yew_data_grid::data_grid::{GridData, GridDataColumn, DataGrid, GridDataColumnProps};

// row data type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Task {
    pub id: usize,
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
    fn get_config(&self) -> GridDataColumnProps {
        match self {
            TaskFields::Id => GridDataColumnProps {
                header_name: "Id".to_string(),
                width: 50,
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
                width: 200,
                editable: true,
                sortable: true
            },
        }
    }
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
    type IdType = usize;
    type ColumnType = TaskFields;
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

// wont compile be cause we are using associated type in trait
// impl GridData for Task {
// type ColumnType = TaskFields2;
// }
#[function_component(App)]
fn app() -> Html {
    let rows = use_state(|| {
        vec![
            Task {
                id: 1,
                name: "Task 1".to_string(),
                description: "Task 1 Description".to_string(),
            },
            Task {
                id: 2,
                name: "Task 2".to_string(),
                description: "Task 2 Description".to_string(),
            },
            Task {
                id: 3,
                name: "Task 3".to_string(),
                description: "Task 3 Description".to_string(),
            },
        ]
    });
    let onclick = {
        let rows = rows.clone();
        Callback::from(move |_| {
            let mut new_rows = (*rows).clone();
            for n in 0..500 {
                new_rows.push(Task {
                    id: n,
                    name: format!("Task {}", n),
                    description: format!("Task {} Description", n),
                });
            }
            rows.set(new_rows);
        })
    };
    let columns: Vec<TaskFields> = vec![TaskFields::Id, TaskFields::Name, TaskFields::Description];
    let height = 400;
    let style = format!("width: 100%; height: {height}px;");
    let rows = (*rows).clone();
    html! (
        // https://yew.rs/docs/next/concepts/basic-web-technologies/css#inline-styles
        <>
            <button {onclick}>{ "Add 500" }</button>
            <div style={style}>
                <DataGrid<Task, TaskFields> rows={rows} columns={columns} page_size={5}/>
            </div>
        </>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
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