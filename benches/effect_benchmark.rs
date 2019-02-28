#[macro_use]
extern crate criterion;

use criterion::*;

extern crate audio_adaptive;

extern crate rand;

use rand::prelude::*;
use rand::distributions::Uniform;

use audio_adaptive::audiograph::*;
use audio_adaptive::samplerate;

fn osc_bench(c : &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1.,1.);
    c.bench_function("osc",  move |b| {
        let mut osc = Oscillator::new(0., 440, 1.);
        let mut input = vec![DspEdge::new(1, 1, 256, 44100);1];
        let size = input[0].buffer().len();
        input[0].buffer_mut().copy_from_slice(&rng.sample_iter(&unity_interval).take(size).collect::<Vec<f32>>());
        b.iter( ||
            {
                let mut output = vec![DspEdge::new(1, 1, 256, 44100);1];
                osc.process(&input, &mut output)}
        )});
}

fn mod_bench(c : &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1.,1.);
    c.bench_function("mod",  move |b| {
        let mut modu = Modulator::new(0., 440, 1.);
        let mut input = vec![DspEdge::new(1, 1, 256, 44100);1];
        let size = input[0].buffer().len();
        input[0].buffer_mut().copy_from_slice(&rng.sample_iter(&unity_interval).take(size).collect::<Vec<f32>>());
        b.iter( ||
            {
                let mut output = vec![DspEdge::new(1, 1, 256, 44100);1];
                modu.process(&input, &mut output)}
            )});
}

fn resampler_bench(c : &mut Criterion) {
    let parameters = vec![samplerate::ConverterType::SincBestQuality, samplerate::ConverterType::SincMediumQuality,
    samplerate::ConverterType::SincFastest, samplerate::ConverterType::ZeroOrderHold,
    samplerate::ConverterType::Linear];
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1.,1.);
    c.bench(
        "resampler",
        ParameterizedBenchmark::new(
            "resampler",
            move |b, conv_type| {
                let mut conv = Resampler::new(conv_type.clone(), 0.5);
                let mut input = vec![DspEdge::new(1, 1, 256, 44100);1];
                let size = input[0].buffer().len();
                input[0].buffer_mut().copy_from_slice(&rng.sample_iter(&unity_interval).take(size).collect::<Vec<f32>>());
                b.iter(|| {
                    let mut output = vec![DspEdge::new(1, 1, 128, 44100);1];
                    conv.process(&input, &mut output)
            })},
            parameters
        )
    );
}

fn mixer_bench(c : &mut Criterion) {
    let n = 6;
    let mut parameters = Vec::with_capacity(n*n);
    for i in 1..n {
        for j in 1..n {
            if i % j == 0 || j % i == 0 {parameters.push((j,i));}
        }
    }
    let mut rng = SmallRng::seed_from_u64(345987);
    let unity_interval = Uniform::new_inclusive(-1.,1.);
    c.bench(
        "mixer",
        ParameterizedBenchmark::new(
            "mixer",
            move |b, (nb_inlets, nb_outlets)| {
                let mut conv = InputsOutputsAdaptor::new(*nb_inlets, *nb_outlets);
                let mut inputs = vec![DspEdge::new(1, 1, 256, 44100);*nb_inlets];
                let size = inputs[0].buffer().len();
                for input in inputs.iter_mut() {
                    input.buffer_mut().copy_from_slice(&rng.sample_iter(&unity_interval).take(size).collect::<Vec<f32>>());
                }
                b.iter(|| {
                    let mut outputs = vec![DspEdge::new(1, 1, 256, 44100);*nb_outlets];
                    conv.process(&inputs, &mut outputs)
            })},
            parameters
        )
    );
}



criterion_group!(benches, osc_bench, mod_bench, resampler_bench, mixer_bench);
criterion_main!(benches);
