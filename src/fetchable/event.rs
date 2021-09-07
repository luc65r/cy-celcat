use serde::{Deserialize, Serialize};

use super::Fetchable;
use crate::entities::{CourseId, EntityType, Module, Room, Staff, Unknown, UnknownId};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SideBarEvent {
    pub federation_id: UnknownId,
    pub entity_type: Unknown,
    pub elements: Vec<SideBarEventElement>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSideBarEventElement<T: EntityType> {
    pub content: Option<String>,
    #[serde(bound(deserialize = "T: EntityType"))]
    pub federation_id: T::Id,
    #[serde(bound(deserialize = "T: EntityType"))]
    pub entity_type: T,
    pub assignment_context: Option<String>,
    pub contains_hyperlinks: bool,
    pub is_notes: bool,
    pub is_student_specific: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "label")]
pub enum SideBarEventElement {
    Time(RawSideBarEventElement<Unknown>),
    #[serde(rename = "Catégorie")]
    Category(RawSideBarEventElement<Unknown>),
    #[serde(rename = "Matière")]
    Module(RawSideBarEventElement<Module>),
    #[serde(rename = "Salle")]
    Room(RawSideBarEventElement<Room>),
    #[serde(rename = "Enseignant")]
    Teacher(RawSideBarEventElement<Staff>),
    #[serde(rename = "Notes")]
    Grades(RawSideBarEventElement<Unknown>),
    Name(RawSideBarEventElement<Unknown>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SideBarEventRequest {
    pub event_id: CourseId,
}

impl Fetchable for SideBarEvent {
    type Request = SideBarEventRequest;

    const METHOD_NAME: &'static str = "GetSideBarEvent";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::RoomId;
    use serde_json::{from_value, json};

    #[test]
    fn deserialize_side_bar_event_element() {
        assert_eq!(
            from_value::<SideBarEventElement>(json!({
                "label": "Time",
                "content": "9/6/2021 8:30 AM-11:45 AM",
                "federationId": null,
                "entityType": 0,
                "assignmentContext": null,
                "containsHyperlinks": false,
                "isNotes": false,
                "isStudentSpecific": false
            }))
            .unwrap(),
            SideBarEventElement::Time(RawSideBarEventElement {
                content: Some("9/6/2021 8:30 AM-11:45 AM".to_owned()),
                federation_id: UnknownId,
                entity_type: Unknown,
                assignment_context: None,
                contains_hyperlinks: false,
                is_notes: false,
                is_student_specific: false,
            })
        );
        assert_eq!(
            from_value::<SideBarEventElement>(json!({
                "label": "Salle",
                "content": "CHE2 Larousse haut AMPHITHÉÂTRE 673p",
                "federationId": "1042721",
                "entityType": 102,
                "assignmentContext": "a-start-end",
                "containsHyperlinks": false,
                "isNotes": false,
                "isStudentSpecific": false
            }))
            .unwrap(),
            SideBarEventElement::Room(RawSideBarEventElement {
                content: Some("CHE2 Larousse haut AMPHITHÉÂTRE 673p".to_owned()),
                federation_id: RoomId("1042721".to_owned()),
                entity_type: Room,
                assignment_context: Some("a-start-end".to_owned()),
                contains_hyperlinks: false,
                is_notes: false,
                is_student_specific: false,
            })
        );
    }

    #[test]
    fn deserialize_side_bar_event() {
        from_value::<SideBarEvent>(json!({
            "federationId": null,
            "entityType": 0,
            "elements": []
        }))
        .unwrap();
    }
}
