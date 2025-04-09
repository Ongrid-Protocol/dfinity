# Bootstrap Node

A decentralized bootstrap node implementation for the P2P network.

## Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/) (version 0.25.0 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Node.js](https://nodejs.org/) (LTS version)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)

## Local Deployment

### 1. Start Local Internet Computer Network

```bash
dfx start --clean --background
```

### 2. Deploy Canisters Locally

```bash
# Build and deploy all canisters
dfx deploy

# Or deploy specific canisters
dfx deploy bootstrap_node_backend
dfx deploy bootstrap_node_frontend
```

### 3. Access the Application

- Frontend: http://localhost:4943
- Backend Candid UI: http://localhost:4943/?canisterId=<BACKEND_CANISTER_ID>

### 4. Environment Configuration

The local deployment uses the following environment variables (automatically set in `.env`):
- `DFX_NETWORK=local`
- `CANISTER_ID_BOOTSTRAP_NODE_BACKEND=bkyz2-fmaaa-aaaaa-qaaaq-cai`

## Mainnet Deployment

### 1. Prerequisites for Mainnet

- ICP tokens for cycles
- Identity with sufficient cycles
- Mainnet identity configured in DFX

### 2. Configure Mainnet Deployment

1. Update `dfx.json`:
```json
{
  "networks": {
    "ic": {
      "providers": ["https://ic0.app"]
    }
  }
}
```

2. Set the network to mainnet:
```bash
dfx identity use default
dfx identity get-principal
```

### 3. Deploy to Mainnet

```bash
# Deploy to mainnet
dfx deploy --network ic

# Deploy specific canisters
dfx deploy --network ic bootstrap_node_backend
dfx deploy --network ic bootstrap_node_frontend
```

### 4. Post-Deployment

1. Note the deployed canister IDs from the deployment output
2. Update your frontend configuration with the new canister IDs
3. Update any environment variables or configuration files with the new canister IDs

## Development

### Building the Project

```bash
# Build Rust backend
cargo build

# Build frontend
npm install
npm run build
```

### Testing

```bash
# Run backend tests
cargo test

# Run frontend tests
npm test
```

## Configuration Modifications

### Backend Configuration

1. Update `src/bootstrap_node_backend/Cargo.toml` for dependencies
2. Modify `src/bootstrap_node_backend/bootstrap_node_backend.did` for interface changes
3. Update environment variables in `.env` file

### Frontend Configuration

1. Update `package.json` for dependencies
2. Modify frontend source in `src/bootstrap_node_frontend`
3. Update build configuration in `tsconfig.json`

## Troubleshooting

### Common Issues

1. **Canister Deployment Fails**
   - Check cycle balance
   - Verify network connectivity
   - Ensure correct identity is selected

2. **Local Network Issues**
   - Stop and restart DFX: `dfx stop && dfx start --clean --background`
   - Clear cache: `dfx cache clean`

3. **Build Failures**
   - Update dependencies: `cargo update` or `npm update`
   - Check for version conflicts
   - Verify Rust and Node.js versions

## Security Considerations

1. Keep private keys secure
2. Regularly update dependencies
3. Monitor canister cycles
4. Implement proper access control
5. Use secure communication channels

## Maintenance

1. Regular cycle top-ups
2. Dependency updates
3. Security patches
4. Performance monitoring
5. Log analysis

## Support

For issues and support, please open an issue in the repository or contact the development team.

Welcome to your new `bootstrap_node` project and to the Internet Computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with `bootstrap_node`, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd bootstrap_node/
dfx help
dfx canister --help
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
