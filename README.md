# hsl

Adjust hue, saturation, and lightness of sRGB colors via the perceptually
uniform Okhsl color space.

```
$ echo '#ff0000' | hsl h + 45
#0087fd
```

```
$ hsl -h
Adjust HSL of sRGB via Okhsl

Usage: hsl [OPTIONS] <PARAMETER> <ADJUSTMENT> <VALUE>

Arguments:
  <PARAMETER>   [possible values: h, s, l]
  <ADJUSTMENT>  [possible values: =, +, -, %]
  <VALUE>

Options:
      --no-clamp  Don't clamp values
  -h, --help      Print help (see more with '--help')
```

## Resources

- ["Okhsv and Okhsl: Two new color spaces for color picking" by Björn Ottosson](https://bottosson.github.io/posts/colorpicker/)
- ["sRGB" on Wikipedia](https://en.wikipedia.org/wiki/SRGB)
- ["Oklab color space" on Wikipedia](https://en.wikipedia.org/wiki/Oklab_color_space)
