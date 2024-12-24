# lightningimg-node

> Gone are the days when image conversion with Node.js was a hassle.

## Install

`npm install lightningimg-node`

## API

It's super simple! Take a look at the [`./example`](./example/index.js) folder.
Please keep in mind to install `lightningimg-node` as a dependency in your project (see [`./example/package.json`](./example/package.json)).

```js
import { processDirectoryDestructive, processDirectory, convertToWebp } from 'lightningimg-node'

// Process all images in the test_images directory and save the output in the test_output directory
processDirectory('./test_images', './test_output')

// Overwrite the original images with the processed images, keeping the original image's file extensions
// (this is only useful when working with bundlers for static site generators like Gatsby, Next.js, Astro, etc.)
processDirectoryDestructive('./test_images', /* keep original file names */ true)

// --- convert an image at runtime

// read the image file into a buffer
const imageBuffer = readFileSync('test_data/defuss_logo_jpg.JPEG');

// convert the image buffer to WebP format
const webpBuffer = convertToWebp(imageBuffer, 'jpeg', {
  quality: 80, // example quality
  dimensions: [400, 300], // resize to specific dimensions
  maintain_aspect_ratio: false // no aspect ratio maintenance, set it to true to zoom in and center
})

// write the webpBuffer to a file named 'liveconvert.webp' in the 'test_output' directory
writeFileSync('test_output/liveconvert.webp', webpBuffer);
```

## Support matrix

### Operating Systems

|                  | node14 | node16 | node18 | node20 |
| ---------------- | ------ | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      | ✓      |

## Contribute (for library developers)

### Install

- `yarn install`

### Building

After `yarn build/npm run build` command, you can see `lightningimg-node.[darwin|win32|linux].node` file in project root. This is the native addon built from [lib.rs](./src/lib.rs).

Debug builds:

- `yarn run build:debug`

Release builds:

- `yarn run build`

### Testing

With [ava](https://github.com/avajs/ava), run `yarn test/npm run test` to testing native addon. You can also switch to another testing framework if you want.

### Benchmarking

- `yarn run bench`

### Linting

- `yarn run lint`

### CI

With GitHub Actions, each commit and pull request will be built and tested automatically in [`node@14`, `node@16`, `@node18`] x [`macOS`, `Linux`, `Windows`] matrix. You will never be afraid of the native addon broken in these platforms.

### Release

Release native package is very difficult in old days. Native packages may ask developers who use it to install `build toolchain` like `gcc/llvm`, `node-gyp` or something more.

With `GitHub actions`, we can easily prebuild a `binary` for major platforms. And with `N-API`, we should never be afraid of **ABI Compatible**.

The other problem is how to deliver prebuild `binary` to users. Downloading it in `postinstall` script is a common way that most packages do it right now. The problem with this solution is it introduced many other packages to download binary that has not been used by `runtime codes`. The other problem is some users may not easily download the binary from `GitHub/CDN` if they are behind a private network (But in most cases, they have a private NPM mirror).

In this package, we choose a better way to solve this problem. We release different `npm packages` for different platforms. And add it to `optionalDependencies` before releasing the `Major` package to npm.

`NPM` will choose which native package should download from `registry` automatically. You can see [npm](./npm) dir for details.

### Development requirements

- Install the latest `Rust`
- Install `Node.js@10+` which fully supported `Node-API`
- Install `yarn@1.x`

### Test locally

- yarn
- yarn build
- yarn test

And you will see:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```

### Release the package

```
yarn version 0.3.3 # just an example, change version!
git add .
git commit -m "0.3.3"
git push
```

GitHub Actions will do the rest for us.
