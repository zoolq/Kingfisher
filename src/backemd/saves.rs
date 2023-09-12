use super::songdatabase::{Query, QueryResult};

pub trait SaveDatabase {
    fn append_query_history(&mut self, query: Query, result: QueryResult);
    fn get_history(&mut self, result: QueryResult);
}
