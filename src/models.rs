use diesel::{Queryable, Insertable, AsChangeset};
use crate::schema::descriptions;

#[derive(Queryable, AsChangeset)]
pub struct Description {
    pub key: String,
    pub value: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = descriptions)]
pub struct NewDescription<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description_struct() {
        let desc = Description { key: "foo".to_string(), value: "bar".to_string() };
        assert_eq!(desc.key, "foo");
        assert_eq!(desc.value, "bar");
    }

    #[test]
    fn test_new_description_struct() {
        let new_desc = NewDescription { key: "foo", value: "bar" };
        assert_eq!(new_desc.key, "foo");
        assert_eq!(new_desc.value, "bar");
    }
}
