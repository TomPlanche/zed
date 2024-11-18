# My custom version of [Zed](https://github.com/zed-industries/zed).

As a beginner in Rust, this is my playground to learn more about the language and the ecosystem.


## Changes

- Better sorting:

  I was tired of lexicographical sorting in the project panel, so I made the [better-project-panel](https://github.com/TomPlanche/zed/tree/better-project-panel) branch.
  It contains new sorting settings.

  - Alphabetical/Lexicographical sorting (reversed or not).
  - Uppercase before lowercase.
  - File type sorting.

  ```json
  "project_panel": {
      "dock": "left",
      "indent_size": 15,
      "sort": {
          "strategy": "alphabetical",
          "uppercase_first": false,
          "file_type": false
      }
  },
  ```
