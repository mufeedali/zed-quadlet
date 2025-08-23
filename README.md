# Quadlet support for Zed

A [Zed](https://zed.dev) language extension that provides support for [Podman Quadlet](https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html) files using the [quadlet-lsp](https://github.com/onlyati/quadlet-lsp) language server.

## Supported File Types

This extension recognizes and provides language support for the following Podman Quadlet file types:

- `.container` - Container unit files
- `.volume` - Volume unit files
- `.network` - Network unit files
- `.image` - Image unit files
- `.pod` - Pod unit files
- `.build` - Build unit files
- `.kube` - Kubernetes YAML unit files

### Install Extension

#### Option 1: Install as Dev Extension (for development/testing)

1. Open Zed
2. Open the command palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows)
3. Run `zed: install dev extension`
4. Select the directory containing this extension

#### Option 2: Install from Published Extension (when available)

1. Open Zed
2. Open the command palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows)
3. Run `zed: extensions`
4. Search for "Quadlet" and install

## Usage

Once installed, the extension will automatically:

- Recognize files with Quadlet extensions (`.container`, `.volume`, etc.)
- Provide syntax highlighting based on INI/systemd unit file syntax
- Offer hover documentation, completions, and error checking

## Development

Follow the dev extension installation instructions above and when you make changes, "Rebuild" the extension from Zed's extensions list.

### Contributing

Contributions are welcome! Please:

1. Fork the repository.
2. Create a feature branch.
3. Make your changes.
4. Update/add examples.
5. Submit a pull request.

### Issues

If you encounter any issues or have feature requests, please open an issue on the GitHub repository.

## License

This extension is released under the MIT License.
