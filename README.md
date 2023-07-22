<img src="https://raw.githubusercontent.com/NAlexPear/pg_branch/main/assets/logo.svg" width="50%" height="50%" alt="AI-generated logo that confers more legitimacy than this project deserves">

# `pg_branch`

![Pre-release Checks](https://github.com/NAlexPear/pg_branch/actions/workflows/check.yml/badge.svg?branch=main)

A Postgres extension for quickly creating "branches" of individual databases within a Postgres cluster using copy-on-write file systems like [`BTRFS`](https://wiki.archlinux.org/title/btrfs).

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
2. [Future Work](#future-work)
2. [Similar Projects](#similar-projects)

## Introduction

> **Warning**
> This is **pre-alpha software**, meant as an experimental proof-of-concept. Use at your own risk!

Postgres makes it easy to create new, empty databases with the `CREATE DATABASE` command. It's so easy, in fact, that one would think that creating new databases from existing databases would be easy, too. [But it's not](https://stackoverflow.com/questions/876522/creating-a-copy-of-a-database-in-postgresql).

Postgres provides the option to create one database from another using [`CREATE DATABASE name [WITH] [TEMPLATE template]`](https://www.postgresql.org/docs/current/sql-createdatabase.html), but doing so has two major restrictions:

1. there can be _no active connections to the `template` database_, and...
2. performance degrades rapidly as the size of the database increases

`pg_branch` is a Postgres extension that solves those problems by giving `CREATE DATABASE` the power of snapshots. If your `PGDATA` directory is on a copy-on-write file system like [`BTRFS`](https://wiki.archlinux.org/title/btrfs), the `pg_branch` extension turns every `CREATE DATABASE` into an atomic file system snapshot that takes seconds instead of minutes (or hours). In addition, the copy-on-write strategy keeps disk usage low by only writing new segment data files to disk when they're modified (rather than read).

TL;DR: `CREATE EXTENSION pg_branch` makes `CREATE DATABASE <dbname> WITH TEMPLATE <bigdbname>` super fast

## Getting Started

Before installing `pg_branch`, it's important to configure the file system that the database cluster will use. The following steps will get you started:

> **Disclaimer**: these steps are written with Linux in mind, and have only been testing on Linux.

0. **install prerequisites**

    You'll need an installation of `btrfs` (usually packaged as `btrfs-progs`) as well as an up-to-date [Rust toolchain](https://rustup.rs/) and the [`pgrx` subcommand for `cargo`](https://github.com/pgcentralfoundation/pgrx/blob/master/cargo-pgrx/README.md#cargo-pgrx).

1. **format a disk as BTRFS**

    The easiest thing to do here is plug in a USB and check which disk it is with `lsblk`. Once you've figure out which disk you'd like to reformat, you can do so with:

    ````sh
    sudo mkfs.btrfs /dev/sdX # replace sdX with your drive
    ````

2. **mount your `btrfs`-formatted disk**

    You need a directory to mount this disk to, first. Something like:

    ````sh
    sudo mkdir /mnt/database
    ````

    ...which you can then use as a mount point for your new `btrfs` drive with:

    ````sh
    sudo mount /dev/sdX /mnt/database
    ````

3. **intialize a Postgres cluster on your mounted disk**

    [`cargo pgrx` can take care of initialization](https://github.com/pgcentralfoundation/pgrx/blob/master/cargo-pgrx/README.md#cargo-pgrx) as long as it knows where to initialize the data through the `PGRX_HOME` variable. Something like:

    ````sh
    PGRX_HOME=/mnt/database cargo pgrx init
    ````

4. **clone this repo**

    The rest of these steps will be done from within this repo, so make sure you've run `git clone git@github.com:NAlexPear/pg_branch.git` and `cd pg_branch`.

5. **convert all segment data directories to subvolumes**

    Before `pg_branch` can take over database creation, the subdirectories in the newly-initialized data directory of your database need to be converted to `btrfs` subvolumes. This repo provides an `init.sh` script for doing just this that, as long as it's provided a `PGDATA` variable that points to the data directory of your cluster.

    `pgrx` data directories have a structure of `$PGRX_HOME/data-$PG_VERSION`. So if you initialized your project as instructed in step 3, you should be able to run the `init.sh` script in this repository like so:

    ````sh
    PGDATA=/mnt/database/data-15 ./init.sh
    ````

    ...and you should have successfully converted all of the initial databases in your cluster to subvolumes.

6. **get into `psql`**

    The quickest way to jump into a `psql` session that recognizes `pg_branch` is to run the following:

    ````sh
    PGX_HOME=/your/mounted/btrfs/disk cargo pgrx run
    ````

7. **create the extension in `psql` with `CREATE EXTENSION pg_branch`**

8. **create some databases**

    After creating the extension, you can run `CREATE DATABASE <dbname> WITH TEMPLATE <template_dbname>` commands to quickly and atomically copy databases without requiring an exclusive lock or dedicated connection. To use the default `CREATE DATABASE` behavior again, pick an explicit `STRATEGY` other than `SNAPSHOT` (i.e. `WAL_COPY` or `FILE_COPY`).

## Future Work

1. distribute as pre-compiled extension
2. implement a cluster-wide `fork`
3. support more of the options supported by `CREATE DATABASE`
4. streamline setup of the data directory and its file system
5. support additional copy-on-write file systems like `ZFS` and `XFS`
6. include an example Dockerfile

## Similar Projects

This project's use of file system snapshots as a branching mechanism is heavily inspired by [`pgcow`](https://github.com/Photonios/pgcow) and [Postgres.ai](https://postgres.ai/). And credit for the concept of "forking" Postgres clusters goes to [Heroku's Database Fork](https://devcenter.heroku.com/articles/heroku-postgres-fork) feature.

