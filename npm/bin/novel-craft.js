#!/usr/bin/env node
"use strict";

const { runLauncher } = require("../lib/launcher");

const status = runLauncher({
  binDir: __dirname,
  wrapperPath: __filename,
});

if (status !== null) {
  process.exit(status);
}
