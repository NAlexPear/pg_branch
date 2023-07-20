# `pg_branch`

![Pre-release Checks](https://github.com/NAlexPear/pg_branch/actions/workflows/check.yml/badge.svg?branch=master)

Quickly create "forks" of your Postgres cluster or "branches" of individual databases within a cluster using copy-on-write file systems like BTRFS.

## Table of Contents

1. [Introduction](#introduction)
2. [Similar Projects](#similar-projects)
2. [Getting Started](#getting-started)

## Introduction

Disclaimer: this is **pre-alpha software**, meant as an experiment and proof-of-concept. Use at your own risk!

## Similar Projects

This project's use of file system snapshots as a branching mechanism is heavily inspired by [`pgcow`](https://github.com/Photonios/pgcow) and [Postgres.ai](https://postgres.ai/). And credit for the concept of "forking" Postgres clusters goes to [Heroku's Database Fork](https://devcenter.heroku.com/articles/heroku-postgres-fork) feature.
