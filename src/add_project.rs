use crate::Project;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{
    classes, events::Event, html, Callback, Component, Context, Html, Properties, TargetCast,
};

#[derive(Debug)]
pub enum Msg {
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateDescription(String),
    Add,
    Abort,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_add: Callback<Project>,
    pub on_abort: Callback<()>,
}

pub struct AddProjectForm {
    project: Project,
}
impl Component for AddProjectForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            project: Project::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let project = &mut self.project;
        match msg {
            Msg::UpdateFirstName(value) => {
                project.first_name = value;
                true
            }
            Msg::UpdateLastName(value) => {
                project.last_name = value;
                true
            }
            Msg::UpdateDescription(value) => {
                project.description = value;
                true
            }
            Msg::Add => {
                ctx.props().on_add.emit(std::mem::take(project));
                true
            }
            Msg::Abort => {
                ctx.props().on_abort.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let Self { project, .. } = self;

        let update_name = |f: fn(String) -> Msg| {
            link.callback(move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                f(input.value())
            })
        };

        let update_desc = link.callback(|e: Event| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::UpdateDescription(textarea.value())
        });

        html! {
            <>
                <div class="names">
                    <input
                        class={classes!("new-project", "firstname")}
                        placeholder="First name"
                        onchange={update_name(Msg::UpdateFirstName)}
                    />
                    <input
                        class={classes!("new-project", "lastname")}
                        placeholder="Last name"
                        onchange={update_name(Msg::UpdateLastName)}
                    />
                    <textarea
                        class={classes!("new-project", "description")}
                        placeholder="Description"
                        onchange={update_desc}
                    />
                </div>

                <button
                    disabled={project.first_name.is_empty() || project.last_name.is_empty()}
                    onclick={link.callback(|_| Msg::Add)}
                >
                    { "Add New" }
                </button>
                <button onclick={link.callback(|_| Msg::Abort)}>
                    { "Go Back" }
                </button>
            </>
        }
    }
}
