use mongodb::ClientSession;

pub struct MockTransaction;
pub enum DBTransaction<'a> {
    MongoDb(&'a mut ClientSession),
    Mock(&'a mut MockTransaction),
}

pub enum RepoDB {
    MongoDb(mongodb::Database),
    Mock,
}
