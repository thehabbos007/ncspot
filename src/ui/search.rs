#![allow(unused_imports)]

use cursive::direction::Orientation;
use cursive::event::{AnyCb, Event, EventResult, Key};
use cursive::traits::{Boxable, Finder, Identifiable, View};
use cursive::view::{IntoBoxedView, Selector, ViewWrapper};
use cursive::views::{EditView, NamedView, ViewRef};
use cursive::{Cursive, Printer, Vec2};
use std::cell::RefCell;
use std::sync::{Arc, Mutex, RwLock};

use crate::album::Album;
use crate::artist::Artist;
use crate::command::{Command, MoveMode};
use crate::commands::CommandResult;
use crate::episode::Episode;
use crate::events::EventManager;
use crate::library::Library;
use crate::playlist::Playlist;
use crate::queue::Queue;
use crate::show::Show;
use crate::spotify::{Spotify, URIType};
use crate::track::Track;
use crate::traits::{ListItem, ViewExt};
use crate::ui::layout::Layout;
use crate::ui::listview::{ListView, Pagination};
use crate::ui::search_results::SearchResultsView;
use crate::ui::tabview::TabView;
use rspotify::model::search::SearchResult;
use rspotify::senum::SearchType;

pub struct SearchView {
    edit: NamedView<EditView>,
    edit_focused: bool,
}

pub const EDIT_ID: &str = "search_edit";

impl SearchView {
    pub fn new(events: EventManager, queue: Arc<Queue>, library: Arc<Library>) -> SearchView {
        let searchfield = EditView::new()
            .on_submit(move |s, input| {
                if !input.is_empty() {
                    let results = SearchResultsView::new(
                        input.to_string(),
                        events.clone(),
                        queue.clone(),
                        library.clone(),
                    );
                    s.call_on_name("main", move |v: &mut Layout| v.push_view(Box::new(results)));
                }
            })
            .with_name(EDIT_ID);

        SearchView {
            edit: searchfield,
            edit_focused: true,
        }
    }

    pub fn clear(&mut self) {
        self.edit
            .call_on(&Selector::Name(EDIT_ID), |v: &mut EditView| {
                v.set_content("");
            });
    }
}

impl View for SearchView {
    fn draw(&self, printer: &Printer<'_, '_>) {
        let printer = &printer
            .offset((0, 0))
            .cropped((printer.size.x, 1))
            .focused(self.edit_focused);
        self.edit.draw(printer);
    }

    fn layout(&mut self, size: Vec2) {
        self.edit.layout(Vec2::new(size.x, 1));
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if event == Event::Key(Key::Esc) || event == Event::Key(Key::Tab) {
            self.edit_focused = !self.edit_focused;
            return EventResult::Consumed(None);
        }

        if self.edit_focused {
            self.edit.on_event(event)
        } else {
            EventResult::Ignored
        }
    }

    fn call_on_any<'a>(&mut self, selector: &Selector<'_>, callback: AnyCb<'a>) {
        self.edit.call_on_any(selector, &mut |v| callback(v));
    }

    fn focus_view(&mut self, selector: &Selector<'_>) -> Result<(), ()> {
        if let Selector::Name(s) = selector {
            self.edit_focused = s == &"search_edit";
            Ok(())
        } else {
            Err(())
        }
    }
}

impl ViewExt for SearchView {
    fn title(&self) -> String {
        "Search".to_string()
    }

    fn on_command(&mut self, _s: &mut Cursive, cmd: &Command) -> Result<CommandResult, String> {
        if let Command::Focus(_) = cmd {
            self.edit_focused = true;
            self.clear();
            return Ok(CommandResult::Consumed(None));
        }

        Ok(CommandResult::Ignored)
    }
}
