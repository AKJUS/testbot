use crate::schema::descriptions;
use diesel::{AsChangeset, Insertable, Queryable};
use chrono::NaiveDateTime;

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

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::command_history)]
pub struct CommandHistory {
    pub id: i32,
    pub user: String,
    pub command: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::schema::command_stats)]
pub struct CommandStat {
    pub id: i32,
    pub command: String,
    pub arguments: String,
    pub count: i32,
    pub last_used: chrono::NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description_struct() {
        let desc = Description {
            key: "foo".to_string(),
            value: "bar".to_string(),
        };
        assert_eq!(desc.key, "foo");
        assert_eq!(desc.value, "bar");
    }

    #[test]
    fn test_new_description_struct() {
        let new_desc = NewDescription {
            key: "foo",
            value: "bar",
        };
        assert_eq!(new_desc.key, "foo");
        assert_eq!(new_desc.value, "bar");
    }
}
