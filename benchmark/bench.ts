import { Bench } from 'tinybench'
import { processDirectoryDestructive, processDirectory, convertToWebp } from '../index.js'
import { copyFile, mkdir, readdir } from 'node:fs/promises'
import { join } from 'node:path'
import { existsSync, mkdirSync, readFileSync } from 'node:fs'
import { rmSync } from 'node:fs'

async function copyFiles(src: string, dest: string): Promise<void> {
  await mkdir(dest, { recursive: true })
  const files = await readdir(src, { withFileTypes: true })
  const copyPromises = files
    .filter((file) => file.isFile())
    .map((file) => copyFile(join(src, file.name), join(dest, file.name)))
  await Promise.all(copyPromises)
}

// remove the test_output directory before processing
rmSync('./test_output', { recursive: true, force: true })

// create the test_images directory only if it does not exist
if (!existsSync('test_images')) {
  mkdirSync('test_images', { recursive: true })
}

// create the test_output directory only if it does not exist
if (!existsSync('test_output')) {
  mkdirSync('test_output', { recursive: true })
}

const b = new Bench()

b.add('Convert files from directory A to directory B', async () => {
  // Copy all files from test_data to test_images
  await copyFiles('./test_data', './test_images')

  // Process all images in the test_images directory and save the output in the test_output directory
  processDirectory('./test_images', './test_output')
})

b.add('Transparently convert files in directory A (destructive)', async () => {
  // Copy all files from test_data to test_image
  await copyFiles('./test_data', './test_images')

  // Overwrite the original images with the processed images, keeping the original image's file extensions
  // (this is only useful when working with bundlers for static site generators like Gatsby, Next.js, Astro, etc.)
  processDirectoryDestructive('./test_images', /* keep original file names */ true)
})

// read the image file into a buffer
const imageBuffer = readFileSync('test_data/defuss_logo_jpg.JPEG');

b.add('Runtime convert single image as Buffer, with aspect-ratio, with scaling', async () => {
  // convert the image buffer to WebP format
  convertToWebp(imageBuffer, 'jpeg', {
    quality: 100, // example quality
    dimensions: [400, 300], // no resizing
    maintain_aspect_ratio: false // no aspect ratio maintenance
  })
});

b.add('Runtime convert single image as Buffer, no aspect-ratio, with scaling', async () => {
  // convert the image buffer to WebP format
  convertToWebp(imageBuffer, 'jpeg', {
    quality: 100, // example quality
    dimensions: [400, 300], // no resizing
    maintain_aspect_ratio: false // no aspect ratio maintenance
  })
});


b.add('Runtime convert single image as Buffer, no aspect-ratio, no scaling', async () => {
  // convert the image buffer to WebP format
  convertToWebp(imageBuffer, 'jpeg', {
    quality: 100, // example quality
  })
});

console.info('Running benchmarks...')

await b.run()

// remove the test_output directory before processing
rmSync('./test_output', { recursive: true, force: true })

console.table(b.table())
