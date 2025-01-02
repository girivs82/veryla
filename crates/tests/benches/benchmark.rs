use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::fs;
use veryla_analyzer::Analyzer;
use veryla_formatter::Formatter;
use veryla_metadata::Metadata;
use veryla_parser::Parser;

#[cfg(target_os = "linux")]
mod perf;

const EXCLUDES: [&str; 4] = [
    r"25_dependency.veryla",
    r"52_include.veryla",
    r"67_cocotb.veryla",
    r"68_std.veryla",
];

fn criterion_benchmark(c: &mut Criterion) {
    let mut text = String::new();
    for testcase in TESTCASES {
        if EXCLUDES.iter().any(|x| testcase.contains(x)) {
            continue;
        }
        let input = fs::read_to_string(testcase).unwrap();
        text.push_str(&input);
    }

    let metadata_path = Metadata::search_from_current().unwrap();
    let metadata = Metadata::load(&metadata_path).unwrap();

    // Check no analyzer error
    let parser = Parser::parse(&text, &"").unwrap();
    let prj = &metadata.project.name;
    let analyzer = Analyzer::new(&metadata);
    let mut errors = Vec::new();
    errors.append(&mut analyzer.analyze_pass1(prj, &text, &"", &parser.veryla));
    Analyzer::analyze_post_pass1();
    errors.append(&mut analyzer.analyze_pass2(prj, &text, &"", &parser.veryla));
    errors.append(&mut analyzer.analyze_pass3(prj, &text, &"", &parser.veryla));
    if !errors.is_empty() {
        dbg!(errors);
        assert!(false);
    }

    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(text.len() as u64));
    group.bench_function("parse", |b| {
        b.iter_with_large_drop(|| Parser::parse(black_box(&text), &""))
    });
    group.bench_function("analyze", |b| {
        b.iter_with_large_drop(|| {
            let parser = Parser::parse(black_box(&text), &"").unwrap();
            let prj = &metadata.project.name;
            let analyzer = Analyzer::new(black_box(&metadata));
            analyzer.analyze_pass1(prj, black_box(&text), &"", &parser.veryla);
            Analyzer::analyze_post_pass1();
            analyzer.analyze_pass2(prj, black_box(&text), &"", &parser.veryla);
            analyzer.analyze_pass3(prj, black_box(&text), &"", &parser.veryla);
            analyzer.clear();
        })
    });
    group.bench_function("format", |b| {
        b.iter_with_large_drop(|| {
            let parser = Parser::parse(black_box(&text), &"").unwrap();
            let mut formatter = Formatter::new(&metadata);
            formatter.format(&parser.veryla);
        })
    });
    group.finish();
}

include!(concat!(env!("OUT_DIR"), "/test.rs"));

#[cfg(target_os = "linux")]
criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = criterion_benchmark
}

#[cfg(not(target_os = "linux"))]
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}

criterion_main!(benches);
