![Build and Test Status](https://github.com/mattkleiny/surreal-rust/workflows/Build%20and%20Test/badge.svg)

# Surreal ![Surreal Icon](./surreal.ico)

A simple but capable game engine, built with Rust.

This project is in active development.

## Design goals

### A usable editor

Building games with code alone is fine for small projects and jams, but not
ideal for something longer-term.

One of the core goals of the project is to produce an editor in the same style
as Unity or Unreal to allow building games ergonomically.

More information on the editor can be found in [the editor crate](./editor)

### Simple and extendable rendering

Some example features:
* Easy to use graphics API bindings with support for custom shader programs and visual shader construction.
* SDF generation for advanced screen space effects (such as ray marching) with built-in techniques.
* Global illumination based on voxel projection planes

### Pluggable and flexible scripting

Writing games is hard, writing games in a highly rigid language like C/C++/Rust/etc is even harder.

Scripting allows for various DSLs to take the brunt of the work required in game development.

DSLs can be used for all sorts of scenarios:

* Dialogue
* Coordination
* Blueprints
* Procedural generation
* Behaviour trees
* State machines
* Triggers and Actuators
* etc

A goal of this project is to implement a shared virtual machine for executable scripts and offer a variety of different front-ends for different scripting languages; the goal is to allow the best tool for the job.

Metadata about scripts can also be provided for reflection, dynamic code generation, etc, allowing different DSLs for different use cases.
