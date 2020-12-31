const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    entry: { bundle: './src/entrypoint.js' },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].[contenthash].js',
        publicPath: ''
    },
    target: 'web',
    plugins: [
        new MiniCssExtractPlugin({
            filename: "style.[contenthash].css",
        }),
        new HtmlWebpackPlugin({
            inject: false,
            hash: true,
            favicon: "assets/favicon.ico",
            template: "src/index.html",
            filename: "index.html"
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
                loader: 'file-loader'
            }
        ]
    },
    experiments: {
        syncWebAssembly: true
    },
    resolve: {
        fallback: {
            util: require.resolve("util/")
        }
    }
};
