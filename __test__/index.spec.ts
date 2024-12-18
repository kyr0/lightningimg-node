import test from 'ava'

import { processDirectoryDestructive, processDirectory } from '../index.js'

test('API contrat filfilled', (t) => {
  t.is(typeof processDirectoryDestructive, "function")
  t.is(typeof processDirectory, "function")
})