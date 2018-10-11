const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
    entry: './entrypoint.js',
    plugins: [
        new HtmlWebpackPlugin({
            inject: "body",
            template: "index.html"
        }),
        new webpack.optimize.LimitChunkCountPlugin({
            maxChunks: 2,
        }),
    ],
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'entrypoint.js',
    }
};
