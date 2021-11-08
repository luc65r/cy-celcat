use std::marker::PhantomData;

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};

use super::Fetchable;
use crate::entities::{CourseId, ModuleId, ResourceType};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Course {
    pub id: CourseId,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
    pub all_day: bool,
    pub description: String,
    pub background_color: String,
    pub text_color: String,
    pub department: Option<String>,
    pub faculty: Option<String>,
    pub event_category: Option<String>,
    pub sites: Option<Vec<String>>,
    pub modules: Option<Vec<ModuleId>>,
    pub register_status: i64, // TODO: values?
    pub student_mark: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum CalView {
    Month,
    AgendaWeek,
    AgendaDay,
    ListWeek,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarDataRequest<T: ResourceType> {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub res_type: T,
    pub cal_view: CalView,
    pub federation_ids: T::Id, // TODO: array
    pub colour_scheme: i64,    // TODO: values?
}

#[derive(Debug, Clone)]
pub struct CalendarData<T: ResourceType> {
    pub courses: Vec<Course>,
    pub request: PhantomData<T>,
}

impl<T> Fetchable for CalendarData<T>
where
    T: ResourceType,
{
    type Request = CalendarDataRequest<T>;

    const METHOD_NAME: &'static str = "GetCalendarData";
}

impl<'de, T> Deserialize<'de> for CalendarData<T>
where
    T: ResourceType,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<Course>::deserialize(deserializer).map(|cs| CalendarData {
            courses: cs,
            request: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::Student;
    use chrono::NaiveDate;
    use serde_json::{from_value, json};

    #[test]
    fn deserialize_course() {
        assert_eq!(
            from_value::<Course>(json!({
                "id": "-1347128091:-662573064:1:42367:4",
                "start": "2021-09-22T14:30:00",
                "end": "2021-09-22T17:45:00",
                "allDay": false,
                "description": "Some description",
                "backgroundColor": "#FF0000",
                "textColor": "#ffffff",
                "department": "1 : UFR DROIT",
                "faculty": null,
                "eventCategory": "CM",
                "sites": [
                    "CHENES"
                ],
                "modules": [
                    "1BAIJU1M"
                ],
                "registerStatus": 2,
                "studentMark": 0,
                "custom1": null,
                "custom2": null,
                "custom3": null
            }))
            .unwrap(),
            Course {
                id: CourseId("-1347128091:-662573064:1:42367:4".to_owned()),
                start: NaiveDate::from_ymd(2021, 9, 22).and_hms(14, 30, 0),
                end: Some(NaiveDate::from_ymd(2021, 9, 22).and_hms(17, 45, 0)),
                all_day: false,
                description: "Some description".to_owned(),
                background_color: "#FF0000".to_owned(),
                text_color: "#ffffff".to_owned(),
                department: Some("1 : UFR DROIT".to_owned()),
                faculty: None,
                event_category: Some("CM".to_owned()),
                sites: Some(vec!["CHENES".to_owned()]),
                modules: Some(vec![ModuleId("1BAIJU1M".to_owned())]),
                register_status: 2,
                student_mark: 0.0,
            }
        );
    }

    #[test]
    fn deserialize_calendar_data() {
        from_value::<CalendarData<Student>>(json!([])).unwrap();
    }
}
