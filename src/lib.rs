use wasm_bindgen::prelude::*;
use image::io::Reader as ImageReader;
use image::ImageFormat;
use rand::seq::SliceRandom;
use std::f32::consts::PI;
use std::io::Cursor;
use hound::{WavWriter, WavSpec, SampleFormat};

fn analyze_image(image_bytes: &[u8], format: ImageFormat, width: u32, height: u32) -> Vec<(u8, u8, u8)> {
    let img = ImageReader::with_format(Cursor::new(image_bytes), format)
        .decode()
        .expect("Failed to open image");
    let img = img.resize(width, height, image::imageops::FilterType::Nearest);
    img.to_rgba8().pixels().map(|p| (p[0], p[1], p[2])).collect()
}

fn color_to_frequency(r: u8, g: u8, b: u8, freq1: u32, freq2: u32) -> u32 {
    let total = r as u32 + g as u32 + b as u32;
    freq1 + (total % freq2)
}

fn generate_music(pixels: &[(u8, u8, u8)], sample_rate: u32, freq1: u32, freq2: u32, durations: &[u32], fade_in_duration: f32, fade_out_duration: f32,) -> Vec<f32> {
    let mut song = Vec::new();

    for pixel in pixels {
        let (r, g, b) = *pixel;
        let freq = color_to_frequency(r, g, b, freq1, freq2);
        let duration = *durations.choose(&mut rand::thread_rng()).unwrap();
        let tone = generate_sine_wave(freq, duration, sample_rate, fade_in_duration, fade_out_duration);
        song.extend(tone);
    }
    song
}

fn generate_sine_wave(freq: u32,duration: u32,sample_rate: u32,fade_in_duration: f32,fade_out_duration: f32,) -> Vec<f32> {
    let num_samples = (duration as f32 / 1000.0 * sample_rate as f32) as usize;
    let fade_in_samples = (num_samples as f32 * fade_in_duration).round() as usize;
    let fade_out_samples = (num_samples as f32 * fade_out_duration).round() as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for t in 0..num_samples {
        let amplitude = if t < fade_in_samples {
            t as f32 / fade_in_samples as f32
        } else if t >= num_samples - fade_out_samples {
            (num_samples - t) as f32 / fade_out_samples as f32
        } else {
            1.0
        };

        let sample = amplitude * (2.0 * PI * freq as f32 * t as f32 / sample_rate as f32).sin();
        samples.push(sample);
    }

    samples
}

fn save_wav(song: Vec<f32>) -> Vec<u8> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut buffer = Vec::new();
    let cursor = Cursor::new(&mut buffer);
    let mut writer = WavWriter::new(cursor, spec).expect("Failed to create WAV writer");

    for sample in song {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).expect("Failed to write sample");
    }

    writer.finalize().expect("Failed to finalize WAV file");

    buffer
}

fn process_image(image_bytes: &[u8],format: ImageFormat,sample_rate: u32,freq1: u32,freq2: u32,durations: &[u32],fade_in_duration: f32,fade_out_duration: f32, width: u32, height: u32) -> Vec<u8> {
    let pixels = analyze_image(image_bytes, format, width, height);
    let song = generate_music(&pixels, sample_rate, freq1, freq2, durations, fade_in_duration, fade_out_duration);
    save_wav(song)
}

#[wasm_bindgen]
pub fn generate_music_wasm(image_bytes: &[u8],format: &str,sample_rate: u32,freq1: u32,freq2: u32,durations: &[u32],fade_in_duration: f32,fade_out_duration: f32, width: u32, height: u32) -> Vec<u8> {
    let image_format = match format.to_lowercase().as_str() {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "tif" | "tiff" => ImageFormat::Tiff,
        _ => panic!("Formato de imagem não suportado: use png, jpg ou tif"),
    };

    process_image(image_bytes,image_format,sample_rate,freq1,freq2,durations,fade_in_duration,fade_out_duration, height, width)
}



/*
primeiro é preciso compilar o codigo para gerar o binario na pasta target para wasm com o seguinte comando:

comando: cargo build --target wasm32-unknown-unknown --release

apos compilar pode ser feito o uso do wasm-bindgen para facilitar o uso:

comando para instalar: cargo install wasm-bindgen-cli
comando para compilar: wasm-bindgen target/wasm32-unknown-unknown/release/seu_projeto.wasm --out-dir ./pkg --target web

apos compilar com wasm-bindgen os arquivos vao sair na pasta pkg

*/