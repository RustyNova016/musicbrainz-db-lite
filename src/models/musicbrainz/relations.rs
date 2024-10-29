use core::marker::PhantomData;

use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Default, PartialEq, Eq, Clone)]
pub struct Relation<T, U> {
    pub id: i64,
    pub type_id: String,
    pub relation_type: String,
    pub direction: String,
    pub begin: Option<i64>,
    pub end: Option<i64>,
    pub attributes: Option<String>,
    pub attribute_ids: Option<String>,
    pub atribute_values: Option<String>,
    pub target_type: Option<String>,
    pub target_credit: Option<String>,
    pub source_credit: Option<String>,

    // Foreign keys
    pub entity0: i64,
    #[sqlx(skip)]
    pub(crate) entity0_phamtom: PhantomData<T>,
    pub entity1: i64,
    #[sqlx(skip)]
    pub(crate) entity1_phamtom: PhantomData<U>,
}

impl<T, U> Relation<T, U>
where
    T: Send + Unpin,
    U: Send + Unpin,
{
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, sqlx::Error> {
        let mut query = sqlx::query_as("INSERT INTO `recordings` (`id`, `type_id`, `relation_type`, `direction`, `begin`, `end`, `attributes`, `attribute_ids`, `atribute_values`, `target_type`, `target_credit`, `source_credit`, `entity0`, `entity1`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `type_id` = excluded.`type_id`, `relation_type` = excluded.`relation_type`, `direction` = excluded.`direction`, `begin` = excluded.`begin`, `end` = excluded.`end`, `attributes` = excluded.`attributes`, `attribute_ids` = excluded.`attribute_ids`, `atribute_values` = excluded.`atribute_values`, `target_type` = excluded.`target_type`, `target_credit` = excluded.`target_credit`, `source_credit` = excluded.`source_credit`, `entity0` = excluded.`entity0`, `entity1` = excluded.`entity1` RETURNING *;");
        query = query.bind(&self.type_id);
        query = query.bind(&self.relation_type);
        query = query.bind(&self.direction);
        query = query.bind(self.begin);
        query = query.bind(self.end);
        query = query.bind(&self.attributes);
        query = query.bind(&self.attribute_ids);
        query = query.bind(&self.atribute_values);
        query = query.bind(&self.target_type);
        query = query.bind(&self.target_credit);
        query = query.bind(&self.source_credit);
        query = query.bind(self.entity0);
        query = query.bind(self.entity1);
        query.fetch_one(conn).await
    }
}
