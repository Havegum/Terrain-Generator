import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import { string } from 'rollup-plugin-string';
import rust from '@wasm-tool/rollup-plugin-rust';
import replace from '@rollup/plugin-replace';
import alias from '@rollup/plugin-alias';

import dotenv from 'dotenv';
dotenv.config();
const production = !process.env.ROLLUP_WATCH;


function serve() {
	let server;

	function toExit() {
		if (server) server.kill(0);
	}

	return {
		writeBundle() {
			if (server) return;
			server = require('child_process').spawn('npm', ['run', 'start', '--', '--dev'], {
				stdio: ['ignore', 'inherit', 'inherit'],
				shell: true
			});

			process.on('SIGTERM', toExit);
			process.on('exit', toExit);
		}
	};
}

const plugins = [
	svelte({
		dev: !production,			         // Enable run-time checks when not in production
		css: css => css.write('bundle.css'), // Extract CSS to separate file for performance
	}),

	replace({
		process: JSON.stringify({
			env : {
				WORLD_POINTS: process.env.WORLD_POINTS || null,
			}
		})
	}),
	alias({
		resolve: ['.js', '.svelte'],
		entries: { '@': __dirname + '/src' }
	}),

	resolve({
		browser: true,
		dedupe: importee => importee === 'svelte' || importee.startsWith('svelte/'),
	}),

	commonjs(),
	rust({ verbose: !production }),
	string({ include: ['**/*.glsl'] })
];

export default [
	{
		input: 'src/utils/terrain-worker.js',
		output: {
			format: 'cjs',
			name: 'worker',
			file: 'public/terrain-worker.js',
		},
		plugins: plugins,
	},
	{
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
			...plugins,
			!production && serve(),
			!production && livereload('public'), // Watch and autoreload if in dev
			production && terser() 				 // Minify if in production
		],
		watch: {
			clearScreen: false
		}
	}
];
