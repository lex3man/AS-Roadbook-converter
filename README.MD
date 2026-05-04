# RoadBook converter
Util for converting RoadBook PDFs to PNG slices

## Dependences
RoadBook converter requires `poppler` to be installed.

### Windows
Windows users will have to build or download `poppler` for Windows.

### macOS
using [homebrew](https://brew.sh):
`brew install poppler`

### Linux
Most distros ship with `pdftoppm` and `pdftocairo`. If they are not installed, refer to your package manager to install `poppler-utils`

## Usage
```
cargo run -- -s <input_file> -o <output_dir> -d
```

### Args
- `-s` or `--source`: Input file
- `-o` or `--output`: Output directory
- `-d` or `--debug`: Activate debug mode
- `-p` or `--padding`: Top padding for page offset
- `--height`: Slice height
- `--width`: Slice width

Only `--source` arg required.