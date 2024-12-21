import { processDirectoryDestructive, processDirectory } from './index.js'
import { rmSync } from 'node:fs'
import { copyFileSync, mkdirSync, readdirSync, statSync, existsSync } from 'node:fs'
import { join } from 'node:path'

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
