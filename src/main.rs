use anyhow::Context as _;
use clap::Parser as _;
use colored::Colorize as _;
use palette::{Clamp as _, IntoColor as _, IsWithinBounds as _, Okhsl, OklabHue, Srgb};
use std::{env, io};

/// Adjust sRGB colors via Okhsl
#[derive(clap::Parser)]
#[command(disable_help_subcommand = true)]
struct Args {
    parameter: Parameter,

    adjustment: Adjustment,

    value: f32,

    /// Don't clamp values
    #[arg(long)]
    no_clamp: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum Parameter {
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

    /// Percentage
    #[value(name = "%")]
    Percentage,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // SAFETY: This program is single-threaded.
    unsafe {
        // `colored` does a bad job of detecting whether a terminal (e.g. Ghostty) has truecolor
        // support. I don't care about supporting legacy terminals, so I'm forcing it on. You can
        // turn colors off with `NO_COLOR`.
        env::set_var("COLORTERM", "truecolor");
    }

    for line in io::stdin().lines() {
        let input_string = line.context("Failed to read input bytes as UTF-8 string")?;

        let input_rgb: Srgb<u8> = input_string
            .parse()
            .context("Failed to parse input string as sRGB color")?;

        let output_rgb = hsl(&args, input_rgb)?;

        let hash = if input_string.starts_with('#') {
            "#"
        } else {
            ""
        };

        let output_string = format!("{hash}{output_rgb:x}")
            .truecolor(output_rgb.red, output_rgb.green, output_rgb.blue)
            .reversed();

        println!("{output_string}");
    }

    Ok(())
}

fn hsl(args: &Args, input_rgb_u8: Srgb<u8>) -> anyhow::Result<Srgb<u8>> {
    let input_rgb_f32: Srgb<f32> = input_rgb_u8.into_format();

    let mut okhsl: Okhsl = input_rgb_f32.into_color();

    match (&args.parameter, &args.adjustment) {
        (Parameter::Hue, _) => match &args.adjustment {
            Adjustment::Set => okhsl.hue = OklabHue::from_degrees(args.value - 180.0),
            Adjustment::Increase => okhsl.hue += OklabHue::from_degrees(args.value - 180.0),
            Adjustment::Decrease => okhsl.hue -= OklabHue::from_degrees(args.value - 180.0),
            Adjustment::Percentage => {
                okhsl.hue = OklabHue::from_degrees(
                    ((okhsl.hue.into_degrees() + 180.0) * (args.value / 100.0)) - 180.0,
                );
            }
        },

        (Parameter::Saturation, Adjustment::Set) => okhsl.saturation = args.value,
        (Parameter::Saturation, Adjustment::Increase) => okhsl.saturation += args.value,
        (Parameter::Saturation, Adjustment::Decrease) => okhsl.saturation -= args.value,
        (Parameter::Saturation, Adjustment::Percentage) => okhsl.saturation *= args.value / 100.0,

        (Parameter::Lightness, Adjustment::Set) => okhsl.lightness = args.value,
        (Parameter::Lightness, Adjustment::Increase) => okhsl.lightness += args.value,
        (Parameter::Lightness, Adjustment::Decrease) => okhsl.lightness -= args.value,
        (Parameter::Lightness, Adjustment::Percentage) => okhsl.lightness *= args.value / 100.0,
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

    Ok(output_rgb_u8)
}
