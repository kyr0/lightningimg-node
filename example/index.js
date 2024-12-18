import { processDirectory } from "lightningimg-node"

console.time("Testing safe processing...")
// Process all images in the test_images directory and save the output in the test_output directory
processDirectory("./input", "./output")
console.timeEnd("Testing safe processing...")
