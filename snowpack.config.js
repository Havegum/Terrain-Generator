
/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
    mount: {
        src: '/dist',
        public: { url: '/', static: true },
    },
    plugins: [
        ['@snowpack/plugin-dotenv'],
        ['@snowpack/plugin-svelte'],
        [
            'snowpack-plugin-wasm-pack',
            {
                projectPath: './src/terrain_generator',
                outDir: './public/assets',
            },
        ],
    ],

    devOptions: {
        port: 5000,
    },
  };