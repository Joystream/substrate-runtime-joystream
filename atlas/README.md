# atlas

## About

The components package holds the React components used in Atlas and their stories, while the app package contains the Atlas App itself.
Given how the code is organized, the first time you clone the repo, you need to build the components package.

## Getting Started

After cloning the repo run:

```bash
$ cd atlas
$ yarn install
```

### Components Package

The components package is located under `./packages/components`, so every command that follows should be prefixed by

```bash
$ cd packages/components
```

To start storybook run

```bash
$ yarn storybook
```

To build the components package and use it elsewhere, for example inside the app package, you can run:

```bash
$ yarn build
```

When developing, you can run the bundler in watch mode with

```bash
$ yarn start
```

#### Populate `index.ts`

Running

```bash
$ yarn index
```

will write import and exports from every file inside `src/components` to `src/index.ts`

For the script to run properly make sure to follow the convention of naming a Component the same as the file is exported from.

For example, exporting `Button` from `Button.tsx` will work while exporting `Cactus` from `Plant.tsx` will not.

If you do not wish to follow this convention, you can just ignore the index script and run

```bash
$ yarn build
```

#### App package

The components package is located under `./packages/app`, so every command that follows should be prefixed by

```bash
$ cd packages/app
```

Then run

```bash
$ yarn dev
```

To start the app on `localhost:1234`

## Deploy Storybook as a static site

To deploy storybook as a static site, a `now.json` file has been setup for deployment with [Zeit Now](https://now.sh).
