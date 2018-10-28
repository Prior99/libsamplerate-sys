#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn simple_sample_rate_conversion() {
        unsafe {
            // Setup sample data and storage.
            let freq = ::std::f32::consts::PI * 880f32 / 44100f32;
            let mut input: Vec<f32> = (0..44100).map(|i| (freq * i as f32).sin()).collect();
            let mut resampled = vec![0f32;48000];
            let mut output = vec![0f32;44100];
            // Convert from 44100Hz to 48000Hz.
            let mut src_pass_1 = SRC_DATA {
                data_in: input.as_mut_ptr(),
                data_out: resampled.as_mut_ptr(),
                input_frames: 44100,
                output_frames: 48000,
                src_ratio: 48000f64 / 44100f64,
                end_of_input: 0,
                input_frames_used: 0,
                output_frames_gen: 0,
            };
            src_simple(&mut src_pass_1 as *mut SRC_DATA, SRC_SINC_BEST_QUALITY as i32, 1);
            // Convert from 48000Hz to 44100Hz.
            let mut src_pass_2 = SRC_DATA {
                data_in: resampled.as_mut_ptr(),
                data_out: output.as_mut_ptr(),
                input_frames: 48000,
                output_frames: 44100,
                src_ratio: 44100f64 / 48000f64,
                end_of_input: 0,
                input_frames_used: 0,
                output_frames_gen: 0,
            };
            src_simple(&mut src_pass_2 as *mut SRC_DATA, SRC_SINC_BEST_QUALITY as i32, 1);
            // Expect the difference between all input frames and all output frames to be less than
            // an epsilon.
            let error = input.iter().zip(output).fold(0f32, |max, (input, output)| max.max((input - output).abs()));
            assert!(error < 2f32);
        }
    }

    #[test]
    fn complex_sample_rate_conversion() {
        unsafe {
            // Setup sample data and storage.
            let freq = ::std::f32::consts::PI * 880f32 / 44100f32;
            let mut input: Vec<f32> = (0..44100).map(|i| (freq * i as f32).sin()).collect();
            let mut resampled = vec![0f32;48000];
            let mut output = vec![0f32;44100];
            // Create the samplerate converter.
            let mut error = 0i32;
            let converter: *mut SRC_STATE = src_new(SRC_SINC_BEST_QUALITY as i32, 1, &mut error as *mut i32);
            assert!(error == 0);
            // Initial input configuration.
            let mut src = SRC_DATA {
                data_in: input.as_mut_ptr(),
                data_out: resampled.as_mut_ptr(),
                input_frames: 44100,
                output_frames: 48000,
                src_ratio: 48000f64 / 44100f64,
                end_of_input: 0,
                input_frames_used: 0,
                output_frames_gen: 0,
            };
            // Convert the input data in slices.
            let slices = 10;
            for i in 0..slices {
                src.data_in = input[i * 44100 / slices..(i + 1) * 44100 / slices].as_mut_ptr();
                src.data_out = resampled[i * 48000 / slices..(i + 1) * 48000 / slices].as_mut_ptr();
                src_process(converter, &mut src as *mut SRC_DATA);
            }
            // Delete the converter.
            src_delete(converter);
            // Convert back from 48000Hz to 44100Hz.
            let mut src_reverse = SRC_DATA {
                data_in: resampled.as_mut_ptr(),
                data_out: output.as_mut_ptr(),
                input_frames: 48000,
                output_frames: 44100,
                src_ratio: 44100f64 / 48000f64,
                end_of_input: 0,
                input_frames_used: 0,
                output_frames_gen: 0,
            };
            src_simple(&mut src_reverse as *mut SRC_DATA, SRC_SINC_BEST_QUALITY as i32, 1);
            // Expect the difference between all input frames and all output frames to be less than
            // an epsilon.
            let error = input.iter().zip(output).fold(0f32, |max, (input, output)| max.max((input - output).abs()));
            assert!(error < 2f32);
        }
    }
}
