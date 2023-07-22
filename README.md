![AI-generated logo that confers more legitimacy than this project deserves](assets/logo.svg)

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

`pg_branch` is an Postgres extension that solves those problems by giving `CREATE DATABASE` the power of snapshots. If your `PGDATA` directory is on a copy-on-write file system like [`BTRFS`](https://wiki.archlinux.org/title/btrfs), the `pg_branch` extension turns every `CREATE DATABASE` into an atomic file system snapshot that takes seconds instead of minutes (or hours). In addition, the copy-on-write strategy keeps disk usage low by only writing new segment data files to disk when they're modified (rather than read).

## Getting Started

1. format a disk as BTRFS
2. mount that distk somewhere to use as data directory (PGX_HOME for dev)
3. intialize a postgres cluster with `cargo pgrx` or `initdb`
4. convert all segment data directories in `$DATA_DIRECTORY/base` to subvolumes
5. for dev, run `PGX_HOME=/your/mounted/btrfs/disk cargo pgrx run` to get to `psql`
6. create the extension with `CREATE EXTENSION pg_branch`
7. run `CREATE DATABASE <dbname> WITH TEMPLATE <template_dbname>` commands to quickly and atomically copy databases without requiring an exclusive lock or dedicated connection.

## Future Work

1. implement a cluster-wide `fork`
2. support more of the options supported by `CREATE DATABASE`
3. streamline setup of the data directory and its file system
4. support additional copy-on-write file systems like `ZFS` and `XFS`
5. include an example Dockerfile

## Similar Projects

This project's use of file system snapshots as a branching mechanism is heavily inspired by [`pgcow`](https://github.com/Photonios/pgcow) and [Postgres.ai](https://postgres.ai/). And credit for the concept of "forking" Postgres clusters goes to [Heroku's Database Fork](https://devcenter.heroku.com/articles/heroku-postgres-fork) feature.

