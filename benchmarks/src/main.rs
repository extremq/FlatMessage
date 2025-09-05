mod get_size_min;
mod structures;

use crate::get_size_min::GetSize;
use ascii_table::{Align, AsciiTable};
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use flat_message::{FlatMessage, FlatMessageOwned, Storage};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Write;
use std::fs;
use std::hash::Hash;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

fn s(mut x: String) -> String {
    x.shrink_to_fit();
    x
}
fn v<T>(mut x: Vec<T>) -> Vec<T> {
    x.shrink_to_fit();
    x
}

#[macro_export]
macro_rules! t {
    ($n:ident) => {
        impl GetSize for $n {}
    };
}

struct TestData {
    vec: Vec<u8>,
    storage: Storage,
    times: u32,
}

// ----------------------------------------------------------------------------

fn se_test_flat_message<'a, T: FlatMessage<'a>>(process: &T, data: &mut TestData) {
    process
        .serialize_to(&mut data.storage, flat_message::Config::default())
        .unwrap();
}

fn de_test_flat_message<T: FlatMessageOwned>(data: &TestData) -> T {
    T::deserialize_from(&data.storage).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bson<S: Serialize>(process: &S, data: &mut TestData) {
    data.vec = bson::to_vec(&process).unwrap();
}

fn de_test_bson<S: DeserializeOwned>(data: &TestData) -> S {
    bson::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_cbor<S: Serialize>(process: &S, data: &mut TestData) {
    ciborium::into_writer(process, &mut data.vec).unwrap();
}

fn de_test_cbor<S: DeserializeOwned>(data: &TestData) -> S {
    ciborium::from_reader(data.vec.as_slice()).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_json<S: Serialize>(process: &S, data: &mut TestData) {
    serde_json::to_writer(&mut data.vec, process).unwrap();
}

fn de_test_json<S: DeserializeOwned>(data: &TestData) -> S {
    serde_json::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_simd_json<S: Serialize>(process: &S, data: &mut TestData) {
    simd_json::serde::to_writer(&mut data.vec, process).unwrap();
}

fn de_test_simd_json<S: DeserializeOwned>(data: &TestData) -> S {
    simd_json::serde::from_reader(data.vec.as_slice()).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_rmp_schema<S: Serialize>(process: &S, data: &mut TestData) {
    rmp_serde::encode::write(&mut data.vec, process).unwrap();
}

fn se_test_rmp_schemaless<S: Serialize>(process: &S, data: &mut TestData) {
    rmp_serde::encode::write_named(&mut data.vec, process).unwrap();
}

fn de_test_rmp<S: DeserializeOwned>(data: &TestData) -> S {
    rmp_serde::decode::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_bincode<S: Serialize>(process: &S, data: &mut TestData) {
    bincode::serialize_into(&mut data.vec, process).unwrap();
}

fn de_test_bincode<S: DeserializeOwned>(data: &TestData) -> S {
    bincode::deserialize(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_flexbuffers<S: Serialize>(process: &S, data: &mut TestData) {
    data.vec = flexbuffers::to_vec(process).unwrap();
}

fn de_test_flexbuffers<S: DeserializeOwned>(data: &TestData) -> S {
    flexbuffers::from_slice(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

fn se_test_postcard<S: Serialize>(process: &S, data: &mut TestData) {
    postcard::to_io(process, &mut data.vec).unwrap();
}

fn de_test_postcard<S: DeserializeOwned>(data: &TestData) -> S {
    postcard::from_bytes(&data.vec).unwrap()
}

// ----------------------------------------------------------------------------

struct TestTimes {
    min: Duration,
    max: Duration,
    median: Duration,
}
impl TestTimes {
    fn to_string(&self) -> String {
        format!(
            "{:>6.2} [{:>6.2} - {:>6.2}]",
            self.median.as_secs_f64() * 1000.0,
            self.min.as_secs_f64() * 1000.0,
            self.max.as_secs_f64() * 1000.0
        )
    }
}

struct Result {
    name: AlgoKind,
    top_test_name: TestKind,
    size: usize,
    needs_schema: bool,
    min_size: usize,
    size_repr: String,
    //
    times_se: Vec<Duration>,
    times_de: Vec<Duration>,
    times_se_de: Vec<Duration>,
    //
    time_se_ms: String,
    time_de_ms: String,
    time_se_de_ms: String,
    //
    compare_key: u128,
}

fn compute_test_times(times: &mut Vec<Duration>) -> TestTimes {
    // sort the times (it is assume that at least one time is present)
    times.sort();
    let min = times[0];
    let max = times[times.len() - 1];
    // for odd number of times, the median is the middle one
    // for even number of times, the median is the average of the two middle ones
    let middle = times.len() / 2;
    let median = if times.len() % 2 == 0 {
        (times[middle] + times[middle - 1]) / 2
    } else {
        times[middle]
    };
    TestTimes { min, max, median }
}

fn se_bench<T, FS: Fn(&T, &mut TestData) + Clone>(
    x: &T,
    serialize: FS,
    data: &mut TestData,
) -> Duration {
    let start = Instant::now();
    for _ in 0..data.times {
        data.vec.clear();
        data.storage.clear();
        black_box(serialize(x, data));
        black_box(data.vec.len());
        black_box(data.storage.len());
    }
    start.elapsed()
}

fn de_bench<T, FD: Fn(&TestData) -> T>(deserialize: FD, data: &TestData) -> Duration {
    let start = Instant::now();
    for _ in 0..data.times {
        black_box(deserialize(black_box(data)));
    }
    start.elapsed()
}

fn se_de_bench<T, FS: Fn(&T, &mut TestData) + Clone, FD: Fn(&TestData) -> T + Clone>(
    x: &T,
    serialize: FS,
    deserialize: FD,
    data: &mut TestData,
) -> Duration {
    let start = Instant::now();
    for _ in 0..data.times {
        data.vec.clear();
        data.storage.clear();
        black_box(serialize(x, data));
        black_box(data.vec.len());
        black_box(data.storage.len());
        black_box(deserialize(black_box(data)));
    }
    start.elapsed()
}

fn bench<T: GetSize, FS: Fn(&T, &mut TestData) + Clone, FD: Fn(&TestData) -> T + Clone>(
    top_test_name: TestKind,
    test_name: AlgoKind,
    x: &T,
    serialize: FS,
    deserialize: FD,
    needs_schema: bool,
    results: &mut Vec<Result>,
    iterations: u32,
) {
    let mut data = TestData {
        vec: Vec::default(),
        storage: Storage::default(),
        times: iterations,
    };
    let time_se = se_bench(x, serialize.clone(), &mut data);
    let time_de = de_bench(deserialize.clone(), &data);
    let time_se_de = se_de_bench(x, serialize, deserialize, &mut data);

    // check if the test has been performed before
    // if yes -> atunci updatam direct / altfel creem
    let index = results
        .iter()
        .position(|x| x.name == test_name && x.top_test_name == top_test_name);

    if let Some(index) = index {
        results[index].times_se.push(time_se);
        results[index].times_de.push(time_de);
        results[index].times_se_de.push(time_se_de);
    } else {
        results.push(Result {
            name: test_name,
            top_test_name,
            size: data.vec.len().max(data.storage.len()),
            min_size: x.get_heap_size(),
            needs_schema,
            times_se: vec![time_se],
            times_de: vec![time_de],
            times_se_de: vec![time_se_de],
            time_se_ms: String::new(),
            time_de_ms: String::new(),
            time_se_de_ms: String::new(),
            size_repr: String::new(),
            compare_key: 0,
        });
    }
}

// Little hack to redirect the deserialize_from to deserialize_from_unchecked
// Just for testing, don't actually do this.
#[derive(get_size_derive::GetSize)]
struct Wrapper<T>(T);
impl<'a, T: FlatMessage<'a>> FlatMessage<'a> for Wrapper<T> {
    fn serialize_to(
        &self,
        output: &mut Storage,
        config: flat_message::Config,
    ) -> std::result::Result<(), flat_message::Error> {
        self.0.serialize_to(output, config)
    }

    fn deserialize_from(input: &'a Storage) -> std::result::Result<Self, flat_message::Error>
    where
        Self: Sized,
    {
        unsafe { Self::deserialize_from_unchecked(input) }
    }

    unsafe fn deserialize_from_unchecked(
        input: &'a Storage,
    ) -> std::result::Result<Self, flat_message::Error>
    where
        Self: Sized,
    {
        Ok(Wrapper(T::deserialize_from_unchecked(input)?))
    }
}

fn add_benches<'a, T: FlatMessageOwned + Clone + Serialize + DeserializeOwned + GetSize>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    iterations: u32,
) {
    let wrapper = Wrapper(x.clone());

    macro_rules! b {
        ($name:expr, $x:expr, $se:expr, $de:expr, $needs_schema:expr) => {
            if all_algos || algos.contains(&$name) {
                bench(
                    top_test_name,
                    $name,
                    $x,
                    $se,
                    $de,
                    $needs_schema,
                    results,
                    iterations,
                );
            }
        };
    }

    use AlgoKind::*;
    b!(
        FlatMessage,
        x,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(
        FlatMessageUnchecked,
        &wrapper,
        se_test_flat_message,
        de_test_flat_message,
        false
    );
    b!(RmpSchema, x, se_test_rmp_schema, de_test_rmp, true);
    b!(RmpSchemaless, x, se_test_rmp_schemaless, de_test_rmp, false);
    b!(Bincode, x, se_test_bincode, de_test_bincode, true);
    b!(
        FlexBuffers,
        x,
        se_test_flexbuffers,
        de_test_flexbuffers,
        false
    );
    b!(Cbor, x, se_test_cbor, de_test_cbor, false);
    b!(Bson, x, se_test_bson, de_test_bson, false);
    b!(Json, x, se_test_json, de_test_json, false);
    b!(SimdJson, x, se_test_simd_json, de_test_simd_json, false);
    b!(Postcard, x, se_test_postcard, de_test_postcard, true);
}

fn print_results_ascii_table(r: &[[&dyn Display; 7]], colums: &[(&str, Align)], file_name: &str) {
    let mut ascii_table: AsciiTable = AsciiTable::default();
    ascii_table.set_max_width(200);

    for (i, (name, align)) in colums.iter().enumerate() {
        ascii_table.column(i).set_header(*name).set_align(*align);
    }

    ascii_table.print(r);
}

fn print_results_markdown(r: &[[&dyn Display; 7]], colums: &[(&str, Align)], file_name: &str) {
    let output = &mut String::with_capacity(4096);

    for i in colums {
        write!(output, "| {} ", i.0).unwrap();
    }
    writeln!(output, "|").unwrap();
    for _ in colums {
        write!(output, "| --- ").unwrap();
    }
    writeln!(output, "|").unwrap();

    for row in r {
        for i in row {
            write!(output, "| {} ", i).unwrap();
        }
        writeln!(output, "|").unwrap();
    }

    fs::write(file_name, output).unwrap();
}

fn print_results_mdbook(r: &[[&dyn Display; 7]], _columns: &[(&str, Align)], file_name: &str) {
    let mut output = String::with_capacity(4096);

    writeln!(output, "| Method | Size (b) | Serialization Time (ms) | Deserialization Time (ms) | Total Time (ms) |").unwrap();
    writeln!(output, "| ------ | -------: | ----------------------: | ------------------------: | --------------: |").unwrap();

    for row in r {
        // name
        write!(output, "| {} ", row[2]).unwrap();
        // size
        write!(output, "| {} ", row[3]).unwrap();
        // se time
        write!(output, "| {} ", row[4]).unwrap();
        // de time
        write!(output, "| {} ", row[5]).unwrap();
        // total time
        write!(output, "| {} ", row[6]).unwrap();
        writeln!(output, "|").unwrap();
    }
    output = output.replace(
        "[",
        r#"<span style="font-family:monospace; opacity:0.5; font-size:0.75em">["#,
    );
    output = output.replace("]", r#"]</span>"#);
    let mut mdbook_output = String::with_capacity(output.len());
    let mut inside_sb = false;
    for ch in output.chars() {
        match ch {
            '[' => {
                inside_sb = true;
                mdbook_output.push(ch);
            }
            ' ' => {
                if !inside_sb {
                    mdbook_output.push(ch);
                } else {
                    mdbook_output.push_str("&nbsp;");
                }
            }
            ']' => {
                inside_sb = false;
                mdbook_output.push(ch);
            }
            _ => {
                mdbook_output.push(ch);
            }
        }
    }

    fs::write(file_name, mdbook_output).unwrap();
}

fn print_results(
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    output: OutputType,
    file_name: &str,
) {
    // compute the times
    for result in results.iter_mut() {
        let a = compute_test_times(&mut result.times_se);
        let b = compute_test_times(&mut result.times_de);
        let c = compute_test_times(&mut result.times_se_de);
        result.time_se_ms = a.to_string();
        result.time_de_ms = b.to_string();
        result.time_se_de_ms = c.to_string();
        result.compare_key = c.median.as_nanos();
    }

    results.sort_by(|x, y| {
        x.top_test_name
            .cmp(&y.top_test_name)
            .then(x.compare_key.cmp(&y.compare_key))
    });

    let colums = [
        ("top name", Align::Left),
        ("schema", Align::Center),
        ("name", Align::Left),
        ("size (b)", Align::Right),
        ("se time (ms)", Align::Right),
        ("de time (ms)", Align::Right),
        ("se + de time (ms)", Align::Right),
    ];

    let mut r: Vec<[&dyn Display; 7]> = Vec::new();
    let mut last = None;

    let dashes: [&dyn Display; 7] = [&"---", &"---", &"---", &"---", &"---", &"---", &"---"];

    let one_algo = if all_algos { false } else { algos.len() == 1 };

    for i in results.iter_mut() {
        let current = Some(&i.top_test_name);
        if !last.is_none() && last != current && !one_algo {
            r.push(dashes);
        }
        last = current;

        i.size_repr = if i.min_size > 0 {
            let proc = (i.size * 100 / i.min_size) as i32 - 100;
            format!("{} [{:>+5}%]", i.size, proc)
        } else {
            format!("{} [-----%]", i.size)
        };

        let ch = if i.needs_schema { &'*' } else { &' ' };
        r.push([
            i.top_test_name.display(),
            ch,
            i.name.display(),
            &i.size_repr,
            &i.time_se_ms,
            &i.time_de_ms,
            &i.time_se_de_ms,
        ]);
    }

    match output {
        OutputType::Ascii => {
            print_results_ascii_table(&r, &colums, file_name);
        }
        OutputType::Markdown => {
            print_results_markdown(&r, &colums, file_name);
        }
        OutputType::Mdbook => {
            print_results_mdbook(&r, &colums, file_name);
        }
    }
}

fn do_one<'a, T: FlatMessageOwned + Clone + Serialize + DeserializeOwned + GetSize>(
    top_test_name: TestKind,
    x: &T,
    results: &mut Vec<Result>,
    algos: &HashSet<AlgoKind>,
    all_algos: bool,
    iterations: u32,
) {
    add_benches(top_test_name, x, results, algos, all_algos, iterations);
}

macro_rules! tests {
    ($enum_name:ident, $(($name:literal, $v:ident)),+) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
        enum $enum_name {
            $(
                $v,
            )+
        }
        impl $enum_name {
            // fn name(self) -> &'static str {
            //     match self {
            //         $(
            //             Self::$v => $name,
            //         )+
            //     }
            // }
            fn display(self) -> &'static dyn Display {
                match self {
                    $(
                        Self::$v => &$name,
                    )+
                }
            }
            fn all() -> &'static [&'static str] {
                &[
                    $(
                        $name,
                    )+
                ]
            }
        }
        impl From<&str> for $enum_name {
            fn from(value: &str) -> Self {
                match value {
                    $(
                        $name => Self::$v,
                    )+
                    _ => panic!("unknown option: {}\navailable option: {}", value, Self::all().join(", ")),
                }
            }
        }
    };
}

tests! {
    TestKind,
    ("process_create", ProcessCreate),
    ("long_strings", LongStrings),
    ("point", Point),
    ("multiple_fields", MultipleFields),
    ("multiple_integers", MultipleIntegers),
    ("multiple_bools", MultipleBools),
    ("vectors", Vectors),
    ("large_vectors", LargeVectors),
    ("enum_fields", EnumFields),
    ("enum_lists", EnumLists),
    ("small_enum_lists", SmallEnumLists),
    ("strings_lists", StringLists),
    ("one_bool", OneBool)
}

tests! {
    AlgoKind,
    ("flat_message", FlatMessage),
    ("flat_message_unchecked", FlatMessageUnchecked),
    ("rmp_schema", RmpSchema),
    ("rmp_schemaless", RmpSchemaless),
    ("bincode", Bincode),
    ("flexbuffers" , FlexBuffers),
    ("cbor", Cbor),
    ("bson", Bson),
    ("json", Json),
    ("simd_json", SimdJson),
    ("postcard", Postcard)
}

fn split_tests<'x, T>(input: &'x str) -> (bool, HashSet<T>)
where
    T: From<&'x str> + Eq + Hash,
{
    if input == "all" {
        (true, HashSet::new())
    } else {
        let tests = input.split(',').map(|x| T::from(x)).collect();
        (false, tests)
    }
}

#[derive(Subcommand, Debug, Default)]
#[command(subcommand_precedence_over_arg = true)]
enum Commands {
    Run,
    #[default]
    Help,
    ListAlgos,
    ListTests,
}

#[derive(Debug, Default, Copy, Clone, ValueEnum)]
enum OutputType {
    #[default]
    Ascii,
    Markdown,
    Mdbook,
}

#[derive(clap::Parser)]
#[command(
    name = "benchmarks",
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, default_value_t = 1_000_000, global = true)]
    times: u32,
    #[arg(long, default_value = "all", global = true)]
    tests: String,
    #[arg(long, default_value = "all", global = true)]
    algos: String,
    #[arg(long, default_value_t = false, global = true)]
    names: bool,
    #[arg(long, default_value_t = 10, global = true)]
    iterations: u32,
    #[arg(long, value_enum, default_value_t = OutputType::Ascii, global = true)]
    output: OutputType,
    #[arg(long, default_value = "result.md", global = true)]
    file_name: String,
}

fn run_tests(args: Args) {
    let (all_tests, tests) = split_tests::<TestKind>(&args.tests);
    let (all_algos, algos) = split_tests(&args.algos);

    let results = &mut Vec::new();
    macro_rules! run {
        ($name:expr, $x:expr) => {
            if all_tests || tests.contains(&$name) {
                do_one($name, $x, results, &algos, all_algos, args.times);
            }
        };
    }

    use std::io::{self, Write};
    use TestKind::*;
    for i in 0..args.iterations {
        print!("Running iteration {}/{}", i + 1, args.iterations);
        io::stdout().flush().unwrap();
        let start = Instant::now();
        {
            let process_small = structures::process_create::generate_flat();
            run!(ProcessCreate, &process_small);
        }
        {
            let s = structures::long_strings::generate(100);
            run!(LongStrings, &s);
        }
        {
            let s = structures::point::generate();
            run!(Point, &s);
        }
        {
            let s = structures::one_bool::generate();
            run!(OneBool, &s);
        }
        {
            let s = structures::multiple_fields::generate();
            run!(MultipleFields, &s);
        }
        {
            let s = structures::multiple_integers::generate();
            run!(MultipleIntegers, &s);
        }
        {
            let s = structures::vectors::generate();
            run!(Vectors, &s);
        }
        {
            let s = structures::large_vectors::generate();
            run!(LargeVectors, &s);
        }
        {
            let s = structures::enum_fields::generate();
            run!(EnumFields, &s);
        }
        {
            let s = structures::enum_lists::generate();
            run!(EnumLists, &s);
        }
        {
            let s = structures::small_enum_lists::generate();
            run!(SmallEnumLists, &s);
        }
        {
            let s = structures::multiple_bools::generate();
            run!(MultipleBools, &s);
        }
        {
            let s = structures::string_lists::generate();
            run!(StringLists, &s);
        }
        println!(" done in {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    }

    print_results(results, &algos, all_algos, args.output, &args.file_name);
}
fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Run => {
            run_tests(args);
        }
        Commands::Help => {
            println!("usage: {} <command>", env!("CARGO_BIN_NAME"));
        }
        Commands::ListAlgos => {
            println!("available algos: {}", AlgoKind::all().join(", "));
        }
        Commands::ListTests => {
            println!("available tests: {}", TestKind::all().join(", "));
        }
    }
}
