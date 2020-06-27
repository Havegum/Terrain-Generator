import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import rust from '@wasm-tool/rollup-plugin-rust';

import autoPreprocess from 'svelte-preprocess';

const production = !process.env.ROLLUP_WATCH;

export default {
	input: 'src/main.js',
	output: {
		sourcemap: true,
		format: 'iife',
		name: 'app',
		file: 'public/bundle.js',
	},
	onwarn: function (warning, warn) {
  		if (warning.code === 'CIRCULAR_DEPENDENCY') return;
  		warn(warning);
	},
	plugins: [
		svelte({
			dev: !production,			                      // Enable run-time checks when not in production
			css: css => css.write('public/bundle.css'), // Extract CSS to separate file for performance
			preprocess: autoPreprocess()								// SCSS support
		}),

		resolve({
			browser: true,
			dedupe: importee => importee === 'svelte' || importee.startsWith('svelte/')
		}),

		commonjs(),

		rust({
	    debug: !production,
			verbose: !production
		}),

		!production && livereload('public'), // Watch and autoreload if in dev
		production && terser() 							 // Minify if in production
	],
	watch: {
		clearScreen: false
	}
};
