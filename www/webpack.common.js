const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    entry: { bundle: './src/entrypoint.js' },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].[contenthash].js',
    },
    target: 'web',
    plugins: [
        new HtmlWebpackPlugin({
            inject: false,
            hash: true,
            favicon: "assets/favicon.ico",
            template: "src/index.html",
            filename: "index.html"
        }),
        new MiniCssExtractPlugin({
            filename: "style.[contenthash].css",
        })
    ],
    module: {
        rules: [
            {
                test: /\.css$/,
                use: [
                    {
                        loader: MiniCssExtractPlugin.loader,
                    },
                    'css-loader'
                ]
            },
            {
                test: /\.(jpg|jpeg|gif|png|woff|woff2|eot|ttf|svg|ico)$/,
                loader: 'url-loader',
                options: {
                    limit: 1024
                }
            }
        ]
    }
};
