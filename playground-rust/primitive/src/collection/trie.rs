struct Solution;

fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>(); // Okay: `M` is a const parameter
    foo::<2021>(); // Okay: `2021` is a literal
    foo::<{ 20 * 100 + 20 * 10 + 1 }>(); // Okay: const expression contains no generic parameters

    let _: [u8; M]; // Okay: `M` is a const parameter
}
