use std::fmt;

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;

use super::Fetchable;
use crate::entities::{CourseId, EntityType, Module, Room, Staff, Unknown, UnknownId};

#[derive(Debug, Clone, PartialEq)]
pub struct Elements(pub Vec<Element>);

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub federation_id: UnknownId,
    pub entity_type: Unknown,
    pub elements: Elements,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RawElement<T: EntityType> {
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

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Element {
    Time(RawElement<Unknown>),
    Category(RawElement<Unknown>),
    Module(RawElement<Module>),
    Room(RawElement<Room>),
    Teacher(RawElement<Staff>),
    Grade(RawElement<Unknown>),
    Name(RawElement<Unknown>),
}

impl<'de> Deserialize<'de> for Elements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElementsVisitor;

        impl<'de> Visitor<'de> for ElementsVisitor {
            type Value = Vec<Element>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of side bar events")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                #[derive(Copy, Clone, PartialEq, Deserialize)]
                #[serde(field_identifier)]
                enum Tag {
                    Time,
                    #[serde(rename = "Catégorie")]
                    Category,
                    #[serde(rename = "Matière")]
                    Module,
                    #[serde(rename = "Salle", alias = "Salles")]
                    Room,
                    #[serde(rename = "Enseignant", alias = "Enseignants")]
                    Teacher,
                    #[serde(rename = "Note", alias = "Notes")]
                    Grade,
                    Name,
                }

                fn deserialize_element<'de, A>(tag: Tag, v: Value) -> Result<Element, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    Ok(match tag {
                        Tag::Time => Element::Time(
                            RawElement::<Unknown>::deserialize(v).map_err(de::Error::custom)?,
                        ),
                        Tag::Category => Element::Category(
                            RawElement::<Unknown>::deserialize(v).map_err(de::Error::custom)?,
                        ),
                        Tag::Module => Element::Module(
                            RawElement::<Module>::deserialize(v).map_err(de::Error::custom)?,
                        ),
                        Tag::Room => Element::Room(
                            RawElement::<Room>::deserialize(v).map_err(de::Error::custom)?,
                        ),
                        Tag::Teacher => Element::Teacher(
                            RawElement::<Staff>::deserialize(v).map_err(de::Error::custom)?,
                        ),
                        Tag::Grade => Element::Grade(
                            RawElement::<Unknown>::deserialize(v).map_err(de::Error::custom)?,
                        ),
                        Tag::Name => Element::Name(
                            RawElement::<Unknown>::deserialize(v).map_err(de::Error::custom)?,
                        ),
                    })
                }

                let mut elements = Vec::with_capacity(seq.size_hint().unwrap_or(0));

                let mut last_tag = None;
                while let Some(v) = seq.next_element::<Value>()? {
                    elements.push({
                        match Option::deserialize(&v["label"]).map_err(de::Error::custom)? {
                            Some(tag) => {
                                last_tag = Some(tag);
                                deserialize_element::<A>(tag, v)?
                            }
                            None => match last_tag {
                                Some(tag) => deserialize_element::<A>(tag, v)?,
                                None => {
                                    return Err(de::Error::custom("first element needs a label"));
                                }
                            },
                        }
                    });
                }

                Ok(elements)
            }
        }

        Ok(Elements(deserializer.deserialize_seq(ElementsVisitor)?))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRequest {
    pub event_id: CourseId,
}

impl Fetchable for Event {
    type Request = EventRequest;

    const METHOD_NAME: &'static str = "GetSideBarEvent";
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json};

    #[test]
    fn deserialize_element() {
        use Element::*;
        assert!(matches!(
            from_value::<Elements>(json!([
                {
                    "label": "Time",
                    "content": "11/9/2021 2:01 PM-5:16 PM",
                    "federationId": null,
                    "entityType": 0,
                    "assignmentContext": null,
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": "Catégorie",
                    "content": "TD",
                    "federationId": null,
                    "entityType": 0,
                    "assignmentContext": null,
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": "Matière",
                    "content": "Anglais [DPGANG3D]",
                    "federationId": "DPGANG3D",
                    "entityType": 100,
                    "assignmentContext": "a-start-end",
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": "Salles",
                    "content": "A ROOM",
                    "federationId": "1172982",
                    "entityType": 102,
                    "assignmentContext": "a-start",
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": null,
                    "content": "AN ANOTHER ROOM",
                    "federationId": "1172981",
                    "entityType": 102,
                    "assignmentContext": "a",
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": null,
                    "content": "YET AN ANOTHER ROOM",
                    "federationId": "1172977",
                    "entityType": 102,
                    "assignmentContext": "a-end-0",
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": "Enseignants",
                    "content": "SOME BODY",
                    "federationId": "012345",
                    "entityType": 101,
                    "assignmentContext": "a-start",
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": null,
                    "content": "SOMEBODY ELSE",
                    "federationId": "54321",
                    "entityType": 101,
                    "assignmentContext": "a-end-0",
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                },
                {
                    "label": "Notes",
                    "content": null,
                    "federationId": null,
                    "entityType": 0,
                    "assignmentContext": null,
                    "containsHyperlinks": false,
                    "isNotes": true,
                    "isStudentSpecific": false
                },
                {
                    "label": "Name",
                    "content": null,
                    "federationId": null,
                    "entityType": 0,
                    "assignmentContext": null,
                    "containsHyperlinks": false,
                    "isNotes": false,
                    "isStudentSpecific": false
                }
            ]))
            .unwrap()
            .0[..],
            [
                Time(_),
                Category(_),
                Module(_),
                Room(_),
                Room(_),
                Room(_),
                Teacher(_),
                Teacher(_),
                Grade(_),
                Name(_),
            ]
        ));
    }

    #[test]
    fn deserialize_event() {
        from_value::<Event>(json!({
            "federationId": null,
            "entityType": 0,
            "elements": []
        }))
        .unwrap();
    }
}
