use std::{fmt::Debug, path::PathBuf};

use axum::{Router, routing::get, serve::Listener};
use clap::{Args, Parser};
use maud::{html, Markup, Render, DOCTYPE};
use tokio::net::{TcpListener, UnixListener};
use tower_http::services::ServeDir;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    listen: Listen
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Listen {
    #[arg(short, group = "listen")]
    port: Option<u16>,

    #[arg(short, group = "listen")]
    uds: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(port) = cli.listen.port {
        serve_with_listener(
            TcpListener::bind(format!("0.0.0.0:{port}"))
                .await
                .unwrap(),
        )
        .await;
    } else if let Some(path) = cli.listen.uds {
        let _ = tokio::fs::remove_file(&path).await;
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();

        serve_with_listener(UnixListener::bind(path.clone()).unwrap()).await;
    }
}

async fn serve_with_listener<L>(listener: L)
where
    L: Listener,
    L::Addr: Debug,
{
    let app = Router::new()
        .route("/", get(root))
        .fallback_service(ServeDir::new("static"));

    axum::serve(listener, app).await.unwrap();
}

pub fn get_meta_tags() -> Markup {
    let title = "JT Raber";
    let description = "See what I have made!";

    html! {
        title { (title) }
        meta name="description" content=(description);

        meta property="og:url" content="https://jtraber.com/";
        meta property="og:type" content="website";
        meta property="og:title" content=(title);
        meta property="og:description" content=(description);

        meta name="twitter:card" content="summary_large_image";
        meta property="twitter:domain" content="jtraber.com";
        meta property="twitter:url" content="https://jtraber.com/";
        meta name="twitter:title" content=(title);
        meta name="twitter:description" content=(description);

        meta charset="UTF-8";
    }
}

async fn root() -> Markup {
    let shelves = vec![
        Shelf {
            name: "Personal Projects",
            projects: vec![
                Project {
                    name: "Whiteboard",
                    description: "A simple collaborative drawing web app utilizing Socket.IO. This app works on mobile and desktop, which provided me with some challenges implementing gesture recognition.",
                },
                Project {
                    name: "Lithobox/Lithophy",
                    description: "A customizable web interface for controlling an RGB lithophane cube. I plan on selling these, so the infrastructure had to be sound. I also made a Blender add-on that turns an image into a ready-to-print lithophane. This was created out of convenience for Lithobox.",
                },
                Project {
                    name: "Flightpath",
                    description: "Complex spaces can be difficult to naviate, and Flightpath can help. Originally a hackathon project meant to help users navigate an airport, it has grown to encompass more spaces. Ease-of-use and accessibility were of utmost importance when designing this project.",
                },
                Project {
                    name: "Blackboard Notify",
                    description: "During my time as a virtual tutor there were many long stretches of time when no students would show up. To free myself some time I wrote a Firefox extension that flashes my internet-controlled lamp when a student joins or messages the chat.",
                },
                Project {
                    name: "Internet Lamp",
                    description: "An ESP8266-based project that allows me (or anyone with internet access) to control a lamp in my room. This project was built from scratch, using Flask and Socket.IO. The lamp was made smart by cutting open it's wires and passing them through a relay controlled by the ESP8266.",
                },
                Project {
                    name: "baqpaq",
                    description: "A simple site to hold your commonly used code snippets. Share with others and gain knowledge. Kind of like stack overflow but less for learning and more for storing your snippets and finding others that might be useful.",
                },
                Project {
                    name: "Modette",
                    description: "The marker board outside of my room kept getting erased, so I decided to use my technomancy skills and create a motion detector using a Raspberry Pi. This project works by capturing an initial image of the area to be watched, and recording a video if too many pixels are too different.",
                },
                Project {
                    name: "PiCast",
                    description: "This project was created to test out WebRTC for myself. It is a simple screen-sharing web app that lets a user share their screen to a room or join a room and watch someone else's screen. I use this to cast my desktop to my TV using a Raspberry Pi.",
                },
                Project {
                    name: "SeRoku",
                    description: "A customizable remote server for a Roku TV. This uses Roku's ECP API with an ESP8266 web server. The user can customize 6 buttons that launch apps on their TV. This project was definitely a fun one and I learned a lot about embedded web servers.",
                },
                Project {
                    name: "Invman",
                    description: "A minimal inventory management system for use with load cells connected to an ESP8266. This project is still in it's early statges, but I think it has a lot of potential for automation.",
                },
                Project {
                    name: "HMS",
                    description: "HMS stands for Home Media Server. I wrote this out of necessity because I listen to a lot of playlists from YouTube and don't have great internet access at my house. So I would take my laptop somewhere with better connection, download the playlists, and then run HMS on my laptop to server the files to my desktop on my home network.",
                },
                Project {
                    name: "pshbtn",
                    description: "This is a very simple productivity web app inspired by Simone Giertz's Every Day Calendar. There are 100 buttons on the screen. When you tap/click on one it will light up and make a satisfying 'click' noise. This project was very CSS heavy to make the most satisfying UX I could.",
                },
            ]
        },
        Shelf {
            name: "Freelance Experience",
            projects: vec![
                Project {
                    name: "Escape Room",
                    description: "An interface for a simulated CCTV in an escape room. This project utilized SSH technology in conjuction with Python to send commands to a remote server for the game.",
                },
                Project {
                    name: "Drowsy Driver",
                    description: "A system that uses a heart rate monitor and an IR camera to detect when a driver is falling asleep while driving. The project used facial recognition to detect when the driver's eyes were closed for a given amount of time and would alert them with a buzzer and flashing light.",
                },
                Project {
                    name: "Internet Radio",
                    description: "An internet radio/mp3 player using a Raspberry Pi. The audio source was controlled using a rotary encoder attached to the Pi, and the play/pause functionality was added using buttons.",
                },
                Project {
                    name: "Test Grader",
                    description: "An automatic test grader that scans a test sheet like a scantron and tells the user their score using openCV. This job was rather challenging due to my then lack of experience using computer vision.",
                },
            ]
        }
    ];

    let sites = vec![
        Site {
            url: "https://www.youtube.com/JMANN240",
            color: "#c4302b",
            icon: "fa-youtube"
        },
        Site {
            url: "https://github.com/JMANN240",
            color: "#4078c0",
            icon: "fa-github"
        },
        Site {
            url: "https://stackoverflow.com/users/14108612/jt-raber",
            color: "#f48024",
            icon: "fa-stack-overflow"
        },
        Site {
            url: "https://www.linkedin.com/in/johnathon-raber-926b63209/",
            color: "#2867b2",
            icon: "fa-linkedin"
        },
    ];

    html! {
        (DOCTYPE)
        html {
            head {
                (get_meta_tags())
                link rel="stylesheet" href="/styles.css";
                script defer src="https://kit.fontawesome.com/4f61f8988e.js" crossorigin="anonymous" {}
            }
            body .flex-column {
                header .flex-row style="font-size: 4rem; justify-content: flex-start;" {
                    span class="neumorphic outset" style="padding: 0.5vmax 1vmax;" { "JT Raber" }
                    div class="flex-column" style="align-items: flex-start; justify-content: space-around;" {
                        span .neumorphic .inset style="font-size: 0.33em; font-weight: 100; padding: 0.75vmax; width: 40vw;" {
                            "💻 Level 100 Technomancer (B.Sc. Computer Science)"
                        }
                        span .neumorphic .inset style="font-size: 0.33em; font-weight: 100; padding: 0.75vmax; width: 40vw;" {
                            "🧮 Level 100 Mathemagician (B.Sc. Applied Mathematics)"
                        }
                    }
                }
                main {
                    @for shelf in shelves {
                        (shelf)
                    }
                }
                footer .neumorphic .inset .flex-row style="font-size: 2rem;" {
                    @for site in sites {
                        (site)
                    }
                }
            }
        }
    }
}

struct Shelf {
    name: &'static str,
    projects: Vec<Project>,
}

impl Render for Shelf {
    fn render(&self) -> Markup {
        html! {
            div class="neumorphic outset flex-column" style="font-size: 2rem; width: 95vw;" {
                span class="neumorphic inset" style="font-weight: 100;" { (self.name) }
                div class="flex-row" style="align-items: stretch; flex-wrap: wrap;" {
                    @for project in self.projects.iter() {
                        (project)
                    }
                }
            }
        }
    }
}

struct Project {
    name: &'static str,
    description: &'static str,
}

impl Render for Project {
    fn render(&self) -> Markup {
        html! {
            div class="neumorphic inset flex-column" {
                h1 { (self.name) }
                p style="width: 30ch;" {
                    (self.description)
                }
            }
        }
    }
}

struct Site {
    url: &'static str,
    color: &'static str,
    icon: &'static str,
}

impl Render for Site {
    fn render(&self) -> Markup {
        html! {
            a href=(self.url) target="_blank" class="clickable social neumorphic outset flex-row" style={"color: " (self.color)} {
                span class={"fab " (self.icon)} style="font-size: 1.5rem;" {}
            }
        }
    }
}
