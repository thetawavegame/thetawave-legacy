# Contributing

If you haven't already, please take some time to read the community [Code of Conduct](https://github.com/amethyst/theta-wave/blob/master/CODE_OF_CONDUCT.md).

## The Basics

If you're familiar with the basics of the [Amethyst](https://amethyst.rs/) engine
or programming with Rust, then you can contribute! And even if you aren't, these
are some of the things you can learn to get up to speed:

### ECS and Amethyst concepts

- [Entity and Component](https://book.amethyst.rs/stable/concepts/entity_and_component.html)
- [System](https://book.amethyst.rs/stable/concepts/system.html)
- [Event Channels](https://book.amethyst.rs/stable/concepts/event-channel.html)

### Rust concepts

- [Patterns and Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Option](https://doc.rust-lang.org/stable/rust-by-example/std/option.html)
- [Traits](https://doc.rust-lang.org/stable/rust-by-example/trait.html)

This isn't an exhaustive list of everything there is to know. But it should be
enough to start contributing.

## Contributing code

In general, the process for contributing goes like
this:

- Pick an issue to work on
- Setup your development environment
- Write the patch, create a PR, and request a review

### Pick an issue to work on

Take a look at the [issues list](https://github.com/amethyst/theta-wave/issues)
and choose an issue you'd like to work on. Issues labeled as [good first issue](https://github.com/amethyst/theta-wave/issues?q=is%3Aissue+label%3A%22good+first+issue%22+is%3Aopen)
are a great way to get started with the codebase.

Make sure to claim the issue by
commenting you'd like to work on it and mentioning either **@cdsupina** or
**@tigleym**. Ask questions if something is unclear about the issue you are
working on.

#### Creating your own issue

If the existing issues aren't what you're looking for then feel free to create
a new issue! This can be fixing a bug, improving documentation, or a
proposal to implement a new feature. Whatever the purpose is, please make sure
to create an issue for it describing what you intend to do and why.

If you need help identifying what the focus of your issue should be,
feel free to mention either **@cdsupina** and **@tigleym**. We can help identify
the sort of work that might be involved for your issue.

This step is important because we want to make sure the work behind each pull
request has context and intent.

### Setting up your dev environment

This part of the process involves getting a local copy of the project and setting
up your development machine to be able to build and run it.

1. Since this is a game built with the Amethyst game engine, it's important that
[Rust is setup](https://book.amethyst.rs/stable/getting-started.html) and have
the proper [dependencies installed](https://github.com/amethyst/amethyst/blob/master/README.md#dependencies)
 on your dev machine.

2. [Fork][fork-link] the Theta Wave project.

3. [Create a local clone of the fork](https://docs.github.com/en/github/getting-started-with-github/fork-a-repo#step-2-create-a-local-clone-of-your-fork)

4. [Run the game](https://github.com/amethyst/theta-wave#to-run)

Once you're able to successfully build the game, you can start writing some code!

### Write the patch, PR, and request a review

Before writing your patch, branch off `develop`:

```none
git checkout -b <branch-name> develop
```

When finished writing your patch, [save][save-changes-link] and
[commit][git-commit-link] your work:

```none
git add
git commit -m "meaningful commit message"
```

Make sure to run `cargo fmt` to ensure your work is formatted correctly and that
no warnings are present when you run `cargo build`. The CI checks will fail if
build warnings are present in your PR.

Now that your patch is ready for review, you can [push][git-push-link] your changes:

```none
git push
```

 and [create that PR][create-pr-link] for review!

**_Please make sure to link the issue you selected earlier in the PR's description._**

[create-pr-link]: https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/creating-an-issue-or-pull-request#creating-a-pull-request
[git-push-link]: https://www.atlassian.com/git/tutorials/syncing/git-push
[save-changes-link]: https://www.atlassian.com/git/tutorials/saving-changes
[git-commit-link]: https://www.atlassian.com/git/tutorials/saving-changes/git-commit
[fork-link]: https://docs.github.com/en/github/getting-started-with-github/fork-a-repo#fork-an-example-repository
