const path = require('path');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: './bootstrap.js',
    plugins: [
//        new CleanWebpackPlugin(['dist']),
        new HtmlWebpackPlugin({
            inject: "body",
            template: "index.html"
        })
    ],
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bootstrap.js',
    }
};
