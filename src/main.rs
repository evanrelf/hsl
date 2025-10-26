use anyhow::Context as _;
use clap::Parser as _;
use palette::{Clamp as _, IntoColor as _, IsWithinBounds as _, OklabHue, Oklch, Srgb};
use std::io;

/// Adjust sRGB colors via Oklch
#[derive(clap::Parser)]
#[command(disable_help_subcommand = true)]
struct Args {
    component: Component,

    adjustment: Adjustment,

    value: f32,

    /// Don't clamp values
    #[arg(long)]
    no_clamp: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum Component {
    /// Perceptual lightness (0.0 to 1.0)
    #[value(name = "l")]
    Lightness,

    /// Chromatic intensity (0.0 to 0.4)
    #[value(name = "c")]
    Chroma,

    /// Hue angle (0.0 to 360.0)
    #[value(name = "h")]
    Hue,
}

#[derive(clap::ValueEnum, Clone)]
enum Adjustment {
    /// Set
    #[value(name = "=")]
    Set,

    /// Increase
    #[value(name = "+")]
    Increase,

    /// Decrease
    #[value(name = "-")]
    Decrease,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    for line in io::stdin().lines() {
        let input = line.context("Failed to parse input into UTF-8 string")?;
        let output = lch(&args, &input)?;
        println!("{output}");
    }
    Ok(())
}

fn lch(args: &Args, input: &str) -> anyhow::Result<String> {
    let input_rgb_u8: Srgb<u8> = input
        .parse()
        .context("Failed to parse input into sRGB color")?;

    let input_rgb_f32: Srgb<f32> = input_rgb_u8.into_format();

    let mut oklch: Oklch = input_rgb_f32.into_color();

    match (&args.component, &args.adjustment) {
        (Component::Lightness, Adjustment::Set) => oklch.l = args.value,
        (Component::Lightness, Adjustment::Increase) => oklch.l += args.value,
        (Component::Lightness, Adjustment::Decrease) => oklch.l -= args.value,

        (Component::Chroma, Adjustment::Set) => oklch.chroma = args.value,
        (Component::Chroma, Adjustment::Increase) => oklch.chroma += args.value,
        (Component::Chroma, Adjustment::Decrease) => oklch.chroma -= args.value,

        (Component::Hue, Adjustment::Set) => oklch.hue = OklabHue::new(args.value),
        (Component::Hue, Adjustment::Increase) => oklch.hue += OklabHue::new(args.value),
        (Component::Hue, Adjustment::Decrease) => oklch.hue -= OklabHue::new(args.value),
    }

    if args.no_clamp {
        if !oklch.is_within_bounds() {
            anyhow::bail!("Value out of bounds");
        }
    } else {
        oklch = oklch.clamp();
    }

    let output_rgb_f32: Srgb<f32> = oklch.into_color();
    let output_rgb_u8: Srgb<u8> = output_rgb_f32.into_format();

    let hash = if input.starts_with('#') { "#" } else { "" };

    let output = format!("{hash}{output_rgb_u8:x}");

    Ok(output)
}
