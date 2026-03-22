pub fn compute_spectrogram_data(samples: &[f32], sample_rate: u32, settings: &SpectrogramSettings) -> Option<SpectrogramData> {
    if samples.is_empty() { return None; }

    let fft_size = settings.fft_size;
    let hop_size = settings.hop_size;

    if samples.len() < fft_size { return None; }

    let window: Vec<f32> = (0..fft_size)
        .map(|n| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * n as f32 / fft_size as f32).cos()))
        .collect();

    let mut planner = realfft::RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);
    let mut input = fft.make_input_vec();
    let mut output = fft.make_output_vec();

    let num_frames = (samples.len() - fft_size) / hop_size + 1;
    let num_bins = fft_size / 2 + 1;

    let mut frames_mag = Vec::with_capacity(num_frames);
    let mut global_peak: f32 = 0.0;

    for i in 0..num_frames {
        let start = i * hop_size;
        for j in 0..fft_size {
            input[j] = samples[start + j] * window[j];
        }
        fft.process(&mut input, &mut output).ok()?;

        let mut frame = Vec::with_capacity(num_bins);
        for b in 0..num_bins {
            let mag = output[b].norm();
            if mag > global_peak { global_peak = mag; }
            frame.push(mag);
        }
        frames_mag.push(frame);
    }

    Some(SpectrogramData { frames_mag, global_peak, num_bins, hop_size, sample_rate, fft_size })
}
