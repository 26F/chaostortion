
/*
"Chaos Distortion" is temporal distortion of sample data with probability coupled with amplitude.
chaostortion input.wav 4.5 800 512
*/

use hound;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::env;

fn mapchaos(generator: &mut ThreadRng, data: &Vec<i32>, chaos_multiplier: f64, sample_max: f64, idx: usize, limit: i32, maxwidth: usize) -> i32{

    if data[idx] == 0 {

        return 0;

    }

    if i32::abs(data[idx]) < limit {

        return data[idx];

    }

    let mut idx = idx;

    let isnegative = data[idx] < 0;
    
    if maxwidth <= 1 {

        return data[idx];

    }

    let current_sample_fl: f64 = i32::abs(data[idx]) as f64;
    let mut probability_of_distort_sample = (sample_max * chaos_multiplier) / current_sample_fl;

    if probability_of_distort_sample > 1.0 {

        probability_of_distort_sample = 1.0;

    }

    let mutate_sample: bool = generator.gen::<f64>() % 1.0 < probability_of_distort_sample;

    if maxwidth > 1 && mutate_sample {

        let sign = generator.gen_range(0..2);
        
        let idxdelta: usize = generator.gen_range(1..maxwidth) as usize;

        // negative
        if sign == 0 {

            if idx >= idxdelta {

                idx -= idxdelta

            } else {

                idx = data.len() -1 as usize - idxdelta + idx;

            }

        } else {

            if idx + idxdelta < data.len() as usize {

                idx += idxdelta;

            } else {

                idx = (idx + idxdelta) % data.len() as usize;

            }

        }

    }

    let resultsign = if data[idx] < 0 { true } else { false };

    if resultsign != isnegative {

        if !isnegative {

            return (data[idx] + 1) * - 1;

        } else {

            return data[idx] * -1;

        }

    }

    return data[idx];

}


fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() <= 3 {

        println!("chaostortion wavefile.wav chaos_multiplier limit width");
        return;

    }

    let fname: &str = args[1].trim();
    let chaos_multiple: f64 = args[2].trim().parse().expect("Failed to read chaos chaos_multiplier");
    let limit: i32 = args[3].trim().parse().expect("Failed to read limit");
    let abswidth: usize = args[4].trim().parse().expect("Failed to read width");

    let mut data: Vec<i32> = Vec::new();

    let mut generator = rand::thread_rng();

    // Read the wave
    let mut wavreader = hound::WavReader::open(fname).expect("Failed to open file");

    // wave file spec
    let spec = wavreader.spec();

    let mut wavwriter = hound::WavWriter::create("out_f.wav", spec).expect("Failed to create output wave");

    let samplebitdepth = spec.bits_per_sample - 1;
    let bitfeildrange = i32::pow(2, samplebitdepth as u32) - 1;

    let samples = wavreader.samples::<i32>();

    // get the samples
    for sample in samples {

        let smpl = sample.unwrap();
        data.push(smpl);

    }

    // mutate 
    for index in 0..data.len() {

        let mut mutant: i32 = mapchaos(&mut generator, &data, chaos_multiple, bitfeildrange as f64, index, limit, abswidth);
        
        if mutant > bitfeildrange {

            println!("{mutant} truncated down");
            let sign: i32  = if mutant < 0 { - 1 } else { 1 }; 
            mutant = (i32::abs(bitfeildrange) - 1) * sign;
        }

        wavwriter.write_sample(mutant).expect("Failed to write wave sample");
    }


    wavwriter.finalize().unwrap();

}



