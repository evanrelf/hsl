# hsl

Adjust hue, saturation, and lightness of sRGB colors via the perceptually
uniform Okhsl color space.

```
$ echo '#ff0000' | hsl h+45
#0087fd
```

```
$ hsl -h
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
```

## Resources

- ["Okhsv and Okhsl: Two new color spaces for color picking" by Björn Ottosson](https://bottosson.github.io/posts/colorpicker/)
- ["sRGB" on Wikipedia](https://en.wikipedia.org/wiki/SRGB)
- ["Oklab color space" on Wikipedia](https://en.wikipedia.org/wiki/Oklab_color_space)
