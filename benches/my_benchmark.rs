use criterion::{black_box, criterion_group, criterion_main, Criterion};

use datame::schema::{RecordSchema, FieldSchema, FieldType};
use datame::data_repr::{Tuple};
use datame::data_gen::{create_data_from_schema};

fn create_single_float_schema() -> RecordSchema {
    RecordSchema::new()
        .with_column(FieldSchema::new("value", FieldType::Float(Default::default())))
}

fn create_single_string_schema() -> RecordSchema {
    RecordSchema::new()
        .with_column(FieldSchema::new("value", FieldType::String(Default::default())))
}

fn create_single_integer_schema() -> RecordSchema {
    RecordSchema::new()
        .with_column(FieldSchema::new("value", FieldType::Integer(Default::default())))
}

fn create_simple_schema() -> RecordSchema {
    RecordSchema::new()
        .with_column(FieldSchema::new("total", FieldType::Float(Default::default())))
        .with_column(FieldSchema::new("transaction_id", FieldType::Integer(Default::default())))
        .with_column(FieldSchema::new("sales_agent", FieldType::String(Default::default())))
        .with_column(FieldSchema::new("manager", FieldType::String(Default::default())))
}

fn create_complex_schema() -> RecordSchema {
    RecordSchema::new()
        .with_column(FieldSchema::new("total", FieldType::Float(Default::default())))
        .with_column(FieldSchema::new("transaction_id", FieldType::Integer(Default::default())))
        .with_column(FieldSchema::new("line_items", FieldType::List(Box::new(FieldType::Record(
            RecordSchema::new()
                .with_column(FieldSchema::new("item", FieldType::String(Default::default())))
                .with_column(FieldSchema::new("sub_items", FieldType::Record(
                    RecordSchema::new()
                        .with_column(FieldSchema::new("name", FieldType::String(Default::default())))
                        .with_column(FieldSchema::new("amount", FieldType::Integer(Default::default())))
                        .with_column(FieldSchema::new("cost", FieldType::Float(Default::default())))
                )))
                .with_column(FieldSchema::new("amount", FieldType::Integer(Default::default())))
                .with_column(FieldSchema::new("cost", FieldType::Float(Default::default())))
        )))))
        .with_column(FieldSchema::new("sales_agents", FieldType::List(Box::new(FieldType::String(Default::default())))))
        .with_column(FieldSchema::new("team", FieldType::Record(
            RecordSchema::new()
                .with_column(FieldSchema::new("project_manager", FieldType::String(Default::default())))
                .with_column(FieldSchema::new("team_members", FieldType::List(Box::new(FieldType::String(Default::default())))))
                .with_column(FieldSchema::new("budget", FieldType::Float(Default::default())))
            )
        ))
}

fn generate_tuple(rs: &RecordSchema) -> Tuple {
    create_data_from_schema(rs)
}

fn criterion_benchmark(c: &mut Criterion) {
    // Prep
    let single_float_schema = create_single_float_schema();
    let single_integer_schema = create_single_integer_schema();
    let single_string_schema = create_single_string_schema();
    let simple_schema = create_simple_schema();
    let complex_schema = create_complex_schema();

    // Run Benches
    // Create schemas
    c.bench_function("create_single_float_schema", |b| b.iter(|| create_single_float_schema()));
    c.bench_function("create_single_integer_schema", |b| b.iter(|| create_single_integer_schema()));
    c.bench_function("create_single_string_schema", |b| b.iter(|| create_single_string_schema()));
    c.bench_function("create_simple_schema", |b| b.iter(|| create_simple_schema()));
    c.bench_function("create_complex_schema", |b| b.iter(|| create_complex_schema()));

    // Generate data
    c.bench_function("tuple_from_single_float_schema", |b| b.iter(|| generate_tuple(black_box(&single_float_schema))));
    c.bench_function("tuple_from_single_integer_schema", |b| b.iter(|| generate_tuple(black_box(&single_integer_schema))));
    c.bench_function("tuple_from_single_string_schema", |b| b.iter(|| generate_tuple(black_box(&single_string_schema))));
    c.bench_function("tuple_from_simple_schema", |b| b.iter(|| generate_tuple(black_box(&simple_schema))));
    c.bench_function("tuple_from_complex_schema", |b| b.iter(|| generate_tuple(black_box(&complex_schema))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
