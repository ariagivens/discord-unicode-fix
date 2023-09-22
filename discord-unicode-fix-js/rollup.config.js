import nodeResolve from '@rollup/plugin-node-resolve';
import dsv from '@rollup/plugin-dsv';
import commonjs from '@rollup/plugin-commonjs';

export default {
    input: 'src/discord_unicode_fix.js',
    output: {
      file: 'dist/discord_unicode_fix.js',
      format: 'es',
    },
    external: [ 'grapheme-splitter' ],
    plugins: [
      nodeResolve(),
      commonjs(),
      dsv(),
    ],
  };
  