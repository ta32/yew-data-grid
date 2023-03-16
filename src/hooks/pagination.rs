use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pagination {
    pub page: i32,
    pub number_pages: i32,
    pub page_size: i32,
    pub total_rows: usize
}

impl Pagination {
    pub fn new(total_rows: usize, page_size: i32) -> Self {
        let number_pages = (total_rows as f32 / page_size as f32).ceil() as i32;
        Self {
            page: 1,
            number_pages,
            page_size,
            total_rows
        }
    }
}

#[hook]
pub fn use_page_view(p: Pagination, data_indexes: &Vec<String>) -> Rc<Vec<String>> {
    let slice = use_memo(|p| get_page_view(p, data_indexes), p);
    slice
}

fn get_page_view(p: &Pagination, data_indexes: &Vec<String>) -> Vec<String> {
    let start = (p.page - 1) * p.page_size;
    let end = start + p.page_size;
    return if end >= data_indexes.len() as i32 {
        data_indexes[start as usize..].to_vec()
    } else {
        data_indexes[start as usize..end as usize].to_vec()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pagination() {
        let total_rows = 105;
        let page_size = 10;
        let p = Pagination::new(total_rows, page_size);
        assert_eq!(p.page, 1);
        assert_eq!(p.number_pages, 11);
        assert_eq!(p.page_size, 10);
    }

    #[test]
    fn test_get_page_view() {
        let total_rows = 105;
        let page_size = 10;
        let p = Pagination::new(total_rows, page_size);
        let data_indexes = (0..total_rows).map(|i| i.to_string()).collect();
        let page_view = get_page_view(&p, &data_indexes);
        assert_eq!(page_view, vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    }

    #[test]
    fn given_empty_list_get_page_view() {
        let total_rows = 0;
        let page_size = 10;
        let p = Pagination::new(total_rows, page_size);
        let data_indexes: Vec<String> = vec![];
        let page_view = get_page_view(&p, &data_indexes);
        assert_eq!(page_view.len(), 0);
    }
}