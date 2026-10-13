use zed_extension_api as zed;

struct NiteExtension { }

impl zed::Extension for NiteExtension {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(NiteExtension);