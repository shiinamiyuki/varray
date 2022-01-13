#![feature(test)]
extern crate test;
use region::Protection;
use test::Bencher;

#[bench]
fn bench_varray(b: &mut Bencher) {
    use varray::VArrayMemBuilder;
    let mut vmem_builder = VArrayMemBuilder::new(1 * 1 << 20, 4 * 1024);

    let mut arrs = vec![];
    let mut total_mem = 0;
    for _ in 0..8 {
        let m = 1024 * 256;
        arrs.push(vmem_builder.allocate(&vec![1.0f32; m]));
        total_mem += m;
    }
    let vmem = vmem_builder.build();
    let mut i = 0usize;
    b.iter(|| {
        let mut s = 0.0;
        for _ in 0..1024 {
            let j = i / arrs[0].len();
            s += arrs[j].read(i % arrs[0].len());
            i += 1;
            i %= total_mem;
        }
        s
    });
}

#[bench]
fn bench_vmem(b: &mut Bencher) {
    let total_mem = 1024 * 1024 * 1024 * 16;
    let mut alloc = region::alloc(total_mem, Protection::READ_WRITE).unwrap();
    let mut slice = unsafe {
        std::slice::from_raw_parts_mut(
            alloc.as_mut_ptr::<f32>(),
            alloc.len() / std::mem::size_of::<f32>(),
        )
    };
    slice.fill(1.0f32);
    let mut i = 0usize;
    b.iter(|| {
        let mut s = 0.0;
        for _ in 0..1024 {
            s += slice[i];
            i += 1;
            i %= total_mem;
        }
        s
    });
}
