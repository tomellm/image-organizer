use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
#[cfg(feature = "backend")]
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



#[cfg(feature = "backend")]
pub trait GraphNode {
    fn to_bulk_item(self) -> Vec<indradb::BulkInsertItem>;
}

impl Person {
    #[cfg(feature = "backend")]
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Person").unwrap()
    }
    #[cfg(feature = "backend")]
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

#[cfg(feature = "backend")]
impl GraphNode for Person {
    #[cfg(feature = "backend")]
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
    #[cfg(feature = "backend")]
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Group").unwrap()
    }
    #[cfg(feature = "backend")]
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

#[cfg(feature = "backend")]
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
    #[cfg(feature = "backend")]
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Activity").unwrap()
    }
    #[cfg(feature = "backend")]
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
    #[cfg(feature = "backend")]
    pub fn start_date_identifier() -> indradb::Identifier {
        indradb::Identifier::new("start-date").unwrap()
    }
    #[cfg(feature = "backend")]
    pub fn end_date_identifier() -> indradb::Identifier {
        indradb::Identifier::new("end-date").unwrap()
    }
}

#[cfg(feature = "backend")]
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
    #[cfg(feature = "backend")]
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Location").unwrap()
    }
    #[cfg(feature = "backend")]
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

#[cfg(feature = "backend")]
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
    #[cfg(feature = "backend")]
    pub fn identifier() -> indradb::Identifier {
        indradb::Identifier::new("Other").unwrap()
    }
    #[cfg(feature = "backend")]
    pub fn name_identifier() -> indradb::Identifier {
        indradb::Identifier::new("name").unwrap()
    }
}

#[cfg(feature = "backend")]
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
