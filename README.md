# B2D

A 2D shape editor that runs in your browser. Vision: like Blender3D for 2D

This project is mainly a personal learning project - not something intended for real use cases, but who knows where it'll end up

Target features:
- Core logic running in Rust through WASM
- Serverless collaborative editing using WebRTC
- UI built with ReactJS in TypeScript, connected to the application state in WASM memory

Inspiration
- Blender architecture

DevLog
- Adding support for WASM in CRA using CRACO: https://dev.to/nicolasrannou/wasm-in-create-react-app-4-in-5mn-without-ejecting-cf6
- Hot-reload after re-compiling WASM using `yarn build:wasm`: Use yarn link: `cd b2d-core/pkg && yarn link && cd ../.. && yarn link b2d-core`
  Edit: Doesn't seem to work with webpack (doesn't put the correct headers in there). So, just yarn install each time (no linking)

## Default CRA readme:

This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app).

### Available Scripts

In the project directory, you can run:

#### `yarn start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.\
You will also see any lint errors in the console.

#### `yarn test`

Launches the test runner in the interactive watch mode.\
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

#### `yarn build`

Builds the app for production to the `build` folder.\
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.\
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

#### `yarn eject`

**Note: this is a one-way operation. Once you `eject`, you can’t go back!**

If you aren’t satisfied with the build tool and configuration choices, you can `eject` at any time. This command will remove the single build dependency from your project.

Instead, it will copy all the configuration files and the transitive dependencies (webpack, Babel, ESLint, etc) right into your project so you have full control over them. All of the commands except `eject` will still work, but they will point to the copied scripts so you can tweak them. At this point you’re on your own.

You don’t have to ever use `eject`. The curated feature set is suitable for small and middle deployments, and you shouldn’t feel obligated to use this feature. However we understand that this tool wouldn’t be useful if you couldn’t customize it when you are ready for it.

### Learn More

You can learn more in the [Create React App documentation](https://facebook.github.io/create-react-app/docs/getting-started).

To learn React, check out the [React documentation](https://reactjs.org/).
