# David Mod for Super Smash Bros Ultimate

This project implements David from Cyberpunk Edgerunners as a playable character in Super Smash Bros Ultimate using Unreal Engine 4.

## Setup Instructions

[... existing content ...]

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Commit Guidelines

We follow these guidelines for commits:

1. Make small, focused commits that address a single concern.
2. Write clear, concise commit messages.
3. Use the present tense ("Add feature" not "Added feature").
4. Use the imperative mood ("Move cursor to..." not "Moves cursor to...").
5. Reference issues and pull requests in the commit message when applicable.

Example of a good commit message:
```
Add Sandevistan activation animation

- Create new animation blueprint for Sandevistan effect
- Implement timeline for slowing down time
- Add visual effects for Sandevistan activation
```

### Using the git_commit.py Script

To make regular commits and ensure consistency, use the `git_commit.py` script:

1. Make your changes to the project files.
2. Run the script with the following command:
   ```
   python git_commit.py <file1> [file2 ...] "Your commit message"
   ```
   Replace `<file1>`, `[file2 ...]` with the files you've changed, and provide a meaningful commit message.

3. The script will stage the specified files, commit the changes with the provided message, and push to the remote repository.

Example usage:
```
python git_commit.py src/character.rs src/moves.rs "Update David's move set and add Sandevistan effect"
```

By using this script, we ensure that all commits follow our guidelines and maintain a consistent commit history.

## Testing the Commit Process

This line is added to test our new git_commit.py script. It will be committed using the script to demonstrate its functionality.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
