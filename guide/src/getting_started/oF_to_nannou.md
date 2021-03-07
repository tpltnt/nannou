# From openFrameworks to Nannou

**Tutorial Info**

- Author: tpltnt
- Required Knowledge:
    - [Getting Started](/getting_started.md)
- Reading Time: XX minutes

---


**[openFrameworks](https://openframeworks.cc)** is a framework for creative coding in **C++**.
It is older than nannou and therefore has a lot of code written for it.
Both frameworks aim to solve similar problems and have similar designs.
This guide is intended to help users of openFrameworks become familiar
with nannou.


Here is an example of a bare-bones openFrameworkd app that opens an empty window:
```cpp,no_run
#include "ofApp.h"

void ofApp::setup(){}

void ofApp::update(){}

void ofApp::draw(){
    ofBackground(ofColor::white);
}

```

Here's an example of a bare-bones nannou app that opens an empty window:

```rust,no_run
use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
}
```

Note that the code is quite similar. Here is a table of
corresponding parts:


|openFrameworks |nannou                                              |note                                          |
|---------------|----------------------------------------------------|----------------------------------------------|
|ofApp::setup() |model(_app: &App) -> Model                          |set up the basics and initial state of the app|
|ofApp::update()|event(_app: &App, _model: &mut Model, _event: Event)|handle events to change the state of the app  |
|ofApp::draw()  |view(_app: &App, _model: &Model, _frame: Frame)     |draw the current state for display            |

