use super::songdatabase::QueryResult;

pub trait SaveDatabase {
    fn append_query_history(&mut self, result: QueryResult);
    fn get_history(&mut self, result: QueryResult);
}
