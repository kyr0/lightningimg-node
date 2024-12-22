import { processDirectoryDestructive, processDirectory, convertToWebp } from './index.js'
import { rmSync } from 'node:fs'
import { copyFileSync, mkdirSync, readdirSync, statSync, existsSync } from 'node:fs'
import { join } from 'node:path'
import { readFileSync, writeFileSync } from 'node:fs';

function copyFiles(src, dest) {
  mkdirSync(dest, { recursive: true })
  const files = readdirSync(src)
  for (const file of files) {
    const srcFile = join(src, file)
    const destFile = join(dest, file)
    if (statSync(srcFile).isFile()) {
      copyFileSync(srcFile, destFile)
    }
  }
}

// remove the test_output directory before processing
rmSync('./test_output', { recursive: true, force: true })
rmSync('./test_images', { recursive: true, force: true })

// create the test_images directory only if it does not exist
if (!existsSync('test_images')) {
  mkdirSync('test_images', { recursive: true })
}

// create the test_output directory only if it does not exist
if (!existsSync('test_output')) {
  mkdirSync('test_output', { recursive: true })
}

// Copy all files from test_data to test_images
copyFiles('./test_data', './test_images')

console.time('Testing safe processing...')
// Process all images in the test_images directory and save the output in the test_output directory
processDirectory('./test_images', './test_output')
console.timeEnd('Testing safe processing...')

console.time('Testing destructive processing...')
// Overwrite the original images with the processed images, keeping the original image's file extensions
// (this is only useful when working with bundlers for static site generators like Gatsby, Next.js, Astro, etc.)
processDirectoryDestructive('./test_images', /* keep original file names */ true)
console.timeEnd('Testing destructive processing...')

// Copy all files from test_data to test_images
copyFiles('./test_data', './test_images')

// test that it works when the directory does not exist
console.time('Testing destructive processing with non-existing folder...')
// Overwrite the original images with the processed images, keeping the original image's file extensions
// (this is only useful when working with bundlers for static site generators like Gatsby, Next.js, Astro, etc.)
processDirectoryDestructive('./test_images_non_existing', /* keep original file names */ true)
console.timeEnd('Testing destructive processing with non-existing folder...')

// read the image file into a buffer
const imageBuffer = readFileSync('test_data/defuss_logo_jpg.JPEG');

// convert the image buffer to WebP format
const webpBuffer = convertToWebp(imageBuffer, 'jpeg', {
  quality: 80, // example quality
  dimensions: [400, 300], // no resizing
  maintain_aspect_ratio: false // no aspect ratio maintenance
})

console.log('Conversion successful, WebP buffer size:', webpBuffer.length);

// write the webpBuffer to a file named 'liveconvert.webp' in the 'test_output' directory
writeFileSync('test_output/liveconvert.webp', webpBuffer);
console.log('WebP file written successfully to test_output/liveconvert.webp');


// convert the image buffer to WebP format
const webpBufferAR = convertToWebp(imageBuffer, 'jpeg', {
  quality: 100, // example quality
  dimensions: [400, 300], // no resizing
  maintain_aspect_ratio: true // no aspect ratio maintenance
})

console.log('Conversion successful with aspect ratio, 100%, WebP buffer size:', webpBufferAR.length);

// write the webpBuffer to a file named 'liveconvert.webp' in the 'test_output' directory
writeFileSync('test_output/liveconvert_100_ar.webp', webpBufferAR);
console.log('WebP file written successfully to test_output/liveconvert_100_ar.webp');
