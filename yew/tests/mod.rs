mod common;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::functional::{FunctionComponent, FunctionProvider};
use yew::{html, Html, Properties};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn props_are_passed() {
    struct PropsPassedFunction {}
    #[derive(Properties, Clone, PartialEq)]
    struct PropsPassedFunctionProps {
        value: String,
    }
    impl FunctionProvider for PropsPassedFunction {
        type TProps = PropsPassedFunctionProps;

        fn run(props: &Self::TProps) -> Html {
            assert_eq!(&props.value, "props");
            return html! {
                <div id="result">
                    {"done"}
                </div>
            };
        }
    }
    type PropsComponent = FunctionComponent<PropsPassedFunction>;
    yew::start_app_with_props_in_element::<PropsComponent>(
        yew::utils::document().get_element_by_id("output").unwrap(),
        PropsPassedFunctionProps {
            value: "props".to_string(),
        },
    );
    let result = obtain_result();
    assert_eq!(result.as_str(), "done");
}
