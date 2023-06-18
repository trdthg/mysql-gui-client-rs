use sqlx::{database::HasArguments, query::Query, Database, Encode, MySql, Type};
pub trait Bindable<DB: Database> {
    fn bind_to<'q>(
        self: Box<Self>,
        query: Query<'q, DB, <DB as HasArguments<'q>>::Arguments>,
    ) -> Query<'q, DB, <DB as HasArguments<'q>>::Arguments>
    where
        Self: 'q;
}

impl<DB: Database, T: Send + for<'q> Encode<'q, DB> + Type<DB>> Bindable<DB> for T {
    fn bind_to<'q>(
        self: Box<Self>,
        query: Query<'q, DB, <DB as HasArguments<'q>>::Arguments>,
    ) -> Query<'q, DB, <DB as HasArguments<'q>>::Arguments>
    where
        Self: 'q,
    {
        query.bind(*self)
    }
}
