#!/usr/bin/env node

import { keyboard } from '@nut-tree/nut-js';

keyboard.config.autoDelayMs = 0;

(async () => {
  const text = process.argv[2];
  await keyboard.type(text);
})();
