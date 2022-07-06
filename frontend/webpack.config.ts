import path from 'path'
import { Configuration, ProgressPlugin, DefinePlugin } from 'webpack'
import type { Configuration as DevServerConfiguration } from 'webpack-dev-server'
import ESLintWebpackPlugin from 'eslint-webpack-plugin'
import HtmlWebPackPlugin from 'html-webpack-plugin'
import HtmlWebpackHarddiskPlugin from 'html-webpack-harddisk-plugin'
import ReactRefreshWebpackPlugin from '@pmmmwh/react-refresh-webpack-plugin'
import ReactRefreshTypeScript from 'react-refresh-typescript'
import MiniCssExtractPlugin from 'mini-css-extract-plugin'

const IS_DEV = process.env.NODE_ENV != 'production'
const IS_DEV_SERVER = process.env.WEBPACK_SERVE

const resolve = (...paths: string[]): string => path.resolve(__dirname, ...paths)

export function baseURL(path: string, isDev: boolean = IS_DEV): string {
  const p = path.replace(/^\//, '')
  if (isDev && IS_DEV_SERVER) {
    return `http://localhost:3000/${p}`
  } else {
    return p
  }
}

type WebpackConfig = Configuration & { devServer?: DevServerConfiguration }

const webpack_config: WebpackConfig = {
  devtool: 'source-map',
  entry: [
    'webpack-dev-server/client?http://localhost:3000',
    'webpack/hot/only-dev-server',
    resolve('./src/app/index.tsx')
  ],
  output: {
    path: resolve('../public'),
    filename: 'js/[name].js',
    publicPath: baseURL('/')
  },
  devServer: {
    liveReload: true,
    host: '0.0.0.0',
    port: 3000,
    compress: true,
    hot: true,
    historyApiFallback: true,
    client: {
      logging: 'verbose',
      overlay: true,
      progress: true,
      webSocketURL: {
        hostname: '0.0.0.0',
        pathname: '/ws',
        port: 3000,
        protocol: 'ws'
      }
    },
    allowedHosts: '*',
    static: {
      publicPath: baseURL('/')
    },
    headers: {
      'Access-Control-Allow-Origin': '*'
    }
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/i,
        use: [
          {
            loader: require.resolve('ts-loader'),
            options: {
              getCustomTransformers: () => ({
                before: [IS_DEV && ReactRefreshTypeScript()].filter(Boolean)
              }),
              transpileOnly: IS_DEV
            }
          }
        ],
        include: resolve('./src'),
        exclude: /node_modules/
      },
      {
        test: /\.css$/i,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              importLoaders: 1,
              sourceMap: true,
              url: false
            }
          },
          'postcss-loader'
        ],
        exclude: [/node_modules/, /\.module\.css$/i],
        include: resolve('./src')
      },
      {
        test: /\.module\.css$/i,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              importLoaders: 1,
              sourceMap: true,
              url: false,
              modules: {
                mode: 'local',
                auto: true,
                localIdentName: '[name]__[local]--[hash:base64:5]',
                localIdentContext: path.resolve(__dirname, 'src')
              }
            }
          },
          'postcss-loader'
        ],
        exclude: /node_modules/,
        include: resolve('./src')
      },

      {
        test: /\.html$/i,
        exclude: /node_modules/,
        include: resolve('./src'),
        use: {
          loader: 'html-loader',
          options: { minimize: false }
        }
      }
    ]
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js'],
    alias: {
      '@': resolve('./src/app')
    }
  },
  plugins: [
    new ESLintWebpackPlugin({
      context: resolve('./src'),
      extensions: ['ts', 'tsx'],
      fix: true,
      threads: true
    }),
    new ProgressPlugin(),
    new HtmlWebPackPlugin({
      template: resolve('./src/html/index.html'),
      alwaysWriteToDisk: true
    }),
    new HtmlWebpackHarddiskPlugin({
      outputPath: resolve('../views')
    }),
    new MiniCssExtractPlugin({
      // Options similar to the same options in webpackOptions.output
      // both options are optional
      filename: IS_DEV ? '[name].css' : '[name].[contenthash].css',
      chunkFilename: IS_DEV ? '[id].css' : '[id].[contenthash].css'
    })
  ]
}

module.exports = () => {
  if (!IS_DEV) {
    webpack_config.mode = 'production'
  } else {
    webpack_config.mode = 'development'
    webpack_config.plugins?.push(new ReactRefreshWebpackPlugin())
  }
  return webpack_config
}
