const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
    entry: './bootstrap.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bootstrap.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            inject: "head",
            template: "index.html"
        })
    ],
    mode: 'development'
};
