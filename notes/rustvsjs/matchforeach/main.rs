use std::{env, fs, io, process};
use std::io::Write;
use std::str::FromStr;

extern crate regex;
use regex::Regex;

extern crate time;
use time::OffsetDateTime;

extern crate sophia;
use sophia::dataset::*;
use sophia::dataset::inmem::*;
use sophia::ns::rdf;
use sophia::parser::nq;
use sophia::quad::stream::*;
use sophia::term::Term;
use sophia::btreeddataset::TreedDataset;
use sophia::fulldataset::FullIndexDataset;
use std::rc::Rc;


fn get_vmsize() -> usize {
    let status = fs::read_to_string("/proc/self/status").unwrap();
    let vmsize_re = Regex::new(r"VmSize:\s*([0-9]+) kB").unwrap();
    let vmsize = vmsize_re.captures(&status).unwrap().get(1).unwrap().as_str();
    usize::from_str(vmsize).unwrap()
}

fn task_query<R> (f: R, variant: Option<&str>, query_num: usize) where
    R: io::BufRead,
{
    if query_num == 1 {
        eprintln!("task    : query");
    } else {
        eprintln!("task    : query{}", query_num);
    }

    match variant {
        None => task_query_g(f, FastDataset::new(), query_num),
        Some("light") => task_query_g(f, LightDataset::new(), query_num),
        Some("tree") => task_query_g(f, TreedDataset::new(), query_num),
        Some("full") => task_query_g(f, FullIndexDataset::new(), query_num),
        Some(v) => {
            eprintln!("Unknown variant {}", v);
            process::exit(1);
        }
    }
}

fn request<G> (g: &G, query_num: usize) -> (f64, f64, usize) where G: MutableDataset {
    let mut time_first: f64 = 0.0;
    let time_rest;
    let dbo_person = Term::<&'static str>::new_iri("http://dbpedia.org/ontology/Person").unwrap();
    let dbr_vincent = Term::<&'static str>::new_iri("http://dbpedia.org/resource/Vincent_Descombes_Sevoie").unwrap();
    let none: Option<&Term<Rc<str>>> = None;

    let mut t0 = OffsetDateTime::now_utc();
    let results = match query_num {
        1 => g.quads_with_pog(&rdf::type_, &dbo_person, none),
        3 => g.quads_with_po(&rdf::type_, &dbo_person),
        2 => g.quads_with_sg(&dbr_vincent, none),
        4 => g.quads_with_s(&dbr_vincent),
        _ => panic!("Unknown request")
    };

    let mut c = 0;
    for _ in results {
        if c == 0 {
            let t1 = OffsetDateTime::now_utc();
            time_first = (t1-t0).as_seconds_f64();
            t0 = OffsetDateTime::now_utc();
        }
        c += 1;
    }
    let t1 = OffsetDateTime::now_utc();
    time_rest = (t1-t0).as_seconds_f64();

    (time_first, time_rest, c)
}

fn task_query_g<G, R> (f: R, mut g: G, query_num: usize) where
    R: io::BufRead,
    G: MutableDataset,
{
    let m0 = get_vmsize();
    let t0 = OffsetDateTime::now_utc();
    nq::parse_bufread(f).in_dataset(&mut g).expect("Error parsing NT file");
    let t1 = OffsetDateTime::now_utc();
    let m1 = get_vmsize();
    let time_parse = (t1-t0).as_seconds_f64();
    let mem_graph = m1-m0;
    eprintln!("loaded  : ~ {:?} triples\n", g.quads().size_hint());

    let (time_first, time_rest, c) = request(&g, query_num);

    eprintln!("matching triple: {}\n", c);

    let (time_second, time_rest_second, _) = request(&g, query_num);

    let end_mem = get_vmsize() - m0;

    println!("{},{},{},{},{},{},{}", time_parse, mem_graph, end_mem, time_first, time_rest, time_second, time_rest_second);
}

fn task_parse<T: io::BufRead> (f: T, variant: Option<&str>) {
    eprintln!("task    : parse");
    match variant {
        None => {
            task_parse_nt(f);
        }
        Some("nt") => {
            task_parse_nt(f);
        }
        Some(v) => {
            eprintln!("Unknown variant {}", v);
            process::exit(1);
        }
    };
}

fn task_parse_nt<T: io::BufRead> (f: T) {
    let t0 = OffsetDateTime::now_utc();
    // nq::parse_bufread(f).for_each_quads(|_| ()).expect("Error parsing NT file");
    let t1 = OffsetDateTime::now_utc();
    let time_parse = (t1-t0).as_seconds_f64();
    println!("{}", time_parse);
}


fn main() {
    eprintln!("program : sophia");
    eprintln!("pid     : {}", process::id());
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        io::stderr().write(b"usage: sophia_benchmark <task> <filename.nt>\n").unwrap();
        process::exit(1);
    }
    let task_id: &str = &args[1];
    let filename = &args[2];
    let variant = if args.len() > 3 {
        Some(&args[3] as &str)
    } else {
        None
    };
    eprintln!("filename: {}", filename);
    let f = fs::File::open(&filename).expect("Error opening file");
    let f = io::BufReader::new(f);
    match task_id {
        "parse"  => task_parse(f, variant),
        "query" => task_query(f, variant, 1),
        "query2" => task_query(f, variant, 2),
        "query3" => task_query(f, variant, 3),
        "query4" => task_query(f, variant, 4),
        _   => {
            eprint!("Unknown task {}", task_id);
            process::exit(1);
        }
    };
}
