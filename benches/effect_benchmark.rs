#[macro_use]
extern crate criterion;

use criterion::*;

extern crate audio_adaptive;

extern crate rand;

use rand::distributions::Uniform;
use rand::prelude::*;

use audio_adaptive::audiograph::*;
use audio_adaptive::faust_effect::*;
use audio_adaptive::samplerate;

fn osc_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "osc",
        move |b: &mut Bencher, n: &usize| {
            let mut osc = Oscillator::new(0., 440, 1.);
            let mut input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            //let size = input[0].buffer().len();
            input[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                osc.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn mod_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "mod",
        move |b: &mut Bencher, n: &usize| {
            let mut modu = Modulator::new(0., 440, 1.);
            let mut input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            //let size = input[0].buffer().len();
            input[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                modu.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn resampler_bench(c: &mut Criterion) {
    let parameters = vec![
        samplerate::ConverterType::SincBestQuality,
        samplerate::ConverterType::SincMediumQuality,
        samplerate::ConverterType::SincFastest,
        samplerate::ConverterType::ZeroOrderHold,
        samplerate::ConverterType::Linear,
    ];
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench(
        "resampler",
        ParameterizedBenchmark::new(
            "resampler",
            move |b, conv_type| {
                let mut conv = Resampler::new(conv_type.clone(), 0.5);
                let mut input = vec![DspEdge::new(1, 1, 256, 44100); 1];
                let size = input[0].buffer().len();
                input[0].buffer_mut().copy_from_slice(
                    &rng.sample_iter(&unity_interval)
                        .take(size)
                        .collect::<Vec<f32>>(),
                );
                b.iter(|| {
                    let mut output = vec![DspEdge::new(1, 1, 128, 44100); 1];
                    conv.process(&input, &mut output)
                })
            },
            parameters,
        ),
    );
}

fn mixer_bench(c: &mut Criterion) {
    let n = 6;
    let mut parameters = Vec::with_capacity(n * n);
    for i in 1..n {
        for j in 1..n {
            if i % j == 0 || j % i == 0 {
                parameters.push((j, i));
            }
        }
    }
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench(
        "mixer",
        ParameterizedBenchmark::new(
            "mixer",
            move |b, (nb_inlets, nb_outlets)| {
                let mut conv = InputsOutputsAdaptor::new(*nb_inlets, *nb_outlets);
                let mut inputs = vec![DspEdge::new(1, 1, 256, 44100); *nb_inlets];
                let size = inputs[0].buffer().len();
                for input in inputs.iter_mut() {
                    input.buffer_mut().copy_from_slice(
                        &rng.sample_iter(&unity_interval)
                            .take(size)
                            .collect::<Vec<f32>>(),
                    );
                }
                b.iter(|| {
                    let mut outputs = vec![DspEdge::new(1, 1, 256, 44100); *nb_outlets];
                    conv.process(&inputs, &mut outputs)
                })
            },
            parameters,
        ),
    );
}

fn guitar_bench(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "guitar",
        move |b: &mut Bencher, n: &usize| {
            let mut guitar = Guitar::new(1., 0.45, 0.9, 1);
            let input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                guitar.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn transpose_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "transpose",
        move |b: &mut Bencher, n: &usize| {
            let mut transposer = Transposer::new(11);
            let mut input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            //let size = input[0].buffer().len();
            input[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                transposer.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn zita_reverb_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "zita_reverb",
        move |b: &mut Bencher, n: &usize| {
            let mut transposer = ZitaReverb::new(10., 300, 10000, 6., 10., 96_200);
            let mut inputs = vec![DspEdge::new(1, 1, *n, 44100); 2];
            //let size = input[0].buffer().len();
            inputs[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            inputs[1].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 2];
                transposer.process(&inputs, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn freeverb_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "freeverb",
        move |b: &mut Bencher, n: &usize| {
            let mut freeverb = MonoFreeverb::new(0.2, 0.6, 0.8, 0.4);
            let mut input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            //let size = input[0].buffer().len();
            input[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                freeverb.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn compressor_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "compressor",
        move |b: &mut Bencher, n: &usize| {
            let mut compressor = Compressor::new(2., 20., 0.5, 1.);
            let mut input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            //let size = input[0].buffer().len();
            input[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                compressor.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn autowah_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "autowah",
        move |b: &mut Bencher, n: &usize| {
            let mut autowah = Autowah::new(0.9);
            let mut input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            //let size = input[0].buffer().len();
            input[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                autowah.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}

fn cubicnl_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1., 1.);
    c.bench_function_over_inputs(
        "cubicnl",
        move |b: &mut Bencher, n: &usize| {
            let mut cubicnl = Cubicnl::new(0.9, 2.1);
            let mut input = vec![DspEdge::new(1, 1, *n, 44100); 1];
            //let size = input[0].buffer().len();
            input[0].buffer_mut().copy_from_slice(
                &rng.sample_iter(&unity_interval)
                    .take(*n)
                    .collect::<Vec<f32>>(),
            );
            b.iter(|| {
                let mut output = vec![DspEdge::new(1, 1, *n, 44100); 1];
                cubicnl.process(&input, &mut output)
            })
        },
        vec![64, 128, 256, 512, 1024, 2048, 4096],
    );
}


criterion_group!(
    benches,
    osc_bench,
    mod_bench,
    resampler_bench,
    mixer_bench,
    guitar_bench,
    transpose_bench,
    zita_reverb_bench,
    freeverb_bench,
    compressor_bench,
    autowah_bench,
    cubicnl_bench
);
criterion_main!(benches);
