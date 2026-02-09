use anyhow::Context as _;
use colored::Colorize as _;
use palette::{Clamp as _, IntoColor as _, IsWithinBounds as _, Okhsl, OklabHue, Srgb};
use std::{env, io, process};

struct Args {
    adjustments: Vec<(Parameter, Adjustment, f32)>,
    no_clamp: bool,
}

enum Parameter {
    Hue,
    Saturation,
    Lightness,
}

enum Adjustment {
    Set,
    Increase,
    Decrease,
    Percentage,
}

const HELP: &str = "\
Adjust HSL of sRGB via Okhsl

Usage: hsl [OPTIONS] <ADJUSTMENT>...

Arguments:
  <ADJUSTMENT>  Parameter (h/s/l), operator (=/+/-/%), and value

Options:
      --no-clamp  Don't clamp values
  -h, --help      Print help

Examples:
  echo '#c0ffee' | hsl h+30 s+0.1
  echo '#bada55' | hsl s+0.2
  echo '#facade' | hsl h-60 l+0.1\
";

fn parse_adjustment(s: &str) -> anyhow::Result<(Parameter, Adjustment, f32)> {
    if s.len() < 3 {
        anyhow::bail!(
            "Invalid adjustment '{s}'; expected a parameter (h/s/l), operator (=/+/-/%), and value"
        );
    }

    let parameter = match &s[0..1] {
        "h" => Parameter::Hue,
        "s" => Parameter::Saturation,
        "l" => Parameter::Lightness,
        other => anyhow::bail!("Invalid parameter '{other}' in '{s}'; expected h/s/l"),
    };

    let adjustment = match &s[1..2] {
        "=" => Adjustment::Set,
        "+" => Adjustment::Increase,
        "-" => Adjustment::Decrease,
        "%" => Adjustment::Percentage,
        other => {
            anyhow::bail!("Invalid operator '{other}' in '{s}'; expected =/+/-/%")
        }
    };

    let value: f32 = s[2..]
        .parse()
        .with_context(|| format!("Invalid value '{}' in '{s}'; expected a number", &s[2..]))?;

    Ok((parameter, adjustment, value))
}

fn parse_args() -> anyhow::Result<Args> {
    use lexopt::{Parser, prelude::*};

    let mut parser = Parser::from_env();
    let mut no_clamp = false;
    let mut adjustments = Vec::new();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                println!("{HELP}");
                process::exit(0);
            }
            Long("no-clamp") => {
                no_clamp = true;
            }
            Value(value) => {
                let adjustment = parse_adjustment(&value.string()?)?;
                adjustments.push(adjustment);
            }
            _ => return Err(arg.unexpected().into()),
        }
    }

    if adjustments.is_empty() {
        anyhow::bail!("Expected at least one adjustment");
    }

    Ok(Args {
        adjustments,
        no_clamp,
    })
}

fn main() -> anyhow::Result<()> {
    let args = parse_args()?;

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

    let input_okhsl: Okhsl = input_rgb_f32.into_color();
    let output_okhsl: Okhsl = hsl_okhsl(args, input_okhsl)?;

    let output_rgb_f32: Srgb<f32> = output_okhsl.into_color();
    let output_rgb_u8: Srgb<u8> = output_rgb_f32.into_format();

    Ok(output_rgb_u8)
}

fn hsl_okhsl(args: &Args, mut okhsl: Okhsl) -> anyhow::Result<Okhsl> {
    for (parameter, adjustment, value) in &args.adjustments {
        match (parameter, adjustment) {
            (Parameter::Hue, _) => match adjustment {
                Adjustment::Set => okhsl.hue = OklabHue::from_degrees(value - 180.0),
                Adjustment::Increase => okhsl.hue += OklabHue::from_degrees(value - 180.0),
                Adjustment::Decrease => okhsl.hue -= OklabHue::from_degrees(value - 180.0),
                Adjustment::Percentage => {
                    okhsl.hue = OklabHue::from_degrees(
                        ((okhsl.hue.into_degrees() + 180.0) * (value / 100.0)) - 180.0,
                    );
                }
            },

            (Parameter::Saturation, Adjustment::Set) => okhsl.saturation = *value,
            (Parameter::Saturation, Adjustment::Increase) => okhsl.saturation += value,
            (Parameter::Saturation, Adjustment::Decrease) => okhsl.saturation -= value,
            (Parameter::Saturation, Adjustment::Percentage) => okhsl.saturation *= value / 100.0,

            (Parameter::Lightness, Adjustment::Set) => okhsl.lightness = *value,
            (Parameter::Lightness, Adjustment::Increase) => okhsl.lightness += value,
            (Parameter::Lightness, Adjustment::Decrease) => okhsl.lightness -= value,
            (Parameter::Lightness, Adjustment::Percentage) => okhsl.lightness *= value / 100.0,
        }
    }

    if args.no_clamp {
        if !okhsl.is_within_bounds() {
            anyhow::bail!("Value out of bounds");
        }
    } else {
        okhsl = okhsl.clamp();
    }

    Ok(okhsl)
}
