# png_diff

A simple rust binary to generate the difference between two pngs of the same size as a heatmap

![heatmap example](./assets/example_heatmap.png)

## Usage

```bash
png_diff <path_to_original> <path_to_new> <path_to_output_heatmap>
```

## Dev Notes
- The current implementation of the program calculates the heatmap using a single thread. My reasoning is that you would ideally spawn a new process whenever required