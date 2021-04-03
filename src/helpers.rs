use handlebars::{
    handlebars_helper, to_json, Context, Handlebars, Helper, HelperResult, JsonRender, Output,
    RenderContext, RenderError,
};
use log::{debug, info, warn};

pub fn register(hbs: &mut Handlebars<'_>) {
    debug!("registering helpers...");

    // Register simple helpers
    handlebars_helper!(lowercase: |v: str| v.to_lowercase());
    hbs.register_helper("lowercase", Box::new(lowercase));

    handlebars_helper!(uppercase: |v: str| v.to_uppercase());
    hbs.register_helper("uppercase", Box::new(uppercase));

    // Register more complex helpers
    hbs.register_helper("default", Box::new(default));
}

// Use a default value if the provided variable is empty or blank
fn default(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).map(|v| v.value()).unwrap();

    if param.is_null() {
        let default = h.param(1).map(|v| v.relative_path().unwrap()).unwrap();
        // For some reason this value is single quoted, so we have to remove them.
        out.write(default.trim_matches('\''))?;
    } else {
        out.write(param.as_str().unwrap())?;
    }

    Ok(())
}

// #[cfg(test)]
// mod test {
//     // use crate::assert_renders;
//     // use crate::tests::assert_helpers;
//     use handlebars::{
//         handlebars_helper, to_json, Context, Handlebars, Helper, HelperResult, JsonRender, Output,
//         RenderContext, RenderError,
//     };
//     use std::error::Error;

//     #[test]
//     fn test_string_helpers() -> Result<(), Box<dyn Error>> {
//         let hbs = Handlebars::new();
//         assert_that!(hbs.render_template()?)
//             .named("lowercase")
//             .is_equal_to("Hello foo-bars");

//         // assert_helpers(
//         //     "Hello foo-bars",
//         //     vec![
//         //         ("lowercase", "hello foo-bars"),
//         //         ("uppercase", "HELLO FOO-BARS"),
//         //     ],
//         // )?;

//         Ok(())
//     }
// }
