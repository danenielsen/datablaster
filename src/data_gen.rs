


trait Record {}

struct TestRecord {
}

impl Record for TestRecord {

}

struct RecordSchema {
    records_schemas: Option<Vec<Box<RecordSchema>>>,

}

impl RecordSchema {
    fn new() -> Self {
        RecordSchema{
            records_schemas: None,
        }
    }

    fn generate() -> Box<dyn Record> {
        Box::new(TestRecord{})
    }
}