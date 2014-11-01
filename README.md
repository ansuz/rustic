# rustic

The rust programming language :: ansuz style.

## What can I use Rust for?

At this point, I'm not really sure. I started looking at it a while back, but there were some advertised features that hadn't quite been implemented yet. There were platforms I wanted to target that weren't quite supported. I lost interest, and just went further into Javascript.

It's been some time, since then, and I want to give it another try. Attempting to compile some of my old source threw a lot of errors, so I know its developers have been busy making changes. I started programming in C, so there's a big part of me that feels guilty for leaning so heavily on a (NON-LisP) interpreted language. I'm not gonna quit using Javascript, but I'd like to simultaneously return to my roots.

## What will this repository do?

It will contain a bunch of rust snippets, as I explore and attempt to identify what parts of the language I like best. If some particular style emerges, maybe I'll turn it into a support library for working in that style. We'll see.

## What do I need?

You'll need `rustc`, the rust compiler. Rust bootstraps (it's compiled using its own compiler), so you'll need to start with a binary, then you can rebuild, if you want.

You can get the latest binary by running:

```bash
curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

Then you'll need this repository:

```bash
git clone https://github.com/ansuz/rustic.git
cd rustic
mkdir bin
./call <one of the files in the src directory>
```
