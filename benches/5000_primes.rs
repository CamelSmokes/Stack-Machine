use criterion::{criterion_group, criterion_main, Criterion};
use stack_machine::{
    interpreter::{Interpreter, InterpreterEvent},
    parser,
};
const PRIME_FINDER: [u8; 316] = [
    0, 104, 0, 0, 0, 0, 0, 0, 0, 6, 32, 11, 33, 1, 18, 18, 15, 32, 14, 0, 89, 0, 0, 0, 0, 0, 0, 0,
    13, 8, 11, 21, 18, 18, 9, 15, 32, 14, 0, 89, 0, 0, 0, 0, 0, 0, 0, 13, 10, 32, 14, 0, 75, 0, 0,
    0, 0, 0, 0, 0, 13, 33, 1, 2, 0, 14, 0, 0, 0, 0, 0, 0, 0, 6, 7, 1, 1, 1, 32, 0, 148, 0, 0, 0, 0,
    0, 0, 0, 6, 1, 1, 1, 33, 1, 0, 148, 0, 0, 0, 0, 0, 0, 0, 6, 33, 1, 32, 12, 33, 2, 33, 1, 12,
    33, 3, 32, 11, 0, 135, 19, 0, 0, 0, 0, 0, 0, 15, 0, 205, 0, 0, 0, 0, 0, 0, 0, 13, 8, 0, 10, 0,
    0, 0, 0, 0, 0, 0, 6, 0, 168, 0, 0, 0, 0, 0, 0, 0, 13, 0, 192, 0, 0, 0, 0, 0, 0, 0, 6, 32, 11,
    33, 1, 2, 8, 32, 12, 18, 9, 12, 33, 2, 2, 0, 115, 0, 0, 0, 0, 0, 0, 0, 6, 33, 2, 2, 0, 115, 0,
    0, 0, 0, 0, 0, 0, 6, 0, 1, 0, 0, 0, 0, 0, 0, 0, 253, 0, 70, 0, 0, 0, 0, 0, 0, 0, 254, 0, 105,
    0, 0, 0, 0, 0, 0, 0, 254, 0, 110, 0, 0, 0, 0, 0, 0, 0, 254, 0, 105, 0, 0, 0, 0, 0, 0, 0, 254,
    0, 115, 0, 0, 0, 0, 0, 0, 0, 254, 0, 104, 0, 0, 0, 0, 0, 0, 0, 254, 0, 101, 0, 0, 0, 0, 0, 0,
    0, 254, 0, 100, 0, 0, 0, 0, 0, 0, 0, 254, 0, 10, 0, 0, 0, 0, 0, 0, 0, 254, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 253, 7,
];
pub fn benchmark_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_sample_size");
    group.sample_size(10);
    group.bench_function("Find 5000 primes", |b| {
        b.iter(|| {
            let parsed = parser::parse_bytes_to_instructions(&PRIME_FINDER);
            let mut interpreter = Interpreter::new(parsed);
            loop {
                let res = interpreter.next_instruction();
                match res {
                    Ok(InterpreterEvent::ProgramEnd) => break,
                    Ok(_) => {
                        // println!("Result: {r:?}");
                        // if !silent_toggle {
                        //     println!("Stack: {:?}", interpreter.debug_get_stack());
                        // }
                    }
                    Err(e) => panic!("{e:?}"),
                }
            }
        });
    });
    group.finish();
}

criterion_group!(benches, benchmark_primes);
criterion_main!(benches);
