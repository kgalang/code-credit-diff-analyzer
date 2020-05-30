# Code Credit: Diff Analyzer

---

<!-- TABLE OF CONTENTS -->
## Table of Contents

- [Code Credit: Diff Analyzer](#code-credit-diff-analyzer)
  - [Table of Contents](#table-of-contents)
  - [About The Project](#about-the-project)
    - [Built With](#built-with)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [License](#license)
  - [Contact](#contact)
  - [Notes](#notes)
  - [TODO](#todo)



<!-- ABOUT THE PROJECT -->
## About The Project

The goal of this project is to be able to extract data from a git diff. With output such as added/removed **significant** lines of code and programming language, I'm hoping to be able to eventually use the derived metrics to analyze the complexity of the git diff or commit.


### Built With

* [Rust](https://www.rust-lang.org/)
* [unidiff-rs](https://github.com/messense/unidiff-rs)



<!-- GETTING STARTED
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* npm
```sh
npm install npm@latest -g
```

### Installation
 
1. Clone the code-credit-diff-analyzer
```sh
git clone https://github.com/kgalang/code-credit-diff-analyzer.git
```
2. Install NPM packages
```sh
npm install
``` -->



<!-- USAGE EXAMPLES -->
<!-- ## Usage -->


<!-- Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_ -->


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Kevin Galang - kevingalang@protonmail.com

Project Link: [https://github.com/kgalang/code-credit-diff-analyzer](https://github.com/kgalang/code-credit-diff-analyzer)


## Notes

- This project helped me gain a deeper understanding of ownership and borrowing in Rust. 
  - Felt like it clicked after needing to work through the model hierarchy of files, hunks, and lines.
    - mainly when I needed to add logic at different steps and wanted to abstract out of one large function.

## TODO
- [ ] Account for block comments
- [ ] Account for whitespace lines
- [ ] Expose as a python module
- [ ] Expose as CLI
