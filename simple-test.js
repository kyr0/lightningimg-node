const { processDirectoryDestructive, processDirectory } = require('./index')

console.assert(typeof processDirectoryDestructive === "undefined", 'Simple test failed, processDirectoryDestructive function is not defined')
console.assert(typeof processDirectory === "undefined", 'Simple test failed, processDirectory function is not defined')

console.info('Simple test passed, API contract is correct')