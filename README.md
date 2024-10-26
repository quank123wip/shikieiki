# Project Shikieiki
![Rust](https://img.shields.io/badge/rust-orange?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-GPL--3.0-blue)
> The great judge in the fantasy world.

## Brief Introduction

Project Shikieiki (name taken from the Touhou franchise) is the new **progressive** framework for consuming competitive programming judge tasks. Aimed to become a new infrastructure of the next-gen online judge system and other systems related to competitive programming, Project Shikieiki is composed by multiple reuseable units and new standards for judge tasks. The basic concept of the project is inspired by the idea of modern CI/CD pipelines, and it allows users to define their own judge pipeline to custom the whole procedure the judge system working and give out the desireable result in fair time. Given out the fact that competitive programmers aren't all familiar with those concepts and techniques, we are trying hard to simplify the user experience and make things like providing out-of-box judge task templates and easy tutorials that doesn't require in-depth knowledges about the standard.

## Modules

### Shikieiki

The great manager of local judge tasks.

### Aya

The judge task runner. Consume a standard task flow configuration file and other required files like user inputs and give the defined outputs after execute the whole judge task. Can be used as a CLI tool to run checkers on the local environment. (Doesn't guarantee the safety of the task)

### Sakuya

The manager of container services.

### Rin

The library for running containers. Based on libcontainer, using cgroups and seccomp to ensure the safety.

### Patchouli

The library for managing images, rootfs(s) and judge files. Support pulling required files from a new standard judge queue dispatcher endpoint and its related file storage services.

## Contributing

...