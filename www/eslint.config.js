const neostandard = require('neostandard');

module.exports = [
    {
        ignores: [
            'src/wasm/**',
            'src/third_party/**',
            'src/bootstrap-3.4.1-dist/**',
            'dist/**'
        ]
    },
    ...neostandard({
        semi: true,
        env: ['browser', 'mocha'],
        ts: false
    }),
    {
        rules: {
            '@stylistic/indent': ['error', 4],
            '@stylistic/eol-last': 'off',
            '@stylistic/no-trailing-spaces': 'off',
            '@stylistic/spaced-comment': 'off'
        }
    }
];
