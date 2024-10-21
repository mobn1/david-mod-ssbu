# David Mod for Super Smash Bros Ultimate

This project implements David from Cyberpunk Edgerunners as a playable character in Super Smash Bros Ultimate using Unreal Engine 4.

## Setup Instructions

### Prerequisites

1. Visual Studio Code
2. Unreal Engine 4.27 or later
3. Git

### Visual Studio Code Setup

1. Install Visual Studio Code from [https://code.visualstudio.com/](https://code.visualstudio.com/)

2. Install the following extensions for Visual Studio Code:
   - C/C++
   - C# for Visual Studio Code
   - Unreal Engine 4 Snippets
   - Unreal Intellisense

3. Clone the repository:
   ```
   git clone https://github.com/your-repo/david-mod-ssbu.git
   cd david-mod-ssbu
   ```

4. Open the project in Visual Studio Code:
   ```
   code .
   ```

### Unreal Engine 4 Setup

1. Open the Unreal Project:
   - Navigate to the `UnrealProject` folder
   - Double-click on `DavidMod.uproject` to open the project in Unreal Engine 4

2. Generate Visual Studio project files:
   - Right-click on `DavidMod.uproject`
   - Select "Generate Visual Studio project files"

3. Build the project:
   - In Unreal Engine 4, go to File > Build Project

### Using Visual Studio Code with the Project

1. The project comes with pre-configured settings for Visual Studio Code. These settings are located in the `.vscode` folder:
   - `settings.json`: Configures the C/C++ extension and sets up include paths
   - `tasks.json`: Defines build tasks for the project

2. To build the project using Visual Studio Code:
   - Press `Ctrl+Shift+B` or go to Terminal > Run Build Task
   - Select "Build DavidMod" from the task list

3. To navigate the codebase:
   - Use `Ctrl+P` to quickly open files
   - Use `F12` to go to definitions
   - Use `Shift+F12` to find all references

4. For debugging:
   - Set breakpoints in your code by clicking on the left margin of the editor
   - Use the Debug view (`Ctrl+Shift+D`) to start debugging sessions

### Building and Running the Project

1. To build the project:
   - Use the "Build DavidMod" task in Visual Studio Code
   - Or use Unreal Engine's built-in build system

2. To run the project:
   - Open the project in Unreal Engine 4
   - Click the "Play" button in the editor

## Project Structure

- `UnrealProject/`: Contains the Unreal Engine 4 project files
  - `Source/`: C++ source code for the project
  - `Content/`: Blueprints, assets, and other content
- `Plugins/DavidModPlugin/`: Custom plugin for David's abilities
- `docs/`: Project documentation
- `.vscode/`: Visual Studio Code configuration files

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
