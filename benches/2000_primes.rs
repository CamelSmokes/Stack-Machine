use criterion::{criterion_group, criterion_main, Criterion};
use stack_machine::{
    interpreter::{Interpreter, InterpreterEvent},
    parser,
};
const PRIME_FINDER: [u8; 210] = [
    40, 108, 0, 0, 0, 0, 0, 0, 0, 6, 22, 32, 11, 33, 1, 22, 18, 18, 15, 32, 14, 40, 92, 0, 0, 0, 0,
    0, 0, 0, 13, 8, 11, 21, 18, 18, 9, 15, 32, 14, 40, 92, 0, 0, 0, 0, 0, 0, 0, 13, 10, 32, 14, 40,
    77, 0, 0, 0, 0, 0, 0, 0, 13, 33, 1, 2, 40, 15, 0, 0, 0, 0, 0, 0, 0, 6, 7, 22, 1, 1, 1, 32, 40,
    148, 0, 0, 0, 0, 0, 0, 0, 6, 22, 1, 1, 1, 33, 1, 40, 148, 0, 0, 0, 0, 0, 0, 0, 6, 22, 33, 1,
    32, 12, 33, 2, 33, 1, 12, 33, 3, 22, 32, 11, 34, 207, 7, 15, 40, 208, 0, 0, 0, 0, 0, 0, 0, 13,
    8, 40, 10, 0, 0, 0, 0, 0, 0, 0, 6, 22, 40, 169, 0, 0, 0, 0, 0, 0, 0, 13, 40, 194, 0, 0, 0, 0,
    0, 0, 0, 6, 22, 32, 11, 33, 1, 2, 8, 32, 12, 18, 9, 12, 33, 2, 2, 40, 120, 0, 0, 0, 0, 0, 0, 0,
    6, 22, 33, 2, 2, 40, 120, 0, 0, 0, 0, 0, 0, 0, 6, 22, 7,
];
pub fn benchmark_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_sample_size");
    group.sample_size(20);
    group.bench_function("Find 2000 primes", |b| {
        b.iter(|| {
            let parsed = parser::parse_bytes_to_instructions(&PRIME_FINDER).unwrap();
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
