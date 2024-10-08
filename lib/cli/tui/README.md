<p align="center"><img src="../../../.github/img/logo-cli-tui.svg" width="142"></p>

<br>
<br>
<br>

<p align="center"><b>lool » <code>cli.stylize</code></b> is a set of utilities for colorizing console outputs.
</p>

<br>
<br>
<br>

# Installation

This crate is for internal use. It's only published privately. 

```bash
cargo add lool --registry=lugit --features cli cli.tui
```

# `lool::cli::tui` Framework

This module provides a small framework for building async terminal user interfaces (TUIs) using a
component-based architecture, using the `ratatui` library.

This module defines two primary elements: 
- the `App` struct, 
- and the `Component` trait.
 
Together, these elements facilitate the creation of modular and interactive terminal applications.

## Framework

### `App` Struct

The `App` struct represents the main application and is responsible of (among other things):

- **Tick Rate and Frame Rate**: Controls the update frequency of the application.
- **Component Management**: Manages a collection of components that make up the user interface.
- **Event Handling**: Processes user inputs and dispatches actions to the appropriate components.
- **Lifecycle Management**: Handles the start, suspension, and termination of the application.

### `Component` Trait

The `Component` trait represents a visual and interactive element of the user interface.

Components can be nested, allowing for a hierarchical structure where each component can have child
components. This trait provides several methods for handling events, updating state, and rendering:

- **Event Handling**: Methods like `handle_frame_event` and `handle_paste_event` allow components
  to respond to different types of events.
- **State Management**: Methods like `update` and `receive_message` enable components to update
  their state based on actions or messages.
- **Initialization**: The `init` method allows components to perform setup tasks when they are first
  created.
- **Rendering**: The `draw` method is responsible for rendering the component within a specified
  area. All components must implement this method to display their content.

## How It Works

### Component-Based Architecture

The framework uses a component-based architecture, where the user interface is composed of multiple
components. Each component can have child components, forming a tree-like structure. This design
promotes modularity and reusability, making it easier to manage complex user interfaces in a 
structured and standardized way.

### Interaction Between `App` and `Component`

- **Initialization**: The `App` initializes all components and sets up the necessary event channels.
- **Event Dispatching**: The `App` listens for user inputs and dispatches actions to the relevant
  components.
- **State Updates**: Components update their state based on the actions they receive and can
  propagate these updates to their child components.
- **Rendering**: Components handle their own rendering logic, allowing for a flexible and
  customizable user interface.


Usually, the `App` is provided with a root component that represents the main component of the
application.

From the Main/Root component, the application can be built by nesting child components as needed in
a tree-like structure. Example:

```txt
App
└── RootComponent
    └── Router
        ├── Home
        │    ├── Header
        │    └── Content
        ├── About
        │    ├── Header
        │    └── Content
        └── Contact
             ├── Header
             └── ContactForm
```

In this example, the `RootComponent` is the main component of the application and contains a
`Router`, which is another component that manages the routing logic. The `Router` component has
three child components: `Home`, `About`, and `Contact` and will render the appropriate component
depending on the current route.

Then, heach "route" component (`Home`, `About`, `Contact`) can have its own child components, such
as `Header`, `Content`, and `ContactForm` and use them to build the final user interface.

The `RootComponent` will call the `draw` method of the `Router` component, which will in turn call
the `draw` method of the current route component (`Home`, `About`, or `Contact`), and so on.

The `draw` chain will propagate down the component tree, allowing each component to render its
content. The `App` starts the draw chain a few times per second. The amount of draw calls per second
is controlled by the `frame_rate` of the `App`:

```rust
let mut app = App::new(...).frame_rate(24); // 24 frames per second
```

Some tasks might be too expensive to be performed on every frame. In these cases, the `App` alsp
defines a `tick_rate` that controls how often the `handle_tick_event` method of the components is
called.

The tick event is often used to update the state of the components, while the frame event is used to
render the components in the terminal.

For example, a tick rate of 1 means that the `handle_tick_event` method of the components will be
called once per second. And a component might use this event to update its state, run background
tasks, or perform other operations that don't need to be done on every frame.

```rust
let mut app = App::new(...).tick_rate(10); // 10 ticks per second
```

### Component Communication

Components can communicate with each other using messages. The `Component` trait defines the 
following methods:

- `register_action_handler`: registers a mpsc sender, to send messages to the bus.
- `receive_message`: receives a message from the bus.

At the start of the application, the `App` will call the  `register_action_handler` method of each
component, and they can store the sender to send messages to the bus.

When a component wants to send a message to another component, it can use the sender it received
during the registration process.

```rust
self.bus.send("an:action".to_string());
```

For simplicity, the messages are just strings, but one can serialize more complex data structures
into strings if needed in any format like JSON, TOML or even a custom format that just suits the
our needs:

```rust
self.bus.send(format!("task:{}:state='{}',date='{}'", task_id, state, date));
```

Then we can receive the message in the `receive_message` method of another component:

```rust
fn receive_message(&mut self, message: String) {
    if message.starts_with("task:") {
        // Parse the message and do whatever is needed with the data
    }
}
```

## Widgets

Apart from the tui framework, this module also provides a set of reusable "ratatui-native" widgets
that can be used. Theese are not components, but Widgets, just like native `Paragraph`, `Block`,
etc.

Right now, the following widgets are available:

### `TextArea`

This is a rip-off of the `TextArea` widget from the
[`tui-rs`](https://github.com/rhysd/tui-textarea) crate, but with less capabilities. In summary,
search, mouse support, copy, paste, and undo/redo functionalities were stripped off.

This implementation also changes the default key bindings to be more similar to the ones used in
the `coco` package (conventional commit cli utility). That is:

- <kbd>Enter</kbd> won't add a new line. **why?** Because that way, we can use the <kbd>Enter</kbd>
  key to submit the "form" or cofirm the input.
- Removes all key bindings of stripped functionalities.

#### Vakudation

IN this implementation, the `TextArea` widget also supports validation. The validation is done by
accepting any number of validation functions that will be called every time the validity of the
text-area is checked.

One can add as many validation functions as needed:

```rust
textarea..with_validations(vec![
    |input: &str| {
        if input.len() > 10 {
            Err(format!("Input must be less than 10 characters"))
        } else {
            Ok(())
        }
    },
    required_validator,
]);
```

## `GridSelector` Widget

A selector restful widget that can be used to select items from a list. The items are displayed in a
grid, and the user can navigate through them using, for example, the arrow keys.

### Example

See the [`widget_grid_selector.rs`](/examples/widget_grid_selector.rs) example for a full
demonstration of how to use the `GridSelector` widget.

In summary, the widget uses a `GridSelectorState` to keep track of the items and the state of the
widget.

The `GridSelectorState` takes a list of `GridItem`, which is a basic struct that encapsulates a 
`String` value.

The `GridSelectorState::new` method accepts a list of any type that can be converted to a
`GridItem`:

```rust
pub fn new<I, T>(items: I) -> Self
  where
    I: IntoIterator<Item = T>,
    T: Into<GridItem>, // Accept anything that can be converted into GridItem
  {
    ...
  }
}
```

As a part of this library, the `Into<GridItem>` trait is implemented for `String`, `&str`.

The example at [`widget_grid_selector.rs`](/examples/widget_grid_selector.rs) demonstrates how to
implement the `Into<GridItem>` trait for a custom type.

## `Switch` Widget

A simple stateless switch widget that can be used to show visual feedback of a boolean state.

### Example

See the [`widget_switch.rs`](/examples/widget_switch.rs) example for a full demonstration of how to
use the `Switch` widget.