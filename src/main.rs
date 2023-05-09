
/*

Chaos distortion effect (for guitar) in which a sample's absolute value determines a width for taking random samples to its left or to its right.
We have a limit for noise cutoff and a width paramter.
*/

use hound;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;

fn mapchaos(generator: &mut StdRng, data: &Vec<i32>, idx: usize, limit: i32, maxwidth: usize) -> i32{

    if data[idx] == 0 {

        return 0;

    }

    if i32::abs(data[idx]) < limit {

        return data[idx];

    }

    let mut idx = idx;

    let isnegative = if data[idx] < 0 { true } else { false };

    let spread: usize = maxwidth / i32::abs(data[idx]) as usize;
    
    if spread <= 1 {

        return data[idx];

    }

    if maxwidth > 1 {

        let sign = generator.gen_range(0..2);
        
        let idxdelta: usize = generator.gen_range(1..spread) as usize;

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

    if args.len() <= 2 {

        println!("chaostortion wavefile.wav limit width");
        return;

    }

    let limit: i32 = args[2].trim().parse().expect("Failed to read limit");
    let abswidth: usize = args[3].trim().parse().expect("Failed to read width");

    let mut data: Vec<i32> = Vec::new();

    let mut generator = StdRng::seed_from_u64(0);

    // Read the wave
    let mut wavreader = hound::WavReader::open(args[1].trim()).expect("Failed to open file");

    // wave file spec
    let spec = wavreader.spec();

    let mut wavwriter = hound::WavWriter::create("out.wav", spec).expect("Failed to create output wave");

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

        let mut mutant: i32 = mapchaos(&mut generator, &data, index, limit, abswidth);
        
        if mutant > bitfeildrange {

            println!("{mutant} truncated down");
            let sign: i32  = if mutant < 0 { - 1 } else { 1 }; 
            mutant = (i32::abs(bitfeildrange) - 1) * sign;
        }

        wavwriter.write_sample(mutant).expect("Failed to write wave sample");
    }


    wavwriter.finalize().unwrap();

}



