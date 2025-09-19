# Bevy Houdini Loader

Loads assets exported from Houdini using the **Bevy Toolbox** plugin into [Bevy](https://bevyengine.org/).

## Instalation

Add this to your `Cargo.toml`:

````toml
[dependincies]
bevy_houdini_loader = "0.1.0"
````

## Example

```rust
use bevy::prelude::*;
use bevy_houdini_loader::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HoudiniImportSettings {
            json_path: "assets/output.json".into(),
        })
        .add_plugins(HoudiniImportPlugin)
        .run();
}
```
