# Zed Unity Editor Extension

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Unity development extension for Zed, providing:

- **Roslyn-based C# Language Server** - Full C# IntelliSense powered by `csharp-language-server`
- **USS (Unity Style Sheets) Language Server** - Complete language support for UI Toolkit styling
- **Unity Package Integration** - Register Zed as Unity's external script editor with bidirectional file sync
- **Unity Debugging** - Debug adapter integration via `netcoredbg` (coming soon)
- **Syntax Highlighting** - Tree-sitter grammars for C# and USS

## Features

### C# Language Support

- **Auto-completion** for types, methods, properties, and Unity APIs
- **Go to Definition** for all symbols
- **Find References** across your codebase
- **Hover Documentation** with XML doc comments
- **Diagnostics** and error reporting
- **Code Actions** and refactoring
- **Unity-specific highlighting** for lifecycle methods and attributes

### USS Language Support

- **Property Completion** for all USS properties (including Unity-specific `-unity-*` properties)
- **Value Completion** with context-aware suggestions (colors, units, keywords)
- **Selector Completion** for Unity UXML elements (Button, Label, VisualElement, etc.)
- **Pseudo-class Support** (`:hover`, `:active`, `:focus`, `:disabled`, etc.)
- **Diagnostics** for syntax errors and unknown properties
- **Color Preview** and color picker integration
- **Variable Support** for USS custom properties (`--my-var`)
- **Document Formatting**

### Unity Debugging (Coming Soon)

- **Attach to Unity Editor** for debugging scripts
- **Breakpoints** and stepping through code
- **Variable inspection** and watch expressions
- **Call stack** navigation

### Unity Package Integration

The companion Unity package ([zed-unity-package](https://github.com/GameBayoumy/zed-unity-package)) provides:

- **External Script Editor** - Register Zed as Unity's external code editor
- **Bidirectional File Sync** - Changes in Zed automatically refresh in Unity
- **Project File Generation** - Automatic `.csproj` and `.sln` generation for Roslyn LSP
- **Cross-Platform Support** - Works on Linux, macOS, and Windows

## Installation

### From Zed Extensions (Coming Soon)

1. Open Zed
2. Go to Extensions (`Cmd/Ctrl + Shift + X`)
3. Search for "Unity"
4. Click Install

The extension will automatically download the required language servers on first use.

### Development Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/GameBayoumy/zed-unity
   ```

2. Create a symlink in Zed's extensions directory:
   ```bash
   ln -s zed-unity ~/.local/share/zed/extensions/installed/zed-unity
   ```

3. Build the extension WASM:
   ```bash
   cd zed-unity
   cargo build --target wasm32-wasip2
   ```

4. Restart Zed

The extension will automatically download the USS language server from [GitHub releases](https://github.com/GameBayoumy/uss-language-server/releases).

### Unity Package Installation

Install the companion Unity package to enable Zed as an external script editor:

#### Via Git URL (Recommended)

1. Open Unity Editor
2. Go to **Window > Package Manager**
3. Click the **+** button > **Add package from git URL**
4. Enter: `https://github.com/GameBayoumy/zed-unity-package.git`
5. Click **Install**

See the [zed-unity-package](https://github.com/GameBayoumy/zed-unity-package) repository for more details.

#### Configure Zed as External Editor

1. Go to **Edit > Preferences > External Tools**
2. Set **External Script Editor** to **Zed**
3. The package will auto-detect Zed's location, or you can set it manually

## Configuration

### Language Server Settings

Add to your Zed settings (`settings.json`):

```json
{
  "lsp": {
    "csharp-language-server": {
      "binary": {
        "path": "/path/to/custom/csharp-language-server",
        "arguments": []
      }
    }
  }
}
```

### Debug Configuration

Create a `.zed/debug.json` in your Unity project:

```json
{
  "configurations": [
    {
      "type": "unity",
      "request": "attach",
      "name": "Attach to Unity Editor",
      "address": "127.0.0.1",
      "port": 56000
    }
  ]
}
```

To find the Unity Editor's debug port:
1. Open Unity Editor
2. Go to Edit > Preferences > External Tools
3. Enable "Editor Attaching"
4. The port is typically `56000 + (process_id % 1000)`

## Unity Project Setup

For the best experience:

1. **Install the Unity Package**: Follow the Unity Package Installation instructions above

2. **Generate Project Files**: In Unity, go to **Tools > Zed > Regenerate Project Files** (or use Edit > Preferences > External Tools)

3. **Solution File**: The package automatically generates a `.sln` file at the project root

4. **Analyzers**: Unity's Roslyn analyzers will automatically be included in project files

### Unity Package Settings

Configure the Zed integration in **Edit > Preferences > External Tools** when Zed is selected:

| Setting | Description |
|---------|-------------|
| **Zed Path** | Path to Zed executable (auto-detected) |
| **Open in New Window** | Always open files in a new Zed window |
| **Enable File Sync** | Auto-refresh Unity when files change in Zed |
| **Sync Interval** | How often to check for file changes (0.5-10s) |
| **Generate .sln/.csproj** | Auto-generate project files for LSP |
| **Use Roslyn Analyzers** | Include Unity analyzers in project files |

### Menu Commands

Access via **Tools > Zed**:

- **Open Project in Zed** - Opens the entire Unity project in Zed
- **Regenerate Project Files** - Manually regenerate `.sln` and `.csproj` files
- **Force Sync** - Force a full file synchronization
- **Open Preferences** - Quick access to External Tools settings

## USS (Unity Style Sheets)

USS files (`.uss`, `.tss`) get full language support:

```css
/* Example USS file */
.my-button {
    background-color: #3498db;
    border-radius: 5px;
    padding: 10px 20px;
    -unity-font-style: bold;
    transition-duration: 0.2s;
}

.my-button:hover {
    background-color: #2980b9;
    scale: 1.05 1.05;
}

#title-label {
    font-size: 24px;
    -unity-text-align: middle-center;
    color: white;
}
```

## Supported File Types

| Extension | Language |
|-----------|----------|
| `.cs`     | C#       |
| `.uss`    | Unity Style Sheets |
| `.tss`    | Theme Style Sheets |

## Requirements

- **Zed** v0.140.0 or later
- **.NET SDK** 8.0 or later (for the C# language server)
- **Rust** toolchain with `wasm32-wasip2` target (for building the extension from source)
- **Unity** 2021.3 or later (for USS support)

### Installing the WASM target

```bash
rustup target add wasm32-wasip2
```

## Building from Source

### Extension WASM

```bash
cd zed-unity
cargo build --target wasm32-wasip2
```

### USS Language Server

The USS language server is maintained in the [uss-language-server](https://github.com/GameBayoumy/uss-language-server).

The extension automatically downloads pre-built binaries from GitHub releases. If you want to build from source:

```bash
git clone https://github.com/GameBayoumy/uss-language-server
cd uss-language-server
cargo build --release

# Install to PATH (Linux)
sudo cp target/release/uss-language-server /usr/local/bin/
```

## Troubleshooting

### C# Language Server Not Starting

1. Check that .NET SDK is installed: `dotnet --version`
2. Verify the solution file exists in your project root
3. Check Zed's log output for errors

### USS Completion Not Working

1. Ensure you're in a `.uss` or `.tss` file
2. The extension will auto-download the USS language server on first use
3. Check if uss-language-server downloaded: `ls ~/.local/share/zed/extensions/work/zed-unity/uss-language-server-*/`
4. Restart the language server: `Cmd/Ctrl + Shift + P` > "Restart Language Server"
5. Check Zed's log for errors: `~/.local/share/zed/logs/`

### USS Files Not Detected

1. Ensure the extension is installed and Zed has been restarted
2. Check the file extension is `.uss` or `.tss`
3. If still showing as CSS, manually select "USS" from the language picker

### Debugging Issues

1. Ensure Unity Editor is running with debugging enabled
2. Check the debug port in Unity's preferences
3. Verify no firewall is blocking the connection

### Unity Package Issues

1. **Zed not detected**: Use "Auto-Detect" in preferences or manually browse to Zed executable
2. **Project files not generating**: Check Unity console for errors, try "Regenerate Project Files"
3. **File sync not working**: Ensure "Enable File Sync" is checked in preferences
4. **LSP not working**: Make sure project files are generated and the Zed extension is installed

See the [zed-unity-package](https://github.com/GameBayoumy/zed-unity-package) repository for more troubleshooting.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE.md](LICENSE.md) for details.

## Acknowledgments

- [zed-unity-package](https://github.com/GameBayoumy/zed-unity-package) - Unity package for Zed integration
- [uss-language-server](https://github.com/GameBayoumy/uss-language-server) - USS Language Server
- [csharp-language-server](https://github.com/SofusA/csharp-language-server) by SofusA
- [netcoredbg](https://github.com/Samsung/netcoredbg) by Samsung
- [tree-sitter-c-sharp](https://github.com/tree-sitter/tree-sitter-c-sharp)
- [Unity Code](https://github.com/hackerzhuli/unity-code) for USS inspiration
- [nvim-unity](https://github.com/apyra/nvim-unity) for Unity integration patterns
