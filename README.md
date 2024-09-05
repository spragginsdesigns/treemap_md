# TreeMapMD

TreeMapMD is a powerful Rust application that generates a markdown representation of your directory structure. It provides an intuitive graphical user interface for easy interaction and customization.

## Features

1. **Directory Selection**:

   - Choose any directory on your system to generate a markdown representation of its structure.
   - The selected directory path is displayed in the UI for confirmation.
2. **Markdown Preview**:

   - View a real-time preview of the generated markdown in the application window.
   - The preview updates automatically when changes are made.
3. **Exclude Directories**:

   - Add specific directories to an exclusion list.
   - Excluded directories and their contents will not appear in the generated markdown.
   - Easily remove directories from the exclusion list with a single click.
4. **Asynchronous Processing**:

   - Directory traversal and markdown generation occur asynchronously.
   - The UI remains responsive during processing of large directory structures.
5. **Export Functionality**:

   - Export the generated markdown to a file named "sourceTree.md" in the selected directory.
   - Receive immediate feedback on the success or failure of the export operation.
6. **Error Handling**:

   - Comprehensive error handling with user-friendly error messages displayed in the UI.
   - Timeout mechanism for long-running operations to prevent the application from hanging.
7. **Dark Mode UI**:

   - A sleek, dark-themed user interface for comfortable viewing in low-light environments.

## How to Use

1. **Select Directory**:
   Click the "Select Directory" button to choose the root directory for markdown generation.
2. **Exclude Directories (Optional)**:
   Click "Add Excluded Directory" to select directories you wish to exclude from the markdown representation.
3. **Generate Preview**:
   Click "Generate Preview" to create and display the markdown representation of your directory structure.
4. **Export**:
   After generating a preview, click "Export" to save the markdown to a file named "sourceTree.md" in the selected directory.

## Technical Details

- Built with Rust for high performance and memory safety.
- Uses `eframe` for the graphical user interface.
- Implements multi-threading for responsive UI during intensive operations.
- Utilizes `tokio` for asynchronous operations.
- File system operations are handled safely with comprehensive error checking.

## Installation

TODO

## Contributing

[If you're open to contributions, provide guidelines on how others can contribute to your project]

## License

MIT
