use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

pub struct User {
    pub id: u32,
    pub name: Option<String>,
    pub username: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub uuid: Option<String>,
    pub token: Option<String>,
    pub sex: Option<i16>,
    pub source: Option<i16>,
    pub from_source: Option<i16>,
    pub avatar: Option<String>,
    pub lng: Option<f32>,
    pub lat: Option<f32>,
    pub created_at: Option<DateTime<UTC>>,
    pub updated_at: Option<DateTime<UTC>>,
    pub status: Option<i16>
}

impl Default for User {
    fn default() -> User {
        User{
            id: 0,
            name: None,
            username: None,
            mobile: None,
            email: None,
            password: None,
            uuid: None,
            token: None,
            sex: None,
            source: None,
            from_source: None,
            avatar: None,
            lng: None,
            lat: None,
            created_at: None,
            updated_at: None,
            status: None
        }
    }
}
