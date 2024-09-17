use {
    super::{
        component::Component,
        events::{Action, Event},
        keyboard::KeyBindings,
        tui::Tui,
    },
    crossterm::event::{KeyCode, KeyEvent},
    eyre::Result,
    ratatui::prelude::Rect,
    std::{collections::HashMap, str::FromStr},
    tokio::sync::mpsc::{self, error::TryRecvError},
};

pub type Kb<'a> = HashMap<&'a str, String>;

pub struct App {
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub keybindings: KeyBindings,
    pub last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<String>,
    action_rx: mpsc::UnboundedReceiver<String>,
}

impl App {
    pub fn new(kb: Kb, components: Vec<Box<dyn Component>>) -> Result<Self> {
        let keybindings = KeyBindings::new(kb);
        let (action_tx, action_rx) = mpsc::unbounded_channel::<String>();

        Ok(Self {
            tick_rate: 1.into(),
            frame_rate: 4.into(),
            action_tx,
            action_rx,
            components,
            keybindings,
            should_quit: false,
            should_suspend: false,
            last_tick_key_events: Vec::new(),
        })
    }

    fn send(&self, action: Action) -> Result<()> {
        match action {
            Action::AppAction(cmd) => self.action_tx.send(cmd)?,
            Action::Key(key) => self.action_tx.send(key)?,
            action => self.action_tx.send(action.to_string())?,
        }

        Ok(())
    }

    fn try_recv(&mut self) -> Result<String, TryRecvError> {
        self.action_rx.try_recv()
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?.tick_rate(self.tick_rate).frame_rate(self.frame_rate);

        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    Event::Quit => self.send(Action::Quit)?,
                    Event::Tick => self.send(Action::Tick)?,
                    Event::Render => self.send(Action::Render)?,
                    // Event::Resize(x, y) => self.send(Action::Resize(x, y))?,
                    Event::Key(key) => {
                        if let Some(action) = self.keybindings.get(&[key]) {
                            self.send(action.clone())?;
                        } else {
                            // If the key was not handled as a single key action,
                            // then consider it for multi-key combinations.
                            self.last_tick_key_events.push(key);

                            // Check for multi-key combinations
                            if let Some(action) = self.keybindings.get(&self.last_tick_key_events) {
                                self.send(action.clone())?;
                            }
                        }

                        // send the key event as simple key event too (not as action) if it's a
                        // single alphanumeric char key
                        if let KeyCode::Char(c) = key.code {
                            if c.is_alphanumeric() {
                                self.send(Action::Key(c.to_string()))?;
                            }
                        }
                    }
                    _ => {}
                }
                let mut actions = Vec::new();

                for component in self.components.iter_mut() {
                    let component_actions = component.handle_events(Some(e.clone()))?;
                    actions.extend(component_actions);
                }

                for action in actions {
                    self.send(action)?;
                }
            }

            while let Ok(action) = self.try_recv() {
                let enum_action = Action::from_str(&action).ok();
                if let Some(a) = enum_action {
                    match a {
                        Action::Tick => {
                            self.last_tick_key_events.drain(..);
                        }
                        Action::Quit => self.should_quit = true,
                        Action::Suspend => self.should_suspend = true,
                        Action::Resume => self.should_suspend = false,
                        Action::Resize(w, h) => {
                            tui.resize(Rect::new(0, 0, w, h))?;
                            let mut errors = Vec::new();
                            tui.draw(|f| {
                                for component in self.components.iter_mut() {
                                    let r = component.draw(f, f.area());
                                    if let Err(e) = r {
                                        errors.push(format!("Failed to draw: {:?}", e));
                                    }
                                }
                            })?;
                            for error in errors {
                                self.send(Action::Error(error)).unwrap();
                            }
                        }
                        Action::Render => {
                            let mut errors = Vec::new();
                            tui.draw(|f| {
                                for component in self.components.iter_mut() {
                                    let r = component.draw(f, f.area());
                                    if let Err(e) = r {
                                        errors.push(format!("Failed to draw: {:?}", e));
                                    }
                                }
                            })?;
                            for error in errors {
                                self.send(Action::Error(error)).unwrap();
                            }
                        }
                        _ => {}
                    }

                    for component in self.components.iter_mut() {
                        component.update(a.clone())?;
                    }
                } else {
                    // unrecognized action, might be a custom component action
                    // send it to all components as a raw string
                    for component in self.components.iter_mut() {
                        let _ = component.receive_message(action.clone());
                    }
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                self.send(Action::Resume)?;
                tui = Tui::new()?.tick_rate(self.tick_rate).frame_rate(self.frame_rate);
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }
}
