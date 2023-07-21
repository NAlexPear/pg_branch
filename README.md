# `pg_branch`

![Pre-release Checks](https://github.com/NAlexPear/pg_branch/actions/workflows/check.yml/badge.svg?branch=master)

Quickly create "branches" of individual databases within a Postgres cluster using copy-on-write file systems like BTRFS.

## Table of Contents

1. [Introduction](#introduction)
2. [Similar Projects](#similar-projects)
2. [Getting Started](#getting-started)

## Introduction

Disclaimer: this is **pre-alpha software**, meant as an experiment and proof-of-concept. Use at your own risk!

## Similar Projects

This project's use of file system snapshots as a branching mechanism is heavily inspired by [`pgcow`](https://github.com/Photonios/pgcow) and [Postgres.ai](https://postgres.ai/). And credit for the concept of "forking" Postgres clusters goes to [Heroku's Database Fork](https://devcenter.heroku.com/articles/heroku-postgres-fork) feature.

## Getting Started

TODO: include more details for either dev or "production" use

1. format disk as BTRFS
2. mount somewhere to use as data directory (PGX_HOME for dev)
3. intialize a postgres cluster (TODO: wrap this script to avoid next step)
4. convert all segment data directories to subvolumes
5. for dev, run `PGX_HOME=/your/mounted/btrfs/disk cargo pgrx run` to get to `psql`
6. create the extension with `CREATE EXTENSION pg_branch`
7. run `CREATE DATABASE <dbname> WITH TEMPLATE <template_dbname>` commands to quickly and atomically copy databases without requiring an exclusive lock or dedicated connection.
