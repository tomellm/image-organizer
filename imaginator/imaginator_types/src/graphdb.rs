use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use indradb;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MediaElement {
    Person(Person),
    Group(Group),
    Activity(Activity),
    Location(Location),
    Other(Other),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub uuid: Uuid,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub uuid: Uuid,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub uuid: Uuid,
    pub name: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Other {
    pub uuid: Uuid,
    pub name: String,
}



pub trait GraphNode {
    fn to_bulk_item(self) -> Vec<indradb::BulkInsertItem>;
}

impl Person {
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Person").unwrap()
    }
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

impl GraphNode for Person {
    fn to_bulk_item(self) -> Vec<indradb::BulkInsertItem> {
        let mut vec = vec![];
        vec.push(indradb::BulkInsertItem::Vertex(
                indradb::Vertex::with_id(self.uuid, Self::identifier())
        ));
        vec.push(indradb::BulkInsertItem::VertexProperty(
            self.uuid, Self::name_identifier(),
            indradb::Json::new(serde_json::to_value(self.name).unwrap())
        ));
        vec
    }
}

impl Group {
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Group").unwrap()
    }
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

impl GraphNode for Group {
    fn to_bulk_item(self) -> Vec<indradb::BulkInsertItem> {
        let mut vec = vec![];
        vec.push(indradb::BulkInsertItem::Vertex(
                indradb::Vertex::with_id(self.uuid, Self::identifier())
        ));
        vec.push(indradb::BulkInsertItem::VertexProperty(
            self.uuid, Self::name_identifier(),
            indradb::Json::new(serde_json::to_value(self.name).unwrap())
        ));
        vec
    }
}

impl Activity {
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Activity").unwrap()
    }
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
    pub fn start_date_identifier() -> indradb::Identifier {
        indradb::Identifier::new("start-date").unwrap()
    }
    pub fn end_date_identifier() -> indradb::Identifier {
        indradb::Identifier::new("end-date").unwrap()
    }
}

impl GraphNode for Activity {
    fn to_bulk_item(self) -> Vec<indradb::BulkInsertItem> {
        let mut vec = vec![];
        vec.push(indradb::BulkInsertItem::Vertex(
                indradb::Vertex::with_id(self.uuid, Self::identifier())
        ));
        vec.push(indradb::BulkInsertItem::VertexProperty(
            self.uuid, Self::name_identifier(),
            indradb::Json::new(serde_json::to_value(self.name).unwrap())
        ));
        vec.push(indradb::BulkInsertItem::VertexProperty(
            self.uuid, Self::start_date_identifier(),
            indradb::Json::new(serde_json::to_value(self.start_date).unwrap())
        ));
        vec.push(indradb::BulkInsertItem::VertexProperty(
            self.uuid, Self::end_date_identifier(),
            indradb::Json::new(serde_json::to_value(self.end_date).unwrap())
        ));
        vec
    }
}


impl Location {
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Location").unwrap()
    }
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

impl GraphNode for Location {

    fn to_bulk_item(self) -> Vec<indradb::BulkInsertItem> {
        let mut vec = vec![];
        vec.push(indradb::BulkInsertItem::Vertex(
                indradb::Vertex::with_id(self.uuid, Self::identifier())
        ));
        vec.push(indradb::BulkInsertItem::VertexProperty(
            self.uuid, Self::name_identifier(),
            indradb::Json::new(serde_json::to_value(self.name).unwrap())
        ));
        vec
    }
}

impl Other {
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Other").unwrap()
    }
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

impl GraphNode for Other {
    fn to_bulk_item(self) -> Vec<indradb::BulkInsertItem> {
        let mut vec = vec![];
        vec.push(indradb::BulkInsertItem::Vertex(
                indradb::Vertex::with_id(self.uuid, Self::identifier())
        ));
        vec.push(indradb::BulkInsertItem::VertexProperty(
            self.uuid, Self::name_identifier(),
            indradb::Json::new(serde_json::to_value(self.name).unwrap())
        ));
        vec
    }
}
