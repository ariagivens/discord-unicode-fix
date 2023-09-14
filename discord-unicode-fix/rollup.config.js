import { base64 } from 'rollup-plugin-base64'

export default {
  input: 'src/index.js',
  output: {
    file: 'dist/discord_unicode_fix.js',
    format: 'es',
  },
  plugins: [
    base64({ include: "**/*.wasm" })
  ],
};
