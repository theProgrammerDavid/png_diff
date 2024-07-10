# png_diff [![Release](https://github.com/theProgrammerDavid/png_diff/actions/workflows/release.yml/badge.svg)](https://github.com/theProgrammerDavid/png_diff/actions/workflows/release.yml)

A simple rust binary to generate the difference between two pngs of the same size as a heatmap

![heatmap example](./assets/example_heatmap.png)

## Usage
![Alt text](./assets/usage.png)

## Dev Notes
- The current implementation of the program calculates the heatmap using a single thread. My reasoning is that you would ideally spawn a new process whenever required