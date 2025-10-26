use anyhow::Context as _;
use clap::Parser as _;
use palette::{Clamp as _, IntoColor as _, IsWithinBounds as _, Okhsl, OklabHue, Srgb};
use std::io;

// TODO: Output in color if `stdout.is_terminal()`

/// Adjust sRGB colors via Okhsl
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
    /// Hue (0.0 to 360.0)
    #[value(name = "h")]
    Hue,

    /// Saturation (0.0 to 0.4)
    #[value(name = "s")]
    Saturation,

    /// Lightness (0.0 to 1.0)
    #[value(name = "l")]
    Lightness,
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
        let output = hsl(&args, &input)?;
        println!("{output}");
    }
    Ok(())
}

fn hsl(args: &Args, input: &str) -> anyhow::Result<String> {
    let input_rgb_u8: Srgb<u8> = input
        .parse()
        .context("Failed to parse input into sRGB color")?;

    let input_rgb_f32: Srgb<f32> = input_rgb_u8.into_format();

    let mut okhsl: Okhsl = input_rgb_f32.into_color();

    match (&args.component, &args.adjustment) {
        (Component::Hue, Adjustment::Set) => okhsl.hue = OklabHue::new(args.value),
        (Component::Hue, Adjustment::Increase) => okhsl.hue += OklabHue::new(args.value),
        (Component::Hue, Adjustment::Decrease) => okhsl.hue -= OklabHue::new(args.value),

        (Component::Saturation, Adjustment::Set) => okhsl.saturation = args.value,
        (Component::Saturation, Adjustment::Increase) => okhsl.saturation += args.value,
        (Component::Saturation, Adjustment::Decrease) => okhsl.saturation -= args.value,

        (Component::Lightness, Adjustment::Set) => okhsl.lightness = args.value,
        (Component::Lightness, Adjustment::Increase) => okhsl.lightness += args.value,
        (Component::Lightness, Adjustment::Decrease) => okhsl.lightness -= args.value,
    }

    if args.no_clamp {
        if !okhsl.is_within_bounds() {
            anyhow::bail!("Value out of bounds");
        }
    } else {
        okhsl = okhsl.clamp();
    }

    let output_rgb_f32: Srgb<f32> = okhsl.into_color();
    let output_rgb_u8: Srgb<u8> = output_rgb_f32.into_format();

    let hash = if input.starts_with('#') { "#" } else { "" };

    let output = format!("{hash}{output_rgb_u8:x}");

    Ok(output)
}
