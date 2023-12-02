extern crate proc_macro;
use std::fs::read_to_string;

use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, Token};
use syn::{parse_macro_input, Ident, LitInt};

struct TestMacroInput {
    day: LitInt,
    result1: LitInt,
    result2: LitInt,
}

impl Parse for TestMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let day = input.parse()?;
        input.parse::<Token![,]>()?;
        let result1 = input.parse()?;
        input.parse::<Token![,]>()?;
        let result2 = input.parse()?;
        Ok(Self {
            day,
            result1,
            result2,
        })
    }
}

/// takes as parameter the current day, the correct answer of part 1, the correct answer of part 2
#[proc_macro]
pub fn test_parts(item: TokenStream) -> TokenStream {
    let input: TestMacroInput = parse_macro_input!(item as TestMacroInput);
    let module = format!("day_{}", input.day.base10_parse::<usize>().unwrap());
    let file_content = read_to_string(format!("src/{}.rs", module)).unwrap();
    let part1_fn_name = file_content
        .lines()
        .filter_map(|l| {
            if l.starts_with("pub fn part1") {
                Some(
                    l.strip_prefix("pub fn")
                        .unwrap()
                        .trim()
                        .split_once('(')
                        .unwrap()
                        .0,
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let part1_fn = part1_fn_name
        .iter()
        .map(|n| Ident::new(n, Span::call_site()));
    let part1_test_fn = part1_fn_name
        .iter()
        .map(|n| Ident::new(&format!("test_{n}"), Span::call_site()));

    let part2_fn_name = file_content
        .lines()
        .filter_map(|l| {
            if l.starts_with("pub fn part2") {
                Some(
                    l.strip_prefix("pub fn")
                        .unwrap()
                        .trim()
                        .split_once('(')
                        .unwrap()
                        .0,
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let part2_fn = part2_fn_name
        .iter()
        .map(|n| Ident::new(n, Span::call_site()));
    let part2_test_fn = part2_fn_name
        .iter()
        .map(|n| Ident::new(&format!("test_{n}"), Span::call_site()));

    let result1 = input.result1;
    let result2 = input.result2;
    let input_file = format!("../inputs/{module}.txt");

    TokenStream::from(quote! {
        #(
        #[test]
        fn #part1_test_fn() {
            assert_eq!(#part1_fn(&generator(include_str!(#input_file))), #result1);
        }
        )*

        #(
        #[test]
        fn #part2_test_fn() {
            assert_eq!(#part2_fn(&generator(include_str!(#input_file))), #result2);
        }
    )*
    })
}

/// takes as parameter the day to benchmark
#[proc_macro]
pub fn benchmark(item: TokenStream) -> TokenStream {
    let input: usize = parse_macro_input!(item as LitInt).base10_parse().unwrap();
    let day = format!("day_{input}");
    let file_name = format!("src/{day}.rs");
    let module_name = Ident::new(&day, Span::call_site());
    let content = read_to_string(file_name).expect("can't read file");

    // TODO use nom to parse the content of the file
    // we are interested in 3 types of functions:
    // - generator
    // - part1
    // - part2
    let mut generator_functions = vec![];
    let mut generator_functions_name = vec![];
    let mut part1_functions = vec![];
    let mut part1_functions_name = vec![];
    let mut part2_functions = vec![];
    let mut part2_functions_name = vec![];

    for l in content.lines() {
        if !l.starts_with("pub fn") {
            continue;
        }
        let fonc_name = l
            .strip_prefix("pub fn")
            .unwrap()
            .trim()
            .split_once('(')
            .unwrap()
            .0;
        let fonc = Ident::new(fonc_name, Span::call_site());

        if fonc_name.contains("part1") {
            part1_functions_name.push(fonc_name);
            part1_functions.push(fonc);
        } else if fonc_name.contains("part2") {
            part2_functions_name.push(fonc_name);
            part2_functions.push(fonc);
        } else if fonc_name.contains("generator") {
            generator_functions_name.push(fonc_name);
            generator_functions.push(fonc);
        }
    }

    println!("{module_name}");
    println!("{generator_functions:?}");
    println!("{part1_functions:?}");
    println!("{part2_functions:?}");

    let input_data_file = format!("../inputs/{day}.txt");
    TokenStream::from(quote! {
        use criterion::{black_box, criterion_group, criterion_main, Criterion};
        // use criterion_cycles_per_byte::CyclesPerByte;
        // use criterion_perf_events::Perf;
        // use perfcnt::linux::HardwareEventType as Hardware;
        // use perfcnt::linux::SoftwareEventType as Software;
        // use perfcnt::linux::PerfCounterBuilderLinux as Builder;
        use aoc::#module_name::*;
        const DATA: &str = include_str!(#input_data_file);

        fn criterion_benchmark(c: &mut Criterion){ //<Perf>) {
            let input = generator(DATA);
            println!("with Perf hadware event");

            let mut group = c.benchmark_group("generator");
            #(group.bench_function(format!("{}", #generator_functions_name), |b| b.iter(|| #generator_functions(black_box(DATA))));)*
            group.finish();

            let mut group = c.benchmark_group("part1");
            #(group.bench_function(format!("{}", #part1_functions_name), |b| b.iter(|| #part1_functions(black_box(&input))));)*
            group.finish();

            let mut group = c.benchmark_group("part2");
            #(group.bench_function(format!("{}", #part2_functions_name), |b| b.iter(|| #part2_functions(black_box(&input))));)*
            group.finish();
        }


        criterion_group!(benches, criterion_benchmark);
        // criterion_group!(
        //     name = benches;
        //     config = Criterion::default().with_measurement(Perf::new(Builder::from_software_event(Software::PageFaults)));
        //     targets = criterion_benchmark
        // );
        criterion_main!(benches);
    })
}

/// takes as parameter number of days implemented
/// This macro can be used in the lib.is to import all day modules
#[proc_macro]
pub fn declare_mods(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as LitInt);
    let input = input.base10_parse::<u16>().unwrap() + 1;
    let mod_names = (1..input)
        .map(|v| (Ident::new(&format!("day_{v}"), Span::call_site())))
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        #(pub mod #mod_names;)*
    })
}

/// takes as parameter number of days implemented
/// This macro can be used in the main.rs to implement the main function
#[proc_macro]
pub fn main(item: TokenStream) -> TokenStream {
    let input: LitInt = parse_macro_input!(item);
    let input = input.base10_parse::<usize>().unwrap();

    let days = (1..input + 1).collect::<Vec<_>>();
    let inputs = (1..input + 1)
        .map(|v| format!("../inputs/day_{v}.txt"))
        .collect::<Vec<_>>();
    let mod_names = (1..input + 1)
        .map(|v| (Ident::new(&format!("day_{v}"), Span::call_site())))
        .collect::<Vec<_>>();

    let expanded = quote! {
        // const DAYS: &[&str] = &[#(#days),*];
        const INPUTS : &[&str] = &[#(include_str!(#inputs)),*];
        #(pub mod #mod_names;)* // TODO can probably be removed

        fn main() {
            let args: Args = argh::from_env();

            let (elapsed, _) = time(&|| {
                let days = match args.day {
                    Some(day) => {
                        assert!(day <= #input, "Requested an unimplemented day");
                        vec![day]
                    },
                    None => (1..#input + 1).collect()
                };

                for day in days.into_iter() {
                    match day {
                        #(#days => {
                            let data = INPUTS[day as usize - 1];

                            let (gen_elapsed, input) = time(&|| #mod_names::generator(&data));
                            let (p1_elapsed, p1_result) = time(&|| #mod_names::part1(&input));
                            let (p2_elapsed, p2_result) = time(&|| #mod_names::part2(&input));

                            let duration = format!("({:.2?})", gen_elapsed + p1_elapsed + p2_elapsed);
                            println!("{} {}", format!("Day {}", day).bold(), duration.dimmed());
                            pretty_print(" · Generator", None, gen_elapsed);
                            pretty_print(" · Part 1", Some(&format!("{}", p1_result)), p1_elapsed);
                            pretty_print(" · Part 2", Some(&format!("{}", p2_result)), p2_elapsed);

                            // Break up whatever comes after us
                            println!()
                        },)*
                        _ => unreachable!() // All the days should've been hit by the match
                    }
                }
            });

            println!("{} {}", "Overall runtime".bold(), format!("({:.2?})", elapsed).dimmed());
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn p(item: TokenStream) -> TokenStream {
    let input: Ident = parse_macro_input!(item as Ident);
    let input_name = input.to_string();

    TokenStream::from(quote! {
        println!("{}: {:?}", #input_name, #input)
    })
}

#[proc_macro]
pub fn pp(item: TokenStream) -> TokenStream {
    let input: Ident = parse_macro_input!(item as Ident);
    let input_name = input.to_string();

    TokenStream::from(quote! {
        println!("{}: {:#?}", #input_name, #input)
    })
}

#[proc_macro]
pub fn binp(item: TokenStream) -> TokenStream {
    let input: Ident = parse_macro_input!(item as Ident);
    let input_name = input.to_string();

    TokenStream::from(quote! {
        println!("{}: {:b}", #input_name, #input)
    })
}
