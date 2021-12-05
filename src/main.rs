use add_project::AddProjectForm;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::{html, Component, Context, Html};

mod add_project;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Project {
    pub no: String,
    pub ptitle: String,
    pub pma: String,
    pub pcondition: String,
    pub pstart: String,
    pub plastupdate: String,
    pub preport: String,
    pub pnextreport: String,
    pub pnotes: String,
    pub first_name: String,
    pub last_name: String,
    pub description: String,
}

impl Project {
    pub fn render(&self) -> Html {
        html! {
            <div class="project" style="margin-bottom: 50px">
                <p>{ format!("First Name: {}", self.first_name) }</p>
                <p>{ format!("Last Name: {}", self.last_name) }</p>
                <p>{ "Description:" }</p>
                { &self.description }
            </div>
        }
    }
}

/// storage key for the projects
const KEY: &str = "yew.crm.Projects";

#[derive(Debug)]
pub enum Scene {
    ProjectsList,
    NewProjectForm,
    Settings,
}

#[derive(Debug)]
pub enum Msg {
    SwitchTo(Scene),
    AddProject(Project),
    ClearProjects,
}

pub struct Model {
    projects: Vec<Project>,
    scene: Scene,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let projects = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        Self {
            projects,
            scene: Scene::ProjectsList,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchTo(scene) => {
                self.scene = scene;
                true
            }
            Msg::AddProject(project) => {
                self.projects.push(project);
                LocalStorage::set(KEY, &self.projects).expect("failed to set");
                // we only need to re-render if we're currently displaying the projects
                matches!(self.scene, Scene::ProjectsList)
            }
            Msg::ClearProjects => {
                if gloo::dialogs::confirm("Do you really want to clear the data?") {
                    self.projects.clear();
                    LocalStorage::delete(KEY);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.scene {
            Scene::ProjectsList => html! {
                <div class="pms">
                    <table style="border: 0px; font-family: Roboto, sans-serif;" width="100%">
                           <tr style="font-weight: bold; background-color: #1c87c9; color: #ffffff; text-align: center;">
                               <td>{"#"}</td>
                               <td>{"Projekttitel"}</td>
                               <td>{"Mitarbeiter"}</td>
                               <td>{"Zustand"}</td>
                               <td>{"Start"}</td>
                               <td>{"Letztes Update"}</td>
                               <td>{"Letzter Bericht"}</td>
                               <td>{"Bericht nächste Sitzung"}</td>
                               <td>{"Bemerkungen"}</td>
                               <td>{"Edit"}</td>
                           </tr>
                        { for self.projects.iter().map(Project::render) }
                    </table><br/><br />
                    <button onclick={ctx.link().callback(|_| Msg::SwitchTo(Scene::NewProjectForm))}>{ "Projekt Hinzufügen" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::SwitchTo(Scene::Settings))}>{ "Settings" }</button>
                </div>
            },
            Scene::NewProjectForm => html! {
                <div class="crm">
                    <h1>{"Add a new project"}</h1>
                    <AddProjectForm on_add={ctx.link().callback(Msg::AddProject)} on_abort={ctx.link().callback(|_| Msg::SwitchTo(Scene::ProjectsList))} />
                </div>
            },
            Scene::Settings => html! {
                <div>
                    <h1>{"Settings"}</h1>
                    <button onclick={ctx.link().callback(|_| Msg::ClearProjects)}>{ "Remove all projects" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::SwitchTo(Scene::ProjectsList))}>{ "Go Back" }</button>
                </div>
            },
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
