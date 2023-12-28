use async_std::task;
use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::ListItem,
};
use sqlx::Error;

use crate::postgres;

use super::stateful_list::StatefulList;

#[derive(Debug, Clone)]
pub struct DBTable {
    pub name: String,
    pub schema: String,
    pub user_defined: bool,
}

#[derive(Clone)]
pub enum DBObject {
    Table(DBTable),
}

pub struct DBExplorer {
    pub show_system_objs: bool,
    items: Vec<DBObject>,
    pub items_visible: StatefulList<DBObject>,
}

trait ToGenericDBObj {
    fn to_obj(&self) -> DBObject;
}

impl ToGenericDBObj for postgres::PGTable {
    fn to_obj(&self) -> DBObject {
        DBObject::Table(DBTable {
            name: self.name.clone(),
            schema: self.schema.clone(),
            user_defined: self.user_defined,
        })
    }
}

pub fn new() -> DBExplorer {
    let items = task::block_on(items()).unwrap();

    DBExplorer {
        show_system_objs: false,
        items: items.clone(),
        items_visible: StatefulList::with_items(items),
    }
}

impl DBExplorer {
    pub fn update_visible_items(&mut self) {
        let items = self
            .items
            .clone()
            .into_iter()
            .filter(|i| match i {
                DBObject::Table(table) => table.user_defined != self.show_system_objs,
            })
            .collect();

        self.items_visible = StatefulList::with_items(items);
    }

    pub fn to_list_items(&self) -> Vec<ListItem> {
        let items: Vec<ListItem> = self
            .items_visible
            .items
            .iter()
            .map(|i| match i {
                DBObject::Table(table) => {
                    let lines = vec![Line::from(format!(
                        "{}.{}",
                        table.schema.clone(),
                        table.name.clone()
                    ))];
                    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
                }
            })
            .collect();

        items
    }
}

pub async fn items() -> Result<Vec<DBObject>, Error> {
    let objects = postgres::tables().await?;
    let result: Vec<_> = objects.into_iter().map(|o| o.to_obj()).collect();
    Ok(result)
}
