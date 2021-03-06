# Disclaimer
This repository is currently a pre-alpha version. It is aimed at my own personal research and not suitable for any purpose. There will be a mature alpha version that can be used by others. Until then the source code, the interfaces, the modules can change arbitrarily without notice.
# Introduction
ZuSearch (Zukunft Search) is a library and ecosystem for lightweight highly modular search for embedded systems to large scale clusters.
The core of ZuSearch is highly modular and users may decide which module to include depending on the system as well as usage context. 
Due to the modularity it can run  small embedding devices up to large scale clusters consisting of millions of nodes. Furthermore, thanks to the modularity you can
integrate any search, such as traditional one using inverted index or search indexes based on large scale neural models for text and/or images, for any type of document, such as text or images.
It is written in Rust for safe, multi-platform and low resource usage applications. The WebAssembly (WASM)/WASM System Interface (WASI) ecosystem provides the foundation to run them securely everywhere.

One important aspect of modularization is that it is easier to get rid of modules not needed anymore or to refactor the software. The objective here is that the software does not get bloated more and more, but with each release functionality can simply be removed. A hope is that in the future it is able to run on very small devices that are decades old and for which no more sustainable replacement is possible.

ZuSearch aims at being much different from other search libraries, such as [Lucene](https://lucene.apache.org/) or [Tantivy](https://github.com/tantivy-search/tantivy), by being highly modular, very lightweight for any type of device and enabling search beyond traditional indexing using logic reasoning or natural languages processing (NLP) models. 
# Use cases
## Search Encrypted Cloud Storage
Tools, such as [Cryptomator](https://cryptomator.org/) allow encrypting your files in the cloud and decrypting them locally on any device. However, searching those files locally 
takes a lot of time due to encryption and loading the files from the cloud. ZuSearch can be used to create an index of the files and store this index as well encrypted
in the cloud using said tools. Once the index is created, the files are searchable with high speed.
## Highly Scalable Search system
You can create a cluster of nodes running each of them ZuSearch locally with parts of the index. You can send a queries to each of the nodes and collect their answers.
A more advanced system "knows" also to which nodes to send the query and get immediate results. Additionally, relevancy ranking needs to be taken into account.
This will be part of ZuSearchNet.
## Search local in the browser
Many web applications nowadays only work if they are online. This is problematic in case of unreliable networks to continue work. ZuSearch can be packaged into a WASM application and thus provide search locally in documents processed by the web application.

# Components
## zusearch
Zusearch is the core of ZuSearch and it provides a lightweight very stable layer that should change rarely. It is a library that is offered on any operating system.
Zusearch is configuarable using configuration files that define "search apps" which are combinations of libzusearch-modules for a specific application. An arbitrary number of "search apps" are supported.
Different bindings will be offered based on a cbinding for Rust for languages such as Java, Python or Swift. Similar to ZuSearch itself, those bindings should be very lightweight enabling support for a multitude of devices up to large scale big compute clusters.
## zusearch-modules-common
This provides various common modules (libraries) for search that users may use or simply not use at all, because they develop their own modules. The idea of common
is 1) to provide some high quality search modules for common search use cases, especially test 2) to demonstrate the integration of various modules in ZuSearch and
3) to be a high quality testbed for ZuSearch
Examples for such modules are different pipeline steps, such as extracting from JSON, decompression or tokenization, different types of indexes, such as inverted text index, low latency NLP models or geospatial ones to specific search types, such as traditional text search, similarity for text, images and video or knowledge-graph based questioning and answers.
## zus-cli
zus-cli is a command line interface (CLI) that uses the library zusearch. It can be used for creating indexes, ingesting document into an index, searching an index locally and remotely (zus-daemon). zus-cli will have an interactive mode to avoid loading the indexes etc. for every new command from scratch. Nevertheless, for search apps needed to serve thousands of users or for remote searching, a cluster of zus-daemons is recommended.
## zus-daemon
zus-daemon is a lightweight background daemon that offers libzusearch via a network. It has various providers, such as REST via HTTP2 (with various authentication possibilities). Those providers are modules that can optionally be integrated. The providers have a conservative default configuration and must not be dependent on various zusearch-modules - they can be used with any zusearch-module now and in the future.
Additionally, zus-daemon can dynamically manage their load and share this with other zus-daemon instances. This is done via messaging mechanisms to distribute load as well as caches to keep track of the load at the global network level. 
zus-daemon does not provide a user interface. As a common practice the user interface should be separated from the backend. This is part of zus-web. 
## zus-web
As mentioned before, zus-daemon should not run also a web user interface, this is provided by zus-web and can be deployed on any other isolated node or container.

## Bindings
ZuSearch is written in Rust. However, also other programming languages should be able benefit from ZuSearch and use the library:
* C
* Java
* Python
* Swift
* Wasm+Wasi

Specific wrappers will be created to enable those and that can target different machine architectures. If different target architectures are offered than also the corresponding modules need to be available for this target architecture.

You cannot write modules in a different language than Rust. However, from  module Rust you can of course link any other language as you like.


## Ecosystem zusearch
It is expect that in the future a full ecosystem of modules will develop around ZuSearch and users can abritrarily combine them offering search possibilities not even envision here. The main goal of ZuSearch is being highly modular with a stable small core suitable for any device. You can add your own query engine, your own index type or own extraction modules.

The modules can be written in any language that is supported by WebAssembly (WASM). We encourage as a default language to use Rust to leverage its features of safeness and performance to meet the safety goals of Zusearch. If you need to call libraries written in other languages then we recommend to create at least a small wrapper in Rust that calls the library. 

# License
   Copyright 2021 J??rn Franke / ZuInnoTe <zuinnote@gmail.com>

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

