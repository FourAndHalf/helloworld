# Contributing to Helloworld.

It is important to me that the code in main is written by own fair hands. After all, the point of the repository is so that I can demonstrate personal competance over a range of languages.

Where solutions are offered via pull requests they'll live in a branch called `contributions`. I warmly accept solutions from the general population but do understand that should they be merged, then they'll live in a contrib branch.

If you want to point out improvements to the code in `main`, then please feel free to do so.

Fun is as much a part of this project as the code. So be creative.

## How to Contribute

### Want to make a quick suggestion?

Open an issue and point to the line(s) of code in question with whatever feedback you deem fit.

### 1. Fork the Repository
Start by forking the repository and creating a new branch for your changes.

```bash
git checkout -b your-github-username/target-language
```

The target language bit is important to trigger the tests.  `elixir` in the pr will trigger the elixir tests, etc etc.

### 2. Make Your Changes

Make sure your code:
* Follows the project’s style and conventions.
* Includes tests where appropriate.
* Passes all existing tests (_note: I expect the vercel deployments to fail for a while until I get the TypeScript branch in_)

### 3. Commit and Push


Write clear, concise commit messages. Then push your branch.

```bash
git push origin your-github-username/target-language
```

### 4. Open a Pull Request

If your PR is an improvement to code already in main, submit a pull request (PR) to the main branch with a clear description of what your change does.

If your PR is a solution to a language I have not yet added, or have only partially implemented, please submit the PR to the contributions branch.

### 5. Code of Conduct

Be respectful. If I need to add more than that, I've failed in this world.

### 6. Need Help?

Open an issue or reach out on email to [me@treejamie.com](mailto:me@treejamie.com)
