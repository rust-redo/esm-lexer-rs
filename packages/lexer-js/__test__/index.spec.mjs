import test from 'ava'

import { parse  } from '../index.js'

test('parse', (t) => {
  t.deepEqual(parse(''), {facade: false, hasModuleSyntax: false})
})
