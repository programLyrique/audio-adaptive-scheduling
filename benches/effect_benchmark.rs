#[macro_use]
extern crate criterion;

use criterion::*;

extern crate audio_adaptive;

use audio_adaptive::audiograph::*;
use audio_adaptive::samplerate;

fn osc_bench(c : &mut Criterion) {
    c.bench_function("osc",  |b| {
        let mut osc = Oscillator::new(0., 440, 1.);
        let input = vec![DspEdge::new(1, 1, 256, 44100);1];
        b.iter( ||
            {
                let mut output = vec![DspEdge::new(1, 1, 256, 44100);1];
                osc.process(&input, &mut output)}
        )});
}

fn mod_bench(c : &mut Criterion) {
    c.bench_function("mod",  |b| {
        let mut modu = Modulator::new(0., 440, 1.);
        let input = vec![DspEdge::new(1, 1, 256, 44100);1];
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
    c.bench(
        "resampler",
        ParameterizedBenchmark::new(
            "my_function",
            move |b, conv_type| {
                let mut conv = Resampler::new(conv_type.clone(), 0.5);
                let input = vec![DspEdge::new(1, 1, 256, 44100);1];
                b.iter(|| {
                    let mut output = vec![DspEdge::new(1, 1, 128, 44100);1];
                    conv.process(&input, &mut output)
            })},
            parameters
        )
    );
}



criterion_group!(benches, osc_bench, mod_bench, resampler_bench);
criterion_main!(benches);
