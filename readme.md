# What is this
Syncro is an CLI tool to that helps you to syncronize files between multiple folders.

Why? The reason that I'm creating this is because I want help to mantain my dotfiles repository. 
My dotfiles must live in their respective place (which is usually .config) and the repository
folder. This program helps me to do so.

Maybe it will be also useful if you want to automatically create security copies of files between
diferent disks.

# Is it ready to be used?
YES!! Syncro can be used from now on!

# How to build
Since is a rust application you will just need to have rust and cargo installed and run the
following command.

```
cargo build --release
```
or for installing it
```
cargo install --path .
```

# How it works
First you need a directory where store the copies, one you have it run
```
syncro init
```
Then start tracking files with
```
syncro add <files>
```
Files can either be files or directories.
Finally, syncronize with
```
syncro update
```
