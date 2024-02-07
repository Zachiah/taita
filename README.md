# Taita

This is a project manager I am building for myself. I would like to make it useful for others as well in the long term, but for now the MVP is going to be hyperspecific to my usecase. [My vision for the project](./VISION.md)

## Usage (pre-alpha but still notes)

### With wofi
```
taita ls -p | wofi --show dmenu | xargs -I "{}" taita open -p "{}"
```
